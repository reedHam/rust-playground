fn main() {
    let x = 5;

    let y = {
        println!("The value of x is: {}", x);
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}