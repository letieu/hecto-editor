use editor::Editor;

mod editor;
mod terminal;
mod document;
mod row;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
