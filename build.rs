use std::env;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
struct SerdeCallbacks;

impl bindgen::callbacks::ParseCallbacks for SerdeCallbacks {
    fn add_derives(&self, _info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        vec!["Serialize".into(), "Deserialize".into()]
    }
}

fn main() {
    let bindings = bindgen::Builder::default()
        .header("src/ptrace.h")
        .raw_line("use serde::{Serialize, Deserialize};")
        .parse_callbacks(Box::new(SerdeCallbacks))
        .allowlist_function("ptrace_.*")
        .allowlist_type("ptrace_.*")
        .allowlist_var("ptrace_.*")
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::current_dir().unwrap()).join("src/ptrace_c.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/ptrace.c")
        .compile("trivial_strace_c.a");
}
