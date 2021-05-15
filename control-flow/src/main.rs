fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");



    let array = [1, 2, 3, 4, 5];
    
    for element in array.iter() {
        println!("The value of element {}", element);
    } 

    for i in 1..=10 {
        println!("The value of i {}", i);
    }

    for i in (1..=10).rev() {
        println!("The value of i {}", i);
    }
}