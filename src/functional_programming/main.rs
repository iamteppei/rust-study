pub fn start() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_closure() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        // closure
        let mut borrows_mutably = || list.push(7);

        // println!("After defining closure: {list:?}"); // error because borrows_mutably captured mutable so no immutable is allowed

        borrows_mutably();
        println!("After calling closure: {list:?}");
    }
}
