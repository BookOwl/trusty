extern crate trusty;

fn main() {
    let mut frontend = trusty::Frontend::new();
    let mut backend = trusty::Backend::new();
    let mut editor = trusty::Editor::new(&mut frontend, &mut backend);
    editor.start();
}
