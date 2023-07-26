use std::f32::consts::PI;

fn main() {
    println!("Hello, world!");
    let hel:String = hello("Sami");
    println!("{}",hel);
println!("{}",make_double(5));
println!("{}",multiply_pi(5.0));

}

fn multiply_pi(num:f32)->f32{
    num*PI
}

fn make_double(num:i32)->i32{
    num*2
}

fn hello(name: &str) -> String{
    format!("Hello {name}")   
}