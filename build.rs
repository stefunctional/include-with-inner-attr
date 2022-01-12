use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    fs::write(out_dir.join("wrap.rs"), MY_WRAP).unwrap();
    let out_dir = out_dir.join("foo");
    fs::create_dir_all(&out_dir).unwrap();
    fs::write(out_dir.join("mod.rs"), MY_MOD).unwrap();
    fs::write(out_dir.join("bar.rs"), MY_BAR).unwrap();
}

const MY_MOD: &str = r#"
#![allow(unused)]

pub mod bar;
"#;

const MY_BAR: &str = r#"
pub struct Bar;
"#;

const MY_WRAP: &str = r#"
    pub mod foo;
"#;
