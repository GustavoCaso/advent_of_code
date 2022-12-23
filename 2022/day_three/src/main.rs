mod item {
    #[repr(transparent)]
    pub(crate) struct Item(u8);
    #[derive(Debug)]
    pub(crate) struct TypeParseError;

    impl TryFrom<u8> for Item {
        type Error = TypeParseError;

        //  We use pattern matching for binary representation
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Self(value)),
                _ => Err(TypeParseError),
            }
        }
    }

    impl Item {
        pub(crate) fn priority(self) -> usize {
            // Using the ASCII values for each letter we can get the pripority
            // a = 97. a - a + 1 = 1
            // b = 98. 98 - 97 + 1 = 2
            // ...

            // self.0 access u8 from Item
            match self.0.is_ascii_lowercase() {
                true => 1 + (self.0 - b'a') as usize,
                false => 27 + (self.0 - b'A') as usize,
            }
        }
    }
}

// Encapsulate item on a separate module
use item::{Item, TypeParseError};

fn main() -> Result<(), TypeParseError> {
    Ok(())
}
