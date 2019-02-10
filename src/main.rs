extern crate ncurses;

use std::char;
use ncurses::*;

mod channel;
mod input;

fn main() {
    do_stuff()
}

fn do_stuff() {
    let mut inp = input::Input::new();
    let mut chan = channel::Channel::new();
    /* Setup ncurses. */
    let window = initscr();
    raw();
    keypad(stdscr(), true);
    noecho();

    wmove(window, LINES() - 2, 0);
    whline(window, ACS_HLINE(), COLS());
    mvprintw(LINES() - 1, 0, "> ");
    refresh();

    let mut ch = getch();
    while ch != 27 {
        // -- DEBUG --
        wmove(window, LINES() - 2, 2);
        wprintw(window, format!("[{: >width$}]", ch, width=4).as_str());
        // -- DEBUG --

        let unboxed_char = char::from_u32(ch as u32).expect("Invalid char");
        handle_control(unboxed_char, &mut inp, &mut chan);

        draw(window, &mut inp, &mut chan);

        ch = getch();
    }

    endwin();
}

fn resize(inp: &mut input::Input, chan: &mut channel::Channel) {
    inp.resize(COLS() - 2);
    chan.resize(LINES() - 2);
}

fn handle_control(ch: char, inp: &mut input::Input, chan: &mut channel::Channel) {
    if inp.handle_control_character(ch) {
        return;
    } else if u32::from(ch) == 10 {
        chan.push_message(inp.pop_value());
    } else {
        inp.write(ch);
    }
}

fn draw(window: WINDOW, inp: &mut input::Input, chan: &mut channel::Channel) {
    resize(inp, chan);

    // write channel
    if chan.is_dirty() {
        let mut line = 0;
        let page = chan.view_page();

        for stmt in page {
            wmove(window, line, 0);
            wclrtoeol(window);
            wmove(window, line, 0);
            wprintw(window, stmt.as_str());
            line += 1;
        }

        while line < LINES() - 2 {
            wmove(window, line, 0);
            wclrtoeol(window);
            line += 1;
        }

        chan.clean();
    }

    // write input
    if inp.is_dirty() {
        wmove(window, LINES() - 1, 0);
        wclrtoeol(window);
        wmove(window, LINES() - 1, 0);
        wprintw(window, "> ");
        let s = inp.visible_chars();
        wprintw(window, s.as_str());
        inp.clean();
    }

    // move cursor
    wmove(window, LINES() - 1, inp.cursor_pos() + 2);
}
