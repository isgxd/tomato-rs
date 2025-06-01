fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/clock.ico");
    res.compile().unwrap();
}
