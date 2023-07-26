fn main() {
    let string1: &str = "Rust Bootcamp";
    let string2: &str = " Solution of HW #1";
    let concatenated_string = concatenate_strings(string1, string2);

    println!("The value of concatenated_string is {}",concatenated_string);    
}

fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut result: String = String::from(string1); 
    result.push_str(string2);
    result
}
