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
}

// Encapsulate item on a separate module
use item::{Item, TypeParseError};

fn main() -> Result<(), TypeParseError> {
    Ok(())
}
