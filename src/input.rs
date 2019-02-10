pub struct Input {
    characters: Vec<char>,
    index: usize,
    visual_index: i32,
    width: i32,
    dirty: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            characters: Vec::new(),
            index: 0,
            visual_index: 0,
            width: 0,
            dirty: false
        }
    }

    pub fn visible_chars(&self) -> String {
        let start = self.visual_index as usize;
        let end = std::cmp::min(self.characters.len(), (self.visual_index + self.width) as usize);
        self.characters[start..end].iter().collect::<String>()
    }

    pub fn resize(&mut self, width: i32) {
        if self.width != width {
            self.width = width;
            self.check_visible_bounds();
            self.dirty = true;
        }
    }

    pub fn write(&mut self, ch: char) {
        self.characters.insert(self.index, ch);
        self.dirty = true;
        self.move_index(1);
    }

    pub fn handle_control_character(&mut self, ch: char) -> bool {
        match u32::from(ch) {
            263 => self.backspace(),
            330 => self.delete(),
            360 => self.end(),
            262 => self.home(),
            260 => self.move_index(-1),
            261 => self.move_index(1),
            410 => return true,
            545 => return true,
            _ => return false,
        }
        self.dirty = true;

        true
    }

    pub fn pop_value(&mut self) -> String {
        let out = self.characters.iter().collect::<String>();
        self.characters = Vec::new();
        self.index = 0;
        self.visual_index = 0;
        self.dirty = true;
        return out;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clean(&mut self) {
        self.dirty = false
    }

    pub fn cursor_pos(&self) -> i32 {
        (self.index as i32 - self.visual_index) as i32
    }

    fn home(&mut self) {
        let movement = -(self.index as i32);
        self.move_index(movement);
    }

    fn end(&mut self) {
        let movement = self.characters.len() as i32 - self.index as i32;
        self.move_index(movement);
    }

    fn delete(&mut self) {
        if self.index < self.characters.len() {
            self.characters.remove(self.index);
        }
    }

    fn backspace(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.characters.remove(self.index);
        }
    }

    fn move_index(&mut self, scroll: i32) {
        if (-scroll as i64) > self.index as i64 {
            self.index = 0;
        } else {
            self.index = ((self.index as i64) + (scroll as i64)) as usize;
        }

        if self.index > self.characters.len() {
            self.index = self.characters.len()
        }

        self.check_visible_bounds();
    }

    fn check_visible_bounds(&mut self) {
        if self.cursor_pos() < 0 {
            let half_width = self.width / 2;
            self.move_visible_cursor(-half_width);
        } else if self.cursor_pos() >= self.width {
            let half_width = self.width / 2;
            self.move_visible_cursor(half_width);
        }
    }

    fn move_visible_cursor(&mut self, scroll: i32) {
        self.visual_index += scroll;

        if self.visual_index > self.characters.len() as i32 {
            self.visual_index = self.characters.len() as i32
        } else if self.visual_index < 0 {
            self.visual_index = 0
        }
    }
}