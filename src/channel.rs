const MAX_MESSAGES: usize = 4096;

pub struct Channel {
    messages: Vec<String>,
    dirty: bool,
    index: usize,
    page_size: i32,
}

impl Channel {

    pub fn new() -> Channel {
        let messages = Vec::new();
        let dirty = true;
        let index = 0;
        let page_size = 0;

        Channel {
            messages,
            dirty,
            index,
            page_size,
        }
    }

    pub fn push_message(&mut self, s: String) {
        self.messages.push(s);

        if self.messages.len() == self.index {
            self.index = self.index + 1;
        }

        while self.messages.len() > MAX_MESSAGES {
            self.messages.remove(0);
            self.index = self.index - 1;
        }
        self.dirty = true;
        self.fix_index();
    }

    pub fn view_page(&self) -> Vec<String> {
        let start = std::cmp::max(0, self.index as i32 - self.page_size) as usize;
        self.messages[start..self.index].to_vec()
    }

    pub fn resize(&mut self, page_size: i32) {
        if self.page_size != page_size {
            self.page_size = page_size;
            self.dirty;
            self.fix_index();
        }
    }

    pub fn scroll(&mut self, scroll_amt: usize) {
        self.index += scroll_amt;
        self.fix_index();
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clean(&mut self) {
        self.dirty = false;
    }

    fn fix_index(&mut self) {
        if (self.index as i32) < self.page_size {
            self.index = self.page_size as usize;
        }

        if self.index > self.messages.len()  {
            self.index = self.messages.len();
        }
    }
}
