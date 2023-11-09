use std::io;

fn main() {
    let mut x: String = String::new();
    let mut y: String = String::new();
    let result: i32;
    let mut op: String = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut x).expect("Invalid Input");
    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Invalid input!");
            return;
        }
    };

    println!("Enter the second number: ");
    io::stdin().read_line(&mut y).expect("Invalid Input");
    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Invalid input!");
            return;
        }
    };

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number assoc with the desidred operation");
    
    io::stdin().read_line(&mut op).expect("Invalid Input");
    let op: i32 = match op.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Invalid input!");
            return;
        }
    };

    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => result = x / y,
        _ => {
            println!("Invalid selection");
            return;
        }
    }
    println!("The result is: {}", result);
}
