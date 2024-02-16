use serde::{Serialize, Serializer};

use super::TwoLetter;

impl Serialize for TwoLetter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let json = serde_json::to_string(&TwoLetter::new("us")).unwrap();
        assert_eq!(json, "\"us\"");
    }
}
