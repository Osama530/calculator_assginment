use std :: io;
fn addition()->f32{
    // print test number of times user inputs:
    println!("Enter 1st value: ");
    let mut num_1 = String::new();
    io::stdin().read_line(&mut num_1);
    let num_1:f32 = num_1.trim().parse().unwrap();

    println!("Enter 2nd value ");
    let mut num_2 = String::new();
    io::stdin().read_line(&mut num_2);
    let num_2:f32 = num_2.trim().parse().unwrap();

    num_1+num_2
}

fn subtraction()->f32{
    // print test number of times user inputs:
    println!("Enter 1st value: ");
    let mut num_1 = String::new();
    io::stdin().read_line(&mut num_1);
    let num_1:f32 = num_1.trim().parse().unwrap();

    println!("Enter 2nd value ");
    let mut num_2 = String::new();
    io::stdin().read_line(&mut num_2);
    let num_2:f32 = num_2.trim().parse().unwrap();

    num_1-num_2
}

fn division()->f32{
    // print test number of times user inputs:
    println!("Enter numrator: ");
    let mut num_1 = String::new();
    io::stdin().read_line(&mut num_1);
    let num_1:f32 = num_1.trim().parse().unwrap();

    println!("Enter Denominator ");
    let mut num_2 = String::new();
    io::stdin().read_line(&mut num_2);
    let num_2:f32 = num_2.trim().parse().unwrap();

    num_1/num_2
}

fn exponent()->f32{
    // print test number of times user inputs:
    println!("Enter value: ");
    let mut num_1 = String::new();
    io::stdin().read_line(&mut num_1);
    let num_1:f32 = num_1.trim().parse().unwrap();

    println!("Enter power: ");
    let mut num_2 = String::new();
    io::stdin().read_line(&mut num_2);
    let num_2:f32 = num_2.trim().parse().unwrap();

    let ans = num_1.powf(num_2) ;
    ans
}

fn multiplication()->f32{
    // print test number of times user inputs:
    println!("Enter 1st value: ");
    let mut num_1 = String::new();
    io::stdin().read_line(&mut num_1);
    let num_1:f32 = num_1.trim().parse().unwrap();

    println!("Enter 2nd value ");
    let mut num_2 = String::new();
    io::stdin().read_line(&mut num_2);
    let num_2:f32 = num_2.trim().parse().unwrap();

    num_1*num_2
}
fn main() {
    loop {  
    println!("1) Addition");
    println!("2) Subtration");
    println!("3) Multiplication");
    println!("4) Division");
    println!("5) exponent");
    println!("0) quit");
    println!("Enter the fuction you wish to perform");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input:i32 = input.trim().parse().unwrap();
    if input==0 {
        break;
    }
    else {
        match input {
            1 => println!("{}",addition()),
            2 => println!("{}",subtraction()),
            3 => println!("{}",multiplication()),
            4 => println!("{}",division()),
            5 => println!("{}",exponent()),
            _ => println!("undefined value entered")
            };  
        };
    };
}
