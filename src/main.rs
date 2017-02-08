extern crate trusty;
use std::env;

fn main() {
    let files = env::args().skip(1).collect();
    let mut frontend = trusty::Frontend::new();
    let mut backend = trusty::Backend::new(files);
    let mut editor = trusty::Editor::new(&mut frontend, &mut backend);
    editor.start();
}
