use std::io::{stdin,stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("FLUSH failed");
    stdin().read_line(input)
        .expect("READ failed");
}

fn main() {
    println!("");
    println!("Welcomn to calcutron v1.00");
    println!("------------------------");
  
    loop {

        let mut first_number  = String::new();
        let mut second_number = String::new();
        let mut operator      = String::new();

        print!("Type in first number: ");
        read(&mut first_number);

        print!("Type in second number: ");
        read(&mut second_number);
        
        // NUMBERS are converted to FLOATING POINT
        print!("Type in operation | + | - | * | / |: ");
        read(&mut operator);

        
        //println!("{} {} {}", first_number,second_number, operator);

        let first_number: f32 = first_number.trim().parse().unwrap();
        let second_number: f32 = second_number.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let operators = String::from("+-*/");

        if !operators.contains(operator) {
            println!("unknown INPUT, select from | + | - | * | / |");
            continue;
        }

        let result = match operator {
            '+' => first_number + second_number,
            '-' => first_number - second_number,
            '*' => first_number * second_number,
            '/' => first_number / second_number,
            _ => panic!("error in operator")
        };

        println!("");
        println!("     :_RESULT_:     ");
        println!("");
        println!("     {} {} {} = {}", first_number, operator, second_number, result);
        println!("");
}
}
