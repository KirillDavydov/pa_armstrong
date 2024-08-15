use std::io;
use std::io::Write;

fn read_int() -> Option<u32> {
    loop {
        let mut buf = String::new();
        print!("Please enter positive integer (or q to exit): ");
        io::stdout().flush().expect("can not flush");
        io::stdin().read_line(&mut buf).expect("can not read");

        if buf.chars().next().unwrap() == 'q' {
            return None;
        } else if let Ok(result) = buf.trim().parse::<u32>() {
            return Some(result);
        } else {
            println!("Input is not valid {}", buf);
        }
    }
}

fn main() {
    println!("|=================================================================|");
    println!("|                Welcome to Armstrog number!                      |");
    println!("|=================================================================|");
    println!("| A number is called an Armstrong number if the sum of the cubes  |");
    println!("| of the digits of the number is equal to the number.             |");
    println!("| For example 153 = 1^3 + 5^3 + 3^3                               |");
    println!("|=================================================================|");
    println!("");
    loop {
        if let Some(num) = read_int() {
            if is_armstrong(num) {
                println!("{num} is an Armstrong number");
                println!("");
            } else {
                println!("{num} is not an Armstrong number");
                println!("");
            }
        } else {
            println!("Goodbye");
            return;
        }
    }
}

fn is_armstrong(number: u32) -> bool {
    let mut sum: u32 = 0;
    let mut num = number;

    if number == 0 {
        return true;
    }

    loop {
        let rem = num % 10;
        num = num / 10;
        sum = sum + rem.pow(3);

        if sum > number {
            return false;
        }

        if num == 0 {
            if sum == number {
                return true;
            } else {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive() {
        assert!(is_armstrong(0));
        assert!(is_armstrong(1));
        assert!(is_armstrong(153));
        assert!(is_armstrong(370));
        assert!(is_armstrong(371));
        assert!(is_armstrong(407));
    }

    #[test]
    fn negative() {
        assert_eq!(is_armstrong(2), false);
        assert_eq!(is_armstrong(3), false);
        assert_eq!(is_armstrong(10), false);
        assert_eq!(is_armstrong(250), false);
        assert_eq!(is_armstrong(333), false);
        assert_eq!(is_armstrong(95436), false);
    }
}
