extern crate ncurses;

use std::char;
use ncurses::*;

mod channel;
mod input;
mod window;

fn main() {
    do_stuff()
}

fn do_stuff() {
    let inp = input::Input::new();
    let chan = channel::Channel::new();
    let mut window = window::Window::new(inp, chan);

    window.draw();

    let mut ch = getch();
    while ch != 27 {
        let unboxed_char = char::from_u32(ch as u32).expect("Invalid char");
        handle_control(unboxed_char, &mut window);

        window.draw();

        ch = getch();
    }

    window.finalize();
}

fn handle_control(ch: char, window: &mut window::Window) {
    if window.input().handle_control_character(ch) {
        return;
    } else if u32::from(ch) == 10 {
        let val = window.input().pop_value();
        window.channel().push_message(val);
    } else {
        window.input().write(ch);
    }
}
