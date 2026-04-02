use crate::prelude::*;

use std::cmp::min;

use super::{AnnotatedString, AnnotatedStringPart};

/// 带注解字符串的迭代器
/// 按注解边界将字符串分割为多个片段，每次返回一个带注解类型的子串
pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current_idx: ByteIdx,  // 当前遍历到的字节位置
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
                annotation.start <= self.current_idx && self.current_idx < annotation.end
            })
            .last()
        {
            let end_idx = min(annotation.end, self.annotated_string.string.len());
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
            .filter(|a| a.start > self.current_idx)
            .map(|a| a.start)
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
