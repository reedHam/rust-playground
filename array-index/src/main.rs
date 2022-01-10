use std::io;

// Define 12 pointers on the stack to strings in the heap
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

fn get_month(index: u8) -> Result<&'static str, &'static str> {
    if index == 0 || index > 12 {
        return Err("Invalid month index");
    }

    return Ok(MONTH_ARRAY[index as usize - 1]);
}

fn main() {
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

        let month = match get_month(index) {
            Ok(month) => month,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        println!("{}", month);
    }
}
