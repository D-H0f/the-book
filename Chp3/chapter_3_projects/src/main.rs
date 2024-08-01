fn main() {
    {
        // converting fahrenheit to celcius and vice versa
        convert_temp(100.0f64, 'F');
        convert_temp(70.3f64, 'F');
        convert_temp(100.0f64, 'C');
        convert_temp(-5.4f64, 'f');
    }
    {
        // generating the fibonacci sequence to the nth number
        generate_fibonacci(4u16, true);
        generate_fibonacci(10u16, true);
        generate_fibonacci(50u16, true);
    }
}

fn convert_temp(temp: f64, mode: char) {
    // Convert temperatures between Fahrenheit and Celcius
    if mode == 'F' || mode == 'f' {
        println!("Converting Fahrenheit to Celcius:");
        let converted_temp: f64 = (temp - 32.0f64) / 1.8f64;
        println!("{temp}°F = {:.2}°C", converted_temp);
    } else if mode == 'C' || mode == 'c' {
        println!("Converting Celcius to Fahrenheit:");
        let converted_temp: f64 = (temp * 1.8f64) + 32.0f64;
        println!("{temp}°C = {:.2}°F", converted_temp);
    } else {
        println!("{mode} is not a valid parameter for convert_temp");
    }
}
fn generate_fibonacci(n: u16, show_sequence: bool) {
    if n == 0 || n == 1 {
        println!("{n}");
    } else {
        let mut previous_number: u128 = 0;
        let mut current_number: u128 = 0;
        for count in 0..=n {
            if count < 1 {
                current_number += 1;
            } else {
                current_number += previous_number;
                previous_number = current_number - previous_number;
            }
            if show_sequence {
                println!("{}: {previous_number}", count,);
            }
        }
        println!("The {n}th number of the Fibonacci sequence is {previous_number}");
    }
}
