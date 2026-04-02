use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use super::{Annotation, AnnotationType, Line, SyntaxHighlighter};
use crate::prelude::*;

#[derive(Default)]
pub struct RustSyntaxHighlighter {
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

fn is_valid_number(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }
    if is_numeric_literal(word) {
        return true;
    }
    let mut chars = word.chars();

    if let Some(first) = chars.next() {
        if !first.is_ascii_digit() {
            return false;
        }
    }

    let mut seen_dot = false;
    let mut seen_e = false;
    let mut prev_was_digit = true;

    for char in chars {
        match char {
            '0'..='9' => {
                prev_was_digit = true;
            }
            '_' => {
                if !prev_was_digit {
                    return false;
                }
                prev_was_digit = false;
            }
            '.' => {
                if seen_dot || seen_e || !prev_was_digit {
                    return false;
                }
                seen_dot = true;
                prev_was_digit = false;
            }
            'e' | 'E' => {
                if seen_e || !prev_was_digit {
                    return false;
                }
                seen_e = true;
                prev_was_digit = false;
            }
            _ => return false,
        }
    }
    prev_was_digit

}

fn is_numeric_literal(word: &str) -> bool {
    if word.len() < 3 {
        return false;
    }
    let mut chars = word.chars();
    if chars.next() != Some('0') {
        return false;
    }

    let base = match chars.next() {
        Some('b'|'B') => 2,
        Some('o'|'O') => 8,
        Some('x'|'X') => 16,
        _ => return false,
    };
    chars.all(|c| c.is_digit(base))
}

impl SyntaxHighlighter for RustSyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        for (start_idx, word) in line.split_word_bound_indices(){
            if is_valid_number(word) {
                result.push(Annotation {
                    start: start_idx,
                    end: start_idx.saturating_add(word.len()),
                    annotation_type: AnnotationType::Number,
                });
            }
        }
        self.highlights.insert(idx, result);
    }
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
}