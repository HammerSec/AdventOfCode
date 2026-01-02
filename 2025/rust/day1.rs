use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut dial_value: i32 = 50;
    let mut zero_counter: i32 = 0;
    let mut file = File::open("../day1.txt").expect("Failed to open the Day 1 data file");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let lines = buffer.lines();

    for line in lines {
        // If the first character is an L
        if line.starts_with("L") {
            //   Subtract the number following L from dial_value
            let amount: i32 = line.rsplit("L").collect::<Vec<_>>()[0].parse().unwrap();
            dial_value -= amount;

            // Keep adding 100 to dial_value until it's positive
            // (This may happen for inputs greater than 100)
            while dial_value < 0 {
                dial_value += 100;
            }

            println!("LEFT {}, DIAL = {}", amount, dial_value);
        }
        // Else if the first character is an R:
        else if line.starts_with("R") {
            //   Add the number following the R to dial_value
            let amount: i32 = line.rsplit("R").collect::<Vec<_>>()[0].parse().unwrap();
            dial_value += amount;

            // Subtract 100 from dial_value until it's below 100
            // (This may happen for inputs greater than 100)
            while dial_value >= 100 {
                dial_value -= 100;
            }

            println!("RIGHT {}, DIAL = {}", amount, dial_value);
        }

        if dial_value == 0 {
            zero_counter += 1;
        }
    }

    println!("Solution: {}", zero_counter);

    Ok(())
}
