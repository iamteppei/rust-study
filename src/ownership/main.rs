pub fn start() {
    /*
     * Stack vs Heap
     * - Stack (First In Last Out): allocate memory for fixed or known size data so it's faster
     * - Heap: allocate memory for unknown size data. It's slower and refered by using pointer
     */

    /* (1) Ownership is about MOVE or COPY */

    let msg1 = String::from("Hello world!");
    let msg2 = msg1; // msg1 is moved into msg2, and msg1 is dropped (drop means free memory. it is handled automatically by Rust)

    // println!("{msg1}"); // error when refer to msg1 because it's drop
    println!("{msg2}");

    // copy value example
    let count1 = 5;
    let count2 = count1;

    println!("{count1}"); // no error when refer to count1 because its value is COPIED. count1 is i32 (fixed size - signed 32 bit)
    println!("{count2}");

    // How to keep refer to value?
    let msg1 = String::from("Hello world!");

    // msg1 is moved into calculate_len and transfered back by returning it
    // this is called take-ownership and return-owership. it's tedious
    let (length, msg2) = calculate_len(msg1);

    println!("{msg2} length {length}");

    // Another approach is use Reference - like a pointer, use & operator. It's call Borrowing.
    let msg1 = String::from("Hello world!");

    let length = calculate_len_use_reference(&msg1); // rust create a reference to msg1 but do not own it.
    println!("{length}");
    println!("{msg1}"); // no error. calculate_len_use_reference returns back the ownership when it's done

    // now, what happen when calculate_len_use_reference try to modify a borrowing variable?
    // ..
    // it's not allowed. Borrowing is not ownership and reference is immutable like the orthers

    // what if we want to modify the value of borrowing data?
    // ..
    // Use mutable references - &mut <variable-name>
    let mut msg1 = String::from("Hello");

    change(&mut msg1);
    println!("{msg1}");

    // Mutable reference has restriction: can not borrow mutable reference more than 1 at a time
    let mut s1 = String::from("Hello");

    let s2 = &mut s1;
    // let s3 = &mut s1; // this cause an error because you can only have 1 mutable reference to variable at a time

    println!("{s2}");

    // why do this? to prevent date race - multi mutate data at the same time

    // Check this code
    let msg1 = String::from("Hello");

    let msg2 = &msg1; // no issue. msg2 borrow msg1. It's immutable
    let msg3 = &msg1; // no issue. msg3 borrow msg1. It's immutable

    // let msg4 = &mut msg1; // It's not allowed, else msg2 and msg2 may refer to a changed value
    println!("{msg2} and {msg3} and {msg1}");

    // Now, continue checking this code
    let mut msg1 = String::from("Hello world!");

    let msg2 = &msg1; // No issue
    let msg3 = &msg1; // No issue

    println!("{msg2} and {msg3}");
    // msg1 and msg2 are dropped at this step. they are out of scope

    let msg3 = &mut msg1; // this is now okay because msg2 and msg2 are dropped
    println!("{msg3}");

    /*
     * Dangling reference: a pointer that references a location in memory that may have been given to someone else.
     */

    // GOOD NEWS: Rust preventing this
    // let reference = dangle();

    // fn dangle() -> &String {
    //     let s = String::from("Hello world!");
    //     &s
    // } // s is out of scope so its memory is dropped <-- refer to s is danger

    /*
     * Slice Type. Slice refer to a contiguous sequence of elements in a collection.
     * A slice is a reference so it does not have ownership.
     */
}

fn calculate_len(s: String) -> (usize, String) {
    // calculate len and return back value of s
    (s.len(), s)
}

fn calculate_len_use_reference(s: &String) -> usize {
    // this mean s does not have ownership of what it refers to
    s.len()
    // because s does not have ownership, it is not dropped when out of scope
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
