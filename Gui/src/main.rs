use fltk::{enums::Color, prelude::*, *};
use vlc::*;
use std::thread;

#[derive(Copy, Clone)]
pub enum Message {
    Play,
    Stop,
}
fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");

    // Create inner window to act as embedded media player
        let mut vlc_win = window::Window::new(100, 100, 800, 600, "");
    vlc_win.end();
    vlc_win.set_color(Color::Black);

    win.end();
    win.show();
    win.make_resizable(true);

    // Take in same args as vlc
    let args: Vec<String> = std::env::args().collect();

    // Instantiate vlc instance and media player
    let instance = Instance::with_args(Some(args)).unwrap();
    let md = Media::new_path(&instance, "../Images/Rick Astley - Never Gonna Give You Up (Official Music Video)-dQw4w9WgXcQ.mkv").unwrap();
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Get vlc_win handle that we'll pass to libvlc
    // Linux u32, windows HWND, Mac NSWindow
    let handle = vlc_win.raw_handle();

    // Pass the handle to vlc
    // Method depends on the platform
    // For Linux
    #[cfg(target_os = "linux")]
        mdp.set_xwindow(handle as u32);
    // For Windows
    #[cfg(target_os = "windows")]
        mdp.set_hwnd(handle);
    // For MacOS
    #[cfg(target_os = "macos")]
        mdp.set_nsobject(handle);

    // Disable event handling on vlc's side
    // Do it thru fltk
    mdp.set_key_input(false);
    mdp.set_mouse_input(false);
    mdp.play().unwrap();
    thread::sleep(::std::time::Duration::from_secs(10));

}
