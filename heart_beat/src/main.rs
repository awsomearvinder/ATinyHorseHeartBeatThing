use winapi_wrapper::window;
fn main() {
    let window = window::Window::new("Heart Beat", 500, 500, 50, 50);
    window.set_level(window::Level::TopMost);
    //Just while we're developing
    std::thread::sleep_ms(5000);
}