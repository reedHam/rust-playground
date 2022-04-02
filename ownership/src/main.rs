fn appendQuestionMark(s: &mut String) {
    s.push_str("?");
}

fn main() {
    // a is a i32 defined on the stack
    let a = 1;

    // b is a mutable copy of a with its own memory
    let mut b = a;
    b = b + 1;
    println!("a: {}", a); // 1
    println!("b: {}", b); // 2

    // s1 is a reference to a string stored on the stack.
    let s1 = String::from("hello");
    // since s1 is a reference we must clone it to get a mutable copy
    let mut s2 = s1.clone();
    s2.push_str(", world");
    println!("s1: {}", s1); // hello
    println!("s2: {}", s2); // hello, world

    // s3 is a reference to a string stored on the heap.
    let mut s3 = String::from("hi");
    // here we set s4 to a reference to s3
    // Since both s3 and s4 point to the same string, changes made in one will be reflected in the other.
    let s4 = &mut s3;
    s4.push_str(", Mom!");
    // we can only borrow each reference once, so we need to print s4 first
    // here the reference to s4 is dropped, but the string is still on the heap
    println!("s4: {}", s4); // hi, Mom!

    // Since we dropped the reference s4 above we can now print s3
    println!("s3: {}", s3); // hi, Mom!

    // Here we create a new string on the heap and assign it to s5
    let mut s5 = String::from("What");
    // Append question mark will borrow s5 and mutate it
    // because s5 is a mutable reference we after the function s5 is not dropped
    appendQuestionMark(&mut s5);
    // here s5 is owned by s5 again
    println!("s5: {}", s5); // What?
}
