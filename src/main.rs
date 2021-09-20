use tax_1040::user_input;
use std::fmt;
use thousands::Separable;

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(u8)]
enum FilingStatus {
    Single,
    MarriedFilingSeparately,
    HeadOfHousehold,
    Married,
}

impl FilingStatus {
    fn standard_deduction(&self) -> i32 {
        match *self {
            FilingStatus::Single => 12550,
            FilingStatus::MarriedFilingSeparately => 12550,
            FilingStatus::HeadOfHousehold => 18800,
            FilingStatus::Married => 25100,
        }
    }
    
    fn qbi_threshold(&self) -> i32 {
        match *self {
            FilingStatus::Single => 163300,
            FilingStatus::Married => 326600,
            _ => 163300,
        }
    }
    
    fn tax_brackets(&self) -> Vec<TaxBracket> {
        match &self {
            _ => (
                vec!(
                    TaxBracket {
                        start: 0,
                        amount: 0,
                        rate: 0.10
                    },
                    TaxBracket {
                        start: 9950,
                        amount: 995,
                        rate: 0.12
                    },
                    TaxBracket {
                        start: 40525,
                        amount: 4664,
                        rate: 0.22
                    },
                    TaxBracket {
                        start: 86375,
                        amount: 14751,
                        rate: 0.24
                    },
                    TaxBracket {
                        start: 164925,
                        amount: 33603,
                        rate: 0.32
                    },
                    TaxBracket {
                        start: 209425,
                        amount: 47843,
                        rate: 0.35
                    },
                    TaxBracket {
                        start: 523600,
                        amount: 157804,
                        rate: 0.37
                    },
                )
            )
        }
    }
}

impl fmt::Display for FilingStatus {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           FilingStatus::Single => write!(f, "Single"),
           FilingStatus::MarriedFilingSeparately => write!(f, "Married Filing Separatey"),
           FilingStatus::HeadOfHousehold => write!(f, "Head of Household"),
           FilingStatus::Married => write!(f, "Married"),
       }
    }
}

struct TaxBracket {
    start: i32,
    amount: i32,
    rate: f32,
}

struct SelfEmploymentIncome {
    agi: i32,
    // income: i32,
    // deduction: i32,
}

fn get_filing_status() -> FilingStatus {
    let enum_options: Vec<FilingStatus> = vec![
        FilingStatus::Single,
        FilingStatus::MarriedFilingSeparately,
        FilingStatus::HeadOfHousehold,
        FilingStatus::Married,
    ];
    let enum_str_options: Vec<String> = enum_options.iter().map(|a| format!("{}", a)).collect();
    let option_num: i32 = user_input::get_option("Filing status?", &enum_str_options);
    let result = enum_options.iter().nth(option_num as usize).unwrap();
    let filing_status: FilingStatus = result.clone();
    
    filing_status
}

fn self_employment_tax_deduction() -> SelfEmploymentIncome {
    let ln_1a: i32 = user_input::get_int("Enter your expected income and profits subject to self-employment tax");
    // let ln_1b: i32 = get_input_int("If you will have farm income and also receive social security retirement or disability benefits, enter your expected Conservation Reserve Program payments that will be included on Schedule F (Form 1040) or listed on Schedule K-1 (Form 1065)");
    let ln_1b: i32 = 0;
    let ln_2: i32 = ln_1a - ln_1b;
    let ln_3: i32 = ((ln_2 as f64) * 0.9235) as i32;
    let ln_4: i32 = ((ln_3 as f64) * 0.029) as i32;
    // maximum amount subject to social security tax
    let ln_5: i32 = 142800;
    // let ln_6: i32 = get_input_int("Enter your expected wages (if subject to social security tax or the 6.2% portion of tier 1 railroad retirement tax)");
    let ln_6: i32 = 0;
    let mut ln_7: i32 = ln_5 - ln_6;
    
    if ln_7 < 0 {
        ln_7 = 0;
    }
    
    // Enter the smaller of line 3 or line 7
    let ln_8: i32 = if ln_3 < ln_7 { ln_3 } else { ln_7 };
    let mut ln_9: i32 = ((ln_8 as f64) * 0.124) as i32;
    
    if ln_7 <= 0 {
        ln_9 = 0;
    }
    
    let ln_10: i32 = ln_4 + ln_9;
    let ln_11 = ((ln_10 as f64) * 0.5) as i32;
    
    SelfEmploymentIncome {
        agi: ln_2 - ln_11,
        // income: ln_2,
        // deduction: ln_11,
    }
}

fn get_qbi_deduction(income: i32, deduction: i32, threshold: i32) -> i32 {
    if income >= threshold {
        // can't take the deduction
        return 0;
    }
    
    let taxable_income: i32 = income - deduction;
    let income_limitation: i32 = ((taxable_income as f64) * 0.2) as i32;
    
    income_limitation
}

fn figure_tax(taxable_income: i32, brackets: Vec<TaxBracket>) -> i32 {
    let mut brackets_iter = brackets.iter().rev();
    let bracket: &TaxBracket = brackets_iter.find(|a| taxable_income > a.start).unwrap();
    let bracket_tax: i32 = (((taxable_income - bracket.start) as f32) * bracket.rate) as i32;
    let tax: i32 = bracket.amount + bracket_tax;
    
    tax
}

fn do_taxes() -> () {
    // let is_dependent: bool = user_input::get_bool("Are you a dependent?", false);
    let filing_status: FilingStatus = get_filing_status();
    let self_employment_income = self_employment_tax_deduction();
    // Adjusted gross income you expect in 2021
    let ln_1: i32 = self_employment_income.agi;
    // Deductions
    let mut ln_2a: i32 = user_input::get_int("Deductions (use 0 to take the standard deduction)");
    let standard_deduction: i32 = filing_status.standard_deduction();
    
    if ln_2a < standard_deduction {
        println!("Taking standard deduction of ${}", standard_deduction.separate_with_commas());
        
        ln_2a = standard_deduction;
    }
    
    let qbi_deduction: i32 = ln_2a;
    let qbi_threshold: i32 = filing_status.qbi_threshold();
    let ln_2b: i32 = get_qbi_deduction(self_employment_income.agi, qbi_deduction, qbi_threshold);
    let ln_2c: i32 = ln_2a + ln_2b;
    // Subtract line 2c from line 1
    let ln_3: i32 = ln_1 - ln_2c;
    let brackets = filing_status.tax_brackets();
    
    if ln_3 <= 0 {
        println!("You do not owe any taxes :)");
        
        return ();
    }
    
    // Tax
    let ln_4 = figure_tax(ln_3, brackets);
    // Alternative minimum tax from Form 6251
    // just use 0
    let ln_5: i32 = 0;
    // Add lines 4 and 5. Add to this amount any other taxes you expect to include in the total on Form 1040, ine 16
    let ln_6: i32 = ln_4 + ln_5;
    // Credits
    let ln_7: i32 = 0;
    // let ln_7: i32 = user_input::get_int("Credits. Do not include any income tax withholding on this line");
    // Subtract line 7 from line 6.
    let mut ln_8: i32 = ln_6 - ln_7;
    
    if ln_8 < 0 {
        // If zero or less, enter -0-
        ln_8 = 0;
    }
    
    // Self-employment tax
    let ln_9: i32 = ((self_employment_income.agi as f64) * 0.153) as i32;
    // Other taxes
    // just use 0
    let ln_10: i32 = 0;
    // Add lines 8 through 10
    let ln_11a: i32 = ln_8 + ln_9 + ln_10;
    let ln_11b: i32 = 0;
    // let ln_11b: i32 = user_input::get_int("Earned income credit, additional child tax credit, fuel tax credit, net premium tax credit, refundable American opportunity credit, and refundable credit from Form 8885*)");
    // Total 2021 estimated tax. Subtract line 11b from line 11a. If zero or less, enter -0-
    let mut ln_11c: i32 = ln_11a - ln_11b;
    
    if ln_11c < 0 {
        ln_11c = 0;
    }
    
    // Multiply line 11c by 90% (662/3% for farmers and fishermen
    // We'll be using 100% instead of 90%
    let ln_12a: i32 = ((ln_11c as f64) * 1.0) as i32;
    // Required annual payment based on prior year’s tax (see instructions
    // unused
    // let ln_12b: i32 = 0;
    // Required annual payment to avoid a penalty. Enter the smaller of line 12a or 12b
    let ln_12c: i32 = ln_12a;
    let ln_13: i32 = 0;
    // let ln_13: i32 = user_input::get_int("Income tax withheld and estimated to be withheld during 2021 (including income tax withholding on pensions, annuities, certain deferred income, etc.");
    // Subtract line 13 from line 12c
    let ln_14a: i32 = ln_12c - ln_13;
    
    if ln_14a <= 0 {
        // Stop here. You are not required to make estimated tax payments.
        println!("You are not required to pay estimated taxes :)");
        
        return ();
    }
    
    // Subtract line 13 from line 11c
    let ln_14b: i32 = ln_11c - ln_13;
    
    if ln_14b < 1000 {
        // Stop here. You are not required to make estimated tax payments.
        println!("You are not required to pay estimated taxes :)");
        
        return ();
    }
    
    // If the first payment you are required to make is due April 15, 2021,
    // enter 1⁄4 of line 14a (minus any 2020 overpayment that you are applying to this
    // installment) here, and on your estimated tax payment voucher(s)
    // if you are paying by check or money order
    // unused
    // let ln_15: i32 = ((ln_14a as f64) * 0.25) as i32;
    
    println!("\nYou should expect to owe ${} in federal taxes", ln_14a.separate_with_commas());
    
    ()
}

fn main() {
    do_taxes();
}
