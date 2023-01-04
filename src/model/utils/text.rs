use serde::{Deserialize, Serialize};

/// An object describing a part of a text
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TextBlock {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: usize,
    /// Length of the entity in UTF-16 code units
    pub length: usize,
}

impl TextBlock {
    /// Gets the part of the text described by the `TextBlock`
    pub fn get_text(&self, text: &str) -> String {
        let mut res: Vec<u16> = Vec::with_capacity(self.length);

        for (i, point) in text.encode_utf16().enumerate() {
            if (self.offset + self.length) <= i {
                break;
            }
            if self.offset <= i {
                res.push(point);
            }
        }

        String::from_utf16_lossy(res.as_slice())
    }
}
