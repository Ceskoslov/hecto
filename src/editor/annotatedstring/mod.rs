use std::{
    fmt::{self, Display},
    cmp::{max, min},
};
pub mod annotationtype;
pub use annotationtype::AnnotationType;
mod annotation;
use annotation::Annotation;
mod annotatedstringpart;
pub use annotatedstringpart::AnnotatedStringPart;
mod annotatedstringiterator;
pub use annotatedstringiterator::AnnotatedStringIterator;

#[derive(Debug, Default)]
pub struct AnnotatedString {
    string: String,
    annotations: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn from(string: &str) -> Self {
        Self {
            string: String::from(string),
            annotations: Vec::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation_type: AnnotationType, start_byte_index: usize, end_byte_index: usize) {
        debug_assert!(start_byte_index <= end_byte_index, "start_byte_index should be less than or equal to end_byte_index");
        self.annotations.push(Annotation {
            annotation_type,
            start_byte_index,
            end_byte_index,
        });
    }

    pub fn replace(&mut self, start_byte_index: usize, end_byte_index: usize, new_string: &str){
        debug_assert!(start_byte_index <= end_byte_index);

        let end_byte_index = min(end_byte_index, self.string.len());

        if start_byte_index > end_byte_index {
            return;
        }

        self.string.replace_range(start_byte_index..end_byte_index, new_string);
        let replaced_range_len = end_byte_index - start_byte_index;
        let shortened = new_string.len() < replaced_range_len;
        let length_difference = new_string.len().abs_diff(replaced_range_len);
        if length_difference == 0 {
            return;
        }

        self.annotations.iter_mut().for_each(|annotation| {
            annotation.start_byte_index = if annotation.start_byte_index > end_byte_index {
                if shortened {
                    annotation.start_byte_index.saturating_sub(length_difference)
                } else {
                    annotation.start_byte_index.saturating_add(length_difference)
                }
            } else if annotation.start_byte_index >= start_byte_index {
                if shortened {
                    max(annotation.start_byte_index.saturating_sub(length_difference), start_byte_index)
                } else {
                    min(annotation.start_byte_index.saturating_add(length_difference), end_byte_index)
                }
            } else {
                annotation.start_byte_index
            };

            annotation.end_byte_index = if annotation.end_byte_index > end_byte_index {
                if shortened {
                    annotation.end_byte_index.saturating_sub(length_difference)
                } else {
                    annotation.end_byte_index.saturating_add(length_difference)
                }
            } else if annotation.end_byte_index >= start_byte_index {
                if shortened {
                    max(annotation.end_byte_index.saturating_sub(length_difference), start_byte_index)
                } else {
                    min(annotation.end_byte_index.saturating_add(length_difference), end_byte_index)
                }
            } else {
                annotation.end_byte_index
            };
        });

        self.annotations.retain(|annotation| annotation.start_byte_index < annotation.end_byte_index && annotation.start_byte_index < self.string.len());
    }
}

impl Display for AnnotatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl <'a> IntoIterator for &'a AnnotatedString {
    type Item = AnnotatedStringPart<'a>;
    type IntoIter = AnnotatedStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AnnotatedStringIterator {
            annotated_string: self,
            current_idx: 0,
        }
    }    
}