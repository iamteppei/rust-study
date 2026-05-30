pub fn start() {}

#[test]
fn test_match_basic() {
    enum Size {
        Small,
        Medium,
    }

    fn value_in_size(size: Size) -> i8 {
        match size {
            Size::Small => 1,
            Size::Medium => 2,
        }
    }

    assert_eq!(value_in_size(Size::Small), 1);
    assert_eq!(value_in_size(Size::Medium), 2);
}
