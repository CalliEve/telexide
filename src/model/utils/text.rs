use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextBlock {
    offset: usize,
    length: usize,
}

impl TextBlock {
    pub fn get_text(&self, text: &str) -> String {
        let mut res: Vec<u16> = Vec::with_capacity(self.length);

        for (i, point) in text.encode_utf16().enumerate() {
            if (self.offset + self.length) <= i {
                break;
            }
            if self.offset <= i {
                res.push(point)
            }
        }

        String::from_utf16_lossy(res.as_slice())
    }
}
