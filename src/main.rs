use editor::Editor;

mod editor;
mod terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
