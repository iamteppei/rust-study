pub fn start() {}

#[test]
fn test_enum() {
    enum IpAddress {
        V4(u8, u8, u8, u8), // each variant can have different type and amount of associated data. This is different with language like Java
        V6(String),
        Other {
            value: String,
        },
    }

    let home = IpAddress::V4(127, 0, 0, 1);

    // Access to value of variant by using patten matching
    match &home {
        IpAddress::V4(first, _, _, _) => {
            assert_eq!(&127, first);
        }
        IpAddress::V6(str) => {
            assert_eq!("::1", str);
        }
        IpAddress::Other { value } => {
            //
        }
    }

    // The match above have to cover all of variants.
    // When we only want to work with a specific variant, we can use catch-all pattern (or if)
    match &home {
        IpAddress::V4(first, _, _, _) => {
            assert_eq!(&127, first);
        }
        _other => (),
    }

    // OR using if let
    if let IpAddress::V4(first, second, third, fourth) = &home {
        assert_eq!(first, &127);
    }
}

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
