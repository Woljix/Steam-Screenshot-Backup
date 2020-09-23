#[cfg(windows)]
use winres;

#[cfg(windows)]
fn main() {
    let res = winres::WindowsResource::new();
    //res.set_icon("test.ico"); // needs to be mutable for this to work.
    res.compile().unwrap();
}