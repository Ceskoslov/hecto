use std::cmp::min;

use super::{AnnotatedString, AnnotatedStringPart};

pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current_idx: usize,
}

impl<'a> Iterator for AnnotatedStringIterator<'a> {
    type Item = AnnotatedStringPart<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.annotated_string.parts.len() {
            return None;
        }
        if let Some(annotation) = self.annotated_string.annotations.iter().filter(|annotation| {
            annotation.start_part_index <= self.current_idx && self.current_idx < annotation.end_part_index
        }).last()
        {
            let end_idx = min(annotation.end_part_index, self.annotated_string.string.len());
            let start_idx = self.current_idx;

            self.current_idx = end_idx;
            return Some(AnnotatedStringPart {
                string: &self.annotated_string.string[start_idx..end_idx],
                annotation_type: None,
            });
        }
    }
}    

    