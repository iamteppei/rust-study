pub fn start() {}

#[cfg(test)]
mod tests {
    fn calculate_len(s: String) -> (usize, String) {
        (s.len(), s)
    }

    fn calculate_len_by_ref(s: &String) -> usize {
        s.len()
    }

    fn append_world(s: &mut String) {
        s.push_str(", world!");
    }

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b' ' {
                return &s[..i];
            }
        }
        s
    }

    // (1) Move: assigning a String to another binding transfers ownership.
    // The original binding is no longer valid after the move.
    #[test]
    fn test_move_transfers_ownership() {
        let msg1 = String::from("Hello world!");
        let msg2 = msg1; // msg1 is moved into msg2
        // msg1 is no longer accessible here
        assert_eq!(msg2, "Hello world!");
    }

    // (2) Copy: primitive types that implement the Copy trait are copied on assignment.
    // Both bindings remain valid.
    #[test]
    fn test_copy_keeps_original_valid() {
        let count1 = 5_i32;
        let count2 = count1; // i32 implements Copy, so count1 is copied
        assert_eq!(count1, 5);
        assert_eq!(count2, 5);
    }

    // (3) Take and return ownership: one way to use a value in a function and keep access to it
    // is to take ownership and return it back alongside the result.
    #[test]
    fn test_take_and_return_ownership() {
        let msg1 = String::from("Hello world!");
        let (length, msg2) = calculate_len(msg1);
        // msg1 was moved into calculate_len, but returned back as msg2
        assert_eq!(length, 12);
        assert_eq!(msg2, "Hello world!");
    }

    // (4) Borrowing: use & to pass a reference instead of moving the value.
    // The function borrows the value without taking ownership.
    #[test]
    fn test_borrow_does_not_move() {
        let msg1 = String::from("Hello world!");
        let length = calculate_len_by_ref(&msg1); // msg1 is borrowed, not moved
        assert_eq!(length, 12);
        assert_eq!(msg1, "Hello world!"); // msg1 is still valid
    }

    // (5) Mutable reference: use &mut to allow a function to modify a borrowed value.
    // Only one mutable reference to a value may exist at a time.
    #[test]
    fn test_mutable_reference_allows_mutation() {
        let mut msg1 = String::from("Hello");
        append_world(&mut msg1);
        assert_eq!(msg1, "Hello, world!");
    }

    // (6) Multiple immutable references: any number of shared (&) references may coexist
    // as long as no mutable reference is active at the same time.
    #[test]
    fn test_multiple_immutable_references_are_allowed() {
        let msg1 = String::from("Hello");
        let msg2 = &msg1;
        let msg3 = &msg1;
        assert_eq!(msg2, msg3);
    }

    // (7) Mutable reference after immutable references go out of scope:
    // once all immutable borrows are done, a mutable borrow is permitted.
    #[test]
    fn test_mutable_reference_allowed_after_immutable_scope_ends() {
        let mut msg1 = String::from("Hello");
        {
            let msg2 = &msg1;
            let msg3 = &msg1;
            assert_eq!(msg2, msg3); // immutable borrows used and dropped here
        }
        let msg4 = &mut msg1; // mutable borrow is now allowed
        msg4.push_str(", world!");
        assert_eq!(msg1, "Hello, world!");
    }

    // (8) Slice type: a slice is a reference to a contiguous sequence of elements.
    // It does not own the data.
    #[test]
    fn test_slice_does_not_own_data() {
        let sentence = String::from("Hello world");
        let word = first_word(&sentence); // &str slice borrows from sentence
        assert_eq!(word, "Hello");
        // sentence is still valid because slice only borrows it
        assert_eq!(sentence, "Hello world");
    }
}
