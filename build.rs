#[cfg(windows)]
use winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    //res.set_icon("test.ico");
    res.compile().unwrap();
}