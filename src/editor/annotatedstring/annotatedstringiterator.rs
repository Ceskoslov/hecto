use std::cmp::min;

use super::{AnnotatedString, AnnotatedStringPart};

pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current_idx: usize,
}

impl<'a> Iterator for AnnotatedStringIterator<'a> {
    type Item = AnnotatedStringPart<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.annotated_string.string.len() {
            return None;
        }
        if let Some(annotation) = self
            .annotated_string
            .annotations
            .iter()
            .filter(|annotation| {
                annotation.start_byte_index <= self.current_idx
                    && self.current_idx < annotation.end_byte_index
            })
            .last()
        {
            let end_idx = min(
                annotation.end_byte_index,
                self.annotated_string.string.len(),
            );
            let start_idx = self.current_idx;

            self.current_idx = end_idx;
            return Some(AnnotatedStringPart {
                string: &self.annotated_string.string[start_idx..end_idx],
                annotation_type: Some(annotation.annotation_type),
            });
        }

        // No annotation at current position — advance to next annotation start or end of string
        let next_annotation_start = self
            .annotated_string
            .annotations
            .iter()
            .filter(|a| a.start_byte_index > self.current_idx)
            .map(|a| a.start_byte_index)
            .min()
            .unwrap_or(self.annotated_string.string.len());

        let end_idx = min(next_annotation_start, self.annotated_string.string.len());
        let start_idx = self.current_idx;
        self.current_idx = end_idx;
        Some(AnnotatedStringPart {
            string: &self.annotated_string.string[start_idx..end_idx],
            annotation_type: None,
        })
    }
}
