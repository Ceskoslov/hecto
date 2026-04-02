//! 带注解的字符串：将文本内容与显示注解（语法高亮、搜索匹配）绑定在一起
//! 支持截断、替换操作时自动调整注解位置

use std::{
    cmp::{max, min},
    fmt::{self, Display},
};
use super::{AnnotationType, Annotation};
mod annotatedstringpart;
pub use annotatedstringpart::AnnotatedStringPart;
mod annotatedstringiterator;
pub use annotatedstringiterator::AnnotatedStringIterator;

use super::ByteIdx;

/// 带注解的字符串：存储原始文本和其上的注解列表
#[derive(Debug, Default)]
pub struct AnnotatedString {
    string: String,                  // 原始文本内容
    annotations: Vec<Annotation>,    // 注解列表（各种高亮区间）
}

impl AnnotatedString {
    pub fn from(string: &str) -> Self {
        Self {
            string: String::from(string),
            annotations: Vec::new(),
        }
    }

    pub fn add_annotation(
        &mut self,
        annotation_type: AnnotationType,
        start: ByteIdx,
        end: ByteIdx,
    ) {
        debug_assert!(
            start <= end,
            "start_byte_index should be less than or equal to end_byte_index"
        );
        self.annotations.push(Annotation {
            annotation_type,
            start,
            end,
        });
    }

    pub fn truncate_left_until(&mut self, until: ByteIdx) {
        self.replace(0, until, "");
    }
    pub fn truncate_right_from(&mut self, from: ByteIdx) {
        self.replace(from, self.string.len(), "");
    }

    /// 替换指定字节范围的内容，并自动调整所有注解的偏移位置
    /// 这是维护注解一致性的关键方法
    pub fn replace(&mut self, start: ByteIdx, end: ByteIdx, new_string: &str) {
        let end = min(end, self.string.len());
        debug_assert!(start <= end);
        debug_assert!(start <= self.string.len());

        if start > end {
            return;
        }

        self.string.replace_range(start..end, new_string);
        let replaced_range_len = end.saturating_sub(start);
        let shortened = new_string.len() < replaced_range_len;
        let length_difference = new_string.len().abs_diff(replaced_range_len);
        if length_difference == 0 {
            return;
        }

        self.annotations.iter_mut().for_each(|annotation| {
            annotation.start = if annotation.start > end {
                if shortened {
                    annotation.start.saturating_sub(length_difference)
                } else {
                    annotation.start.saturating_add(length_difference)
                }
            } else if annotation.start >= start {
                if shortened {
                    max(annotation.start.saturating_sub(length_difference), start)
                } else {
                    min(annotation.start.saturating_add(length_difference), end)
                }
            } else {
                annotation.start
            };

            annotation.end = if annotation.end > end {
                if shortened {
                    annotation.end.saturating_sub(length_difference)
                } else {
                    annotation.end.saturating_add(length_difference)
                }
            } else if annotation.end >= start {
                if shortened {
                    max(annotation.end.saturating_sub(length_difference), start)
                } else {
                    min(annotation.end.saturating_add(length_difference), end)
                }
            } else {
                annotation.end
            };
        });

        self.annotations.retain(|annotation| {
            annotation.start < annotation.end && annotation.start < self.string.len()
        });
    }
}

impl Display for AnnotatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a> IntoIterator for &'a AnnotatedString {
    type Item = AnnotatedStringPart<'a>;
    type IntoIter = AnnotatedStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AnnotatedStringIterator {
            annotated_string: self,
            current_idx: 0,
        }
    }
}
