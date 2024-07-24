use std::io;

pub fn Calculate() {
    // String input
    let mut str_value: String = String::new();
    println!("Enter a String   ");
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading number 1");

    println!("Your Stirng  is String {} ", str_value);

    // Numbers Input

    let mut number: String = String::new();

    println!("Enter the number ");
    io::stdin()
        .read_line(&mut number)
        .expect("Error readin number ");

    let number: i32 = number.trim().parse().expect("Failed to read number ");
    println!(" the integer is {}", number);

    // same goes with precision baed numbers

    let mut float_number: String = String::new();

    println!("Enter the Floating number");
    io::stdin()
        .read_line(&mut float_number)
        .expect("Error reading number ");

    let float_number: f32 = float_number.trim().parse().expect("Failed to read number");

    println!("Floating Number is : {}", float_number);
    println!("Text float_number{}", float_number);
}

pub fn calculator() {
    let mut num2 = String::new();
    let mut num1 = String::new();

    println!("Enter the first number");

    io::stdin()
        .read_line(&mut num1)
        .expect("Error readin number");
    let num1: f32 = num1.trim().parse().expect("Faild to parse number");

    // se cond  number  input

    println!("Enter the Second  number");

    io::stdin()
        .read_line(&mut num2)
        .expect("Error readin number");
    let num2: f32 = num2.trim().parse().expect("Faild to parse number");

    // taking operator input

    let mut operator = String::new();

    println!("Enter the operator Amon( +, -, *, /)");
    io::stdin()
        .read_line(&mut operator)
        .expect("Erorr reading operator");

    let operator: char = operator
        .trim()
        .parse()
        .expect("Failed to prase to char vlaue ");

    // switch statement for performing calculation

    match operator {
        '+' => println!("The result  is {}", num1 + num2),
        '-' => println!("The result  is {}", num1 - num2),
        '*' => println!("The result  is {}", num1 * num2),
        '/' => {
            if num2 > 0.0 {
                println!("The result is {}", num1 / num2);
            } else {
                println!("Error division by zero");
            }
        }
        _ => println!("Invalid operator"),
    }
}
