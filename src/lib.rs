include!(concat!(env!("OUT_DIR"), "/wrap.rs"));

pub fn baz() {
    let _ = foo::bar::Bar;
}
