use std::io;
use std::process;
 
fn main(){ 
    loop {
        println!("first number: ");
        let a = read_user_input();
        println!("second number: ");
        let b = read_user_input();

        let result = sum(a, b);
        println!("{} + {} =  {}", a, b, result);
            
    }
}

fn sum(a: u32, b:u32) -> u32 {
    a+b
}

fn read_user_input() -> u32 {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let digit:u32;

    match input.trim().parse() {
        Ok(val) => {
            digit = val;
        }, 
        Err(_err) => { // 에러는 _err 와 같이 언더스코어 써주지 않으면 warning
            println!("This is not a valid number");
            process::exit(1);
        }
    }
    digit
}