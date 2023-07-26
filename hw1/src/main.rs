fn main() {
    let string1: &str = "Rust Bootcamp";
    let string2: &str = " Birinci Ödev Çözümü";
    let concatenated_string = concatenate_strings(string1, string2);

    println!("The value of concatenated_string is {}",concatenated_string);    
}

fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut result: String = String::from("");

    result.push_str(string1);
    result.push_str(string2);
    result
}
