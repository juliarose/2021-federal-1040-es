use std::io;

pub fn get_option(prompt: &str, options: &Vec<String>) -> i32 {
    println!("{}", prompt);
    
    let count: i32 = options.clone().iter().count() as i32;
    let mut i: i32 = 1;
    
    for option in options {
        println!("{}: {}", i, option);
        
        i = i + 1;
    }
    
    let input: String = get();
    let parsed = input.parse();
    
    if parsed.is_ok() {
        let as_int: i32 = parsed.unwrap();
        
        if as_int <= 0 {
            println!("Try again - number must be greater than 0");
            
            return get_option(prompt, &options);
        } else if as_int > count {
            println!("Try again - number must be less than {}", count);
            
            return get_option(prompt, &options);
        } else {
            return as_int - 1;
        }
    } else {
        return get_option("Try again - not a number", &options);
    }
}

pub fn get_int(prompt: &str) -> i32 {
    println!("{}", prompt);
    
    let input: String = get();
    let parsed = input.parse();
    
    if parsed.is_ok() {
        let as_int: i32 = parsed.unwrap();
        
        return as_int;
    } else {
        return get_int("Try again - not a number");
    }
}

pub fn get_bool(prompt: &str, default_value: bool) -> bool {
    let default_value_str: &str = if default_value == true { "[Y/n]" } else { "[N/y]" };
    let formatted_prompt_string = format!("{} {}", prompt, default_value_str);
    let formatted_prompt: &str = &formatted_prompt_string;
    
    println!("{}", formatted_prompt);
        
    let input: String = get();
    let lcase_input: String = input.to_lowercase();
    let lcase_input_str: &str = &lcase_input;
    
    if lcase_input_str == "" {
        return default_value;
    }
    
    let yes: bool = match lcase_input_str {
        "" => true,
        "y" => true,
        "yes" => true,
        _ => false
    };
    
    return yes;
}

pub fn get_string(prompt: &str) -> String {
    println!("{}", prompt);
    
    let input: String = get();
    
    input
}

fn get() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}