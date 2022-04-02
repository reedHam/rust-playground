fn append_question_mark(s: &mut String) {
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
    append_question_mark(&mut s5);
    // here s5 is owned by s5 again
    println!("s5: {}", s5); // What?

    let mut str_vec = vec![String::from("hello"), String::from("world")];
    // Here we create a vector of strings on the heap
    println!("str_vec: {:?}", str_vec); // ["hello", "world"]

    // since you cannot borrow a variable more than once at a time
    // we cant swap elements in the vector
    // e.g.
    // let s = str_vec[0];
    // let t = str_vec[1];
    // str_vec[0] = t;
    // str_vec[1] = s;
    // This will not compile since we cannot borrow str_vec more than once at a time
    // to accomplish this we need to use unsafe code,
    unsafe {
        // Here we get raw pointers to the first and second elements of the vector
        let ptr_a: *mut String = &mut str_vec[0];
        let ptr_b: *mut String = &mut str_vec[1];
        // we then call the pointer swap function
        std::ptr::swap(ptr_a, ptr_b);
    }
    println!("str_vec: {:?}", str_vec); // ["world", "hello"]

    // there is a function that encapsulates the swap function
    str_vec.swap(0, 1);
    println!("str_vec: {:?}", str_vec); // ["hello", "world"]

    let mut str_vec1 = vec![String::from("hello"), String::from("world")];
    let mut str_vec2 = vec![String::from("hi"), String::from("mom")];
    // Here we create two vectors of references to strings
    println!("str_vec1: {:?}", str_vec1); // ["hello", "world"]
    println!("str_vec2: {:?}", str_vec2); // ["hi", "mom"]

    // now we will preform a memory swap
    std::mem::swap(&mut str_vec1[0], &mut str_vec2[0]); // also uses unsafe code
    println!("str_vec1: {:?}", str_vec1); // ["hi", "world"]
    println!("str_vec2: {:?}", str_vec2); // ["hello", "mom"]
}
