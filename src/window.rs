use input;
use channel;
use ncurses::*;

pub struct Window {
    input: input::Input,
    channel: channel::Channel,
    window: WINDOW,
    width: i32,
    height: i32,
    dirty: bool,
}

impl Window {
    pub fn new(input: input::Input, channel: channel::Channel) -> Window {
        /* Setup ncurses. */
        let window = initscr();
        raw();
        keypad(stdscr(), true);
        noecho();

        Window {
            input,
            channel,
            window,
            width: 0,
            height: 0,
            dirty: true,
        }
    }

    pub fn draw(&mut self) {
        self.resize();

        if self.dirty {
            let mut line : i32 = 0;
            while line < self.height {
                wmove(self.window, line, 0);
                wclrtoeol(self.window);
                line += 1
            }
        }

        // write channel
        if self.channel.is_dirty() || self.dirty {
            let mut line = 0;
            let page = self.channel.view_page();

            for stmt in page {
                wmove(self.window, line, 0);
                wclrtoeol(self.window);
                wmove(self.window, line, 0);
                wprintw(self.window, stmt.as_str());
                line += 1;
            }

            self.channel.clean();
        }

        if self.dirty {
            wmove(self.window, LINES() - 2, 0);
            whline(self.window, ACS_HLINE(), COLS());
        }

        // write input
        if self.input.is_dirty() || self.dirty {
            wmove(self.window, LINES() - 1, 0);
            wclrtoeol(self.window);
            wmove(self.window, LINES() - 1, 0);
            wprintw(self.window, "> ");
            let s = self.input.visible_chars();
            wprintw(self.window, s.as_str());
            self.input.clean();
        }

        self.dirty = false;

        // move cursor
        wmove(self.window, LINES() - 1, self.input.cursor_pos() + 2);
        wrefresh(self.window);
    }

    pub fn input(&mut self) -> &mut input::Input {
        return &mut self.input;
    }

    pub fn channel(&mut self) -> &mut channel::Channel {
        return &mut self.channel;
    }

    pub fn finalize(&mut self) {
        endwin();
    }

    fn resize(&mut self) {
        let width = COLS();
        let height = LINES();

        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.dirty = true;

            self.input.resize(width - 2);
            self.channel.resize(height - 2);
        }
    }
}
