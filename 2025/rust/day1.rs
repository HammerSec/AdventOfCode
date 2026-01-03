use std::fs;

// Returns the final dial value and number of times zero is passed
fn process_combo(contents: &String) -> (i32, i32) {
    let mut dial_value: i32 = 50;
    let mut zero_counter: i32 = 0;

    let lines = contents.lines();

    // TODO: Adjust business logic, it's not producing the correct output

    for line in lines {
        // If the first character is an L
        if line.starts_with("L") {
            //   Subtract the number following L from dial_value
            let amount: i32 = line.rsplit("L")
                .collect::<Vec<_>>()[0]
                .parse() // <- Today I learned about this!
                .unwrap();
            dial_value -= amount;

            // Keep adding 100 to dial_value until it's positive
            // (This may happen for inputs greater than 100)
            while dial_value < 0 {
                dial_value += 100;
                zero_counter += 1;
            }

            if dial_value == 0 {
                zero_counter += 1;
            }

            println!("LEFT {}, DIAL = {}", amount, dial_value);
        }
        // Else if the first character is an R:
        else if line.starts_with("R") {
            //   Add the number following the R to dial_value
            let amount: i32 = line.rsplit("R")
                .collect::<Vec<_>>()[0]
                .parse()
                .unwrap();
            dial_value += amount;

            // Subtract 100 from dial_value until it's below 100
            // (This may happen for inputs greater than 100)
            while dial_value >= 100 {
                dial_value -= 100;
                zero_counter += 1;
            }

            println!("RIGHT {}, DIAL = {}", amount, dial_value);
        }
    }

    (dial_value, zero_counter)

}

// Tests are absolutely NOT necessary to solve this, but I kept hitting snags
// while attempting to sprint my way to a solution. Tests help me walk my
// way to a solution, YMMV.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_this_thing_on() {
        let c: String = "L50".to_string();
        let (dial, zeroes) = process_combo(&c);
        assert_eq!(dial, 0, "Dial is at {dial}, expected 0");
        assert_eq!(zeroes, 1);
    }

    #[test]
    fn basic_dial_underflow() {
        let c: String = "L51".to_string();
        let (dial, zeroes) = process_combo(&c);
        assert_eq!(dial, 99, "Dial is at {dial}, expected 99");
        assert_eq!(zeroes, 1, "Zeroes is at {zeroes}, expected 1");
    }

    #[test]
    fn basic_dial_overflow() {
        let c: String = "R51".to_string();
        let (dial, zeroes) = process_combo(&c);
        assert_eq!(dial, 1, "Dial is at {dial}, expected 1");
        assert_eq!(zeroes, 1, "Zeroes is at {zeroes}, expected 1");
    }

    #[test]
    fn dial_underflow_land_on_zero() {
        let c: String = "L350".to_string();
        let (dial, zeroes) = process_combo(&c);
        assert_eq!(dial, 0, "Dial didn't land on zero, landed at {dial}");
        assert_eq!(zeroes, 4, "Zeroes is at {zeroes}, expected 4");
    }

    #[test]
    fn advent_of_code_demo_test() {
        let c: String = "L68\n\
        L30\n\
        R48\n\
        L5"
        .to_string();
        
        let (dial, zeroes) = process_combo(&c);
        assert_eq!(dial, 95, "Dial set to wrong position");
        assert_eq!(zeroes, 2, "Zeroes are incorrect");
    }
}

fn main() {
    let file_path = "../day1.txt".to_string();
    let buffer: String = fs::read_to_string(file_path)
        .expect("Failed to read the contents of the file");

    let answers = process_combo(&buffer); 

    println!("Solution: {}", answers.1);

}
