//! 高亮系统：协调语法高亮和搜索结果高亮
//! 采用策略模式，根据文件类型动态选择语法高亮器

use super::super::super::{Annotation, AnnotationType, FileType, Line};
use crate::prelude::*;
mod syntaxhighlighter;
use searchresulthighlighter::SearchResultHighlighter;
use syntaxhighlighter::SyntaxHighlighter;
mod rustsyntaxhighlighter;
mod searchresulthighlighter;
use rustsyntaxhighlighter::RustSyntaxHighlighter;

/// 根据文件类型创建对应的语法高亮器（工厂方法）
fn create_syntax_highlighter(file_type: FileType) -> Option<Box<dyn SyntaxHighlighter>> {
    match file_type {
        FileType::Rust => Some(Box::<RustSyntaxHighlighter>::default()),
        FileType::Text => None,
    }
}

/// 高亮器：组合语法高亮和搜索结果高亮
#[derive(Default)]
pub struct Highlighter<'a> {
    syntax_highlighter: Option<Box<dyn SyntaxHighlighter>>,              // 语法高亮器（可选）
    search_result_highlighter: Option<SearchResultHighlighter<'a>>,      // 搜索结果高亮器（可选）
}

impl<'a> Highlighter<'a> {
    pub fn new(
        matched_word: Option<&'a str>,
        selected_match: Option<Location>,
        file_type: FileType,
    ) -> Self {
        let search_result_highlighter = matched_word
            .map(|matched_word| SearchResultHighlighter::new(matched_word, selected_match));
        Self {
            syntax_highlighter: create_syntax_highlighter(file_type),
            search_result_highlighter,
        }
    }
    /// 获取指定行的所有注解（合并语法高亮和搜索高亮）
    pub fn get_annotations(&self, idx: LineIdx) -> Vec<Annotation> {
        let mut result = Vec::new();

        if let Some(syntax_highlighter) = &self.syntax_highlighter {
            if let Some(annotations) = syntax_highlighter.get_annotations(idx) {
                result.extend(annotations.iter().copied());
            }
        }
        if let Some(search_result_highlighter) = &self.search_result_highlighter {
            if let Some(annotations) = search_result_highlighter.get_annotations(idx) {
                result.extend(annotations.iter().copied());
            }
        }
        result
    }
    pub fn highlight(&mut self, idx: LineIdx, line: &Line) {
        if let Some(syntax_highlighter) = &mut self.syntax_highlighter {
            syntax_highlighter.highlight(idx, line);
        }
        if let Some(search_result_highlighter) = &mut self.search_result_highlighter {
            search_result_highlighter.highlight(idx, line);
        }
    }
}