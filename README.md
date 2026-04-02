# Hecto —— 用 Rust 从零构建的终端文本编辑器

## 项目架构

```
src/
├── main.rs                    # 入口：创建 Editor 实例并运行事件循环
├── prelude/                   # 全局类型别名和基础数据结构
│   ├── mod.rs                 # GraphemeIdx, LineIdx, ByteIdx, ColIdx, RowIdx 类型别名
│   ├── location.rs            # Location: 文本逻辑坐标（行号 + 字素簇索引）
│   ├── position.rs            # Position: 屏幕渲染坐标（行 + 列）
│   └── size.rs                # Size: 宽高尺寸
└── editor/
    ├── mod.rs                 # Editor 主结构体：事件循环、命令分发、UI 协调
    ├── annotation.rs          # Annotation: 文本注解（高亮区间）
    ├── annotationtype.rs      # AnnotationType: 注解类型枚举（关键字、类型、注释等）
    ├── documentstatus.rs      # DocumentStatus: 文档状态（供状态栏显示）
    ├── filetype.rs            # FileType: 文件类型枚举（Rust / Text）
    ├── command/               # 命令解析：将键盘事件转化为编辑器命令
    │   ├── mod.rs             # Command 枚举：Move / Edit / System
    │   ├── edit.rs            # Edit: 插入、删除、换行等编辑命令
    │   ├── movecommand.rs     # Move: 光标移动命令（方向键、翻页、行首行尾）
    │   └── system.rs          # System: 系统命令（保存、退出、搜索、取消）
    ├── line/                  # 行处理：Unicode 字素簇感知的单行文本
    │   ├── mod.rs             # Line: 行的核心数据结构（编辑、搜索、显示）
    │   ├── graphemewidth.rs   # GraphemeWidth: 半角/全角宽度
    │   └── textfragment.rs    # TextFragment: 字素簇片段（字节偏移、替换字符、宽度）
    ├── annotatedstring/       # 带注解的字符串系统
    │   ├── mod.rs             # AnnotatedString: 文本 + 注解列表
    │   ├── annotatedstringpart.rs      # 迭代器产出的片段
    │   └── annotatedstringiterator.rs  # 按注解边界分割的迭代器
    ├── terminal/              # 终端抽象层
    │   ├── mod.rs             # Terminal: 封装 crossterm 的统一接口
    │   └── attribute.rs       # Attribute: 注解类型到 RGB 颜色的映射
    └── uicomponents/          # UI 组件
        ├── mod.rs             # 统一导出所有组件
        ├── uicomponent.rs     # UIComponent trait: 组件公共接口
        ├── commandbar.rs      # CommandBar: 底部输入栏（搜索/另存为）
        ├── messagebar.rs      # MessageBar: 消息提示栏（带过期机制）
        ├── statusbar.rs       # StatusBar: 状态栏（文件信息、光标位置）
        └── view/              # 核心编辑视图
            ├── mod.rs         # View: 文本渲染、光标、滚动、搜索
            ├── buffer.rs      # Buffer: 文档存储、文件 I/O、搜索
            ├── fileinfo.rs    # FileInfo: 文件路径和类型
            ├── searchdirection.rs  # SearchDirection: 搜索方向
            ├── searchinfo.rs  # SearchInfo: 搜索状态保存
            └── highlighter/   # 高亮系统
                ├── mod.rs     # Highlighter: 组合语法高亮和搜索高亮
                ├── syntaxhighlighter.rs         # SyntaxHighlighter trait
                ├── searchresulthighlighter.rs   # 搜索结果高亮器
                └── rustsyntaxhighlighter.rs     # Rust 语法高亮器
```

---

### 1. Rust 语言核心特性

#### 所有权与生命周期
- **`Highlighter<'a>`** 使用生命周期参数引用搜索查询字符串，避免不必要的拷贝
- **`AnnotatedStringPart<'a>`** 和 **`AnnotatedStringIterator<'a>`** 是零拷贝迭代器设计的范例：迭代时直接借用原始字符串的切片
- `SearchResultHighlighter<'a>` 中 `matched_word: &'a str` 演示了如何在结构体中持有借用

#### Trait 和多态
- **`UIComponent` trait** 定义了 `set_needs_redraw`、`resize`、`render`、`draw` 等统一接口，`View`、`StatusBar`、`MessageBar`、`CommandBar` 共用同一套渲染流程
- **`SyntaxHighlighter` trait** 实现策略模式，`RustSyntaxHighlighter` 和 `SearchResultHighlighter` 可以独立替换
- **`TryFrom<Event>` trait** 用于将终端事件安全转换为编辑器命令，展示了 Rust 的类型安全转换模式

#### 枚举与模式匹配
- `Command` 三层枚举（`Move` / `Edit` / `System`）将所有用户操作建模为数据类型
- `PromptType` 枚举驱动状态机，在搜索、保存、正常三种模式之间切换
- `AnnotationType` 枚举统一管理所有高亮类型

#### 错误处理
- `TryFrom` 返回 `Result` 处理无法识别的键盘事件
- `Buffer::load` 使用 `Result<Self, Error>` 处理文件 I/O 错误
- `Editor::new` 中的 panic hook 确保即使程序崩溃也能恢复终端状态

### 2. Unicode 处理

#### 字素簇（Grapheme Clusters）
- 使用 `unicode-segmentation` 库正确分割 Unicode 文本，一个"字符"可能由多个码点组成
- `TextFragment` 记录每个字素簇的字节偏移、显示宽度和替换字符
- `GraphemeWidth::Half / Full` 正确处理中文等全角字符的显示宽度

#### 三种索引体系
- **`ByteIdx`**：字节偏移（用于字符串内部操作和注解定位）
- **`GraphemeIdx`**：字素簇索引（用于光标定位和编辑操作）
- **`ColIdx`**：列索引（用于屏幕渲染，考虑全角字符宽度）
- `Line` 提供了 `byte_idx_to_grapheme_idx` 和 `grapheme_idx_to_byte_idx` 在不同索引间转换

#### 特殊字符处理
- Tab 显示为空格、控制字符显示为 `▯`、零宽字符显示为 `.`、不可见空格显示为 `␣`
- 全角字符截断时使用 `⋯` 省略号提示

### 3. 终端 UI 架构

#### 组件化设计
- **View**（主编辑区） + **StatusBar**（状态栏） + **MessageBar**（消息栏） + **CommandBar**（命令栏）四个独立组件
- 每个组件维护自己的 `needs_redraw` 标志，实现按需重绘
- `UIComponent` trait 的 `render` 方法提供模板方法模式：统一的错误处理 + `draw` 抽象方法

#### 双缓冲渲染
- 使用 crossterm 的 `queue!` 宏批量排队渲染命令，最后通过 `flush()` 一次性输出
- 渲染顺序：先隐藏光标 → 渲染各组件 → 移动光标到正确位置 → 显示光标 → flush

#### 滚动系统
- `scroll_offset` 记录视口偏移，`text_location` 记录光标逻辑位置
- `scroll_text_location_into_view` 自动调整视口使光标可见
- `center_text_location` 搜索跳转时将光标居中

#### Location vs Position 的区分
- `Location`（逻辑坐标）：`line_idx` + `grapheme_idx`，表示文本中的位置
- `Position`（屏幕坐标）：`row` + `col`，表示终端上的渲染位置
- `text_location_to_position` 完成从逻辑坐标到屏幕坐标的转换（考虑全角字符宽度）
- `caret_position` 减去 `scroll_offset` 得到最终的光标屏幕位置

### 4. 搜索系统

#### 增量搜索
- 用户每输入一个字符就触发搜索更新，实时显示匹配结果
- `SearchInfo` 保存搜索前的光标位置和滚动偏移，Esc 取消时可恢复

#### 循环搜索
- `search_forward` 和 `search_backward` 使用 `Iterator::cycle()` 实现环绕搜索
- 搜索到文件末尾后自动从头继续

#### 字素簇安全搜索
- `find_all` 先做字节级匹配，再用 `match_graphme_clusters` 验证是否与字素簇边界对齐
- 防止匹配到多字节字符的中间位置

### 5. 语法高亮系统

#### 策略模式
- `create_syntax_highlighter` 工厂函数根据文件类型创建对应的高亮器
- 新增语言支持只需实现 `SyntaxHighlighter` trait 并注册到工厂函数

#### Rust 高亮器实现
- **单词级标记**：关键字、类型、已知值通过 `unicode-segmentation` 的 `split_word_bounds` 分割后查表匹配
- **跨行状态**：`ml_comment_balance` 追踪嵌套多行注释深度，`in_ml_string` 追踪跨行字符串
- **数字识别**：支持整数、浮点、十六进制（`0x`）、八进制（`0o`）、二进制（`0b`）字面量
- **字符和生命周期**：区分 `'a'`（字符字面量）和 `'a`（生命周期标识符）

#### AnnotatedString 的注解偏移维护
- `replace` 方法在截断或替换文本时自动调整所有注解的起止位置
- 使用 `saturating_add/sub` 防止溢出
- 失效的注解（起始 >= 结束或超出字符串长度）自动被 `retain` 过滤

### 6. 设计模式

| 模式 | 应用 |
|------|------|
| **策略模式** | `SyntaxHighlighter` trait + 多个实现 |
| **组合模式** | `Highlighter` 组合语法高亮器和搜索高亮器 |
| **模板方法** | `UIComponent::render` 定义流程，`draw` 由子类实现 |
| **状态机** | `PromptType` 驱动编辑器在不同模式间切换 |
| **命令模式** | `Command` 枚举将键盘事件解耦为独立命令对象 |
| **工厂方法** | `create_syntax_highlighter` 根据类型创建高亮器 |

### 7. 防御式编程

- 大量使用 `debug_assert!` 在开发时捕获逻辑错误，发布版本零开销
- `saturating_add/sub` 代替直接算术运算，防止溢出 panic
- `#[cfg(debug_assertions)]` 区分调试/发布版本的错误处理策略
- 启用严格的 clippy lint：`clippy::pedantic`、`clippy::arithmetic_side_effects`、`clippy::as_conversions`

### 8. 终端编程

- **Raw Mode**：禁用终端的行缓冲和回显，逐键读取输入
- **Alternate Screen**：使用备用屏幕缓冲区，退出时恢复原始内容
- **Panic Hook**：自定义 panic 处理，确保崩溃时恢复终端状态（关闭 raw mode、离开备用屏幕）
- **crossterm 库**：跨平台终端操作的 Rust 绑定，支持 Windows、macOS、Linux

### 9. 依赖库

| 库 | 用途 |
|----|------|
| `crossterm` | 跨平台终端操作（输入读取、光标控制、颜色输出） |
| `unicode-segmentation` | Unicode 字素簇分割 |
| `unicode-width` | Unicode 字符显示宽度计算 |

---

## 构建与运行

```bash
# 构建
cargo build --release

# 运行（打开文件）
cargo run --release -- <文件名>

# 运行（空编辑器）
cargo run --release
```
