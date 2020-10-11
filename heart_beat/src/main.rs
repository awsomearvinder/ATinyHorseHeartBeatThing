use winapi_wrapper::window;
fn main() {
    let window = window::Window::new("Heart Beat", 500, 500, 50, 50).expect("Failed to make menu");
    window
        .set_level(window::Level::TopMost)
        .expect("Couldn't set to top level");
    window.set_window_color(winapi_wrapper::util::RGBA(0xff,0xee,0xdd,255)).expect("Couldn't set color");
    window.remove_window_styling().expect("Failed to reset window styling.");
    //Just while we're developing
    std::thread::sleep_ms(5000);
}
