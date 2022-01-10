use std::io;

fn main() {
    const MONTH_ARRAY: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    loop {
        println!("Enter a number between 1 and 12 to get the month represented by that number:");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line.");

        let index: u8 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input.");
                continue;
            }
        };

        if index == 0 || index > 12 {
            println!("Invalid Input.");
            continue;
        }

        println!("{}", MONTH_ARRAY[index as usize - 1]);
    }
}
