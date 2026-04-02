/// 注解类型：定义不同的语法高亮和搜索标记类型
/// 每种类型对应不同的显示颜色（在 terminal/attribute.rs 中定义）
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnnotationType{
    Match,              // 搜索匹配
    SelectedMatch,      // 当前选中的搜索匹配
    Number,             // 数字字面量
    Keyword,            // 语言关键字
    Type,               // 类型名
    KnownValue,         // 已知值（如 Some, None, Ok, Err）
    Char,               // 字符字面量
    LifetimeSpecifier,  // 生命周期标识符
    Comment,            // 注释
    String,             // 字符串字面量
}