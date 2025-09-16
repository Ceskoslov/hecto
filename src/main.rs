#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;
use editor::Editor;

fn main(){
    let mut editor = Editor::default();
    editor.run();
}