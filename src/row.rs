use std::cmp;

use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    pub len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}
impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);

        let mut result = String::new();
        self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .for_each(|grapheme| {
                if grapheme == "\t" {
                    result.push_str(" ");
                } else {
                    result.push_str(grapheme);
                }
            });

        result
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    fn update_len(&mut self) {
        self.len = self.string.graphemes(true).count();
    }
}
