use crate::frame::Frame;

pub enum State {
    Stopping,
    WaitingForGdb,
}

pub struct Options {
    frame: Frame,
    state: State,
}

impl Options {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            frame: Frame::new("Options".to_string(), x, y, w, h),
            state: State::WaitingForGdb,
        }
    }

    pub fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    pub fn print(&mut self) {
        let mut scmem = "Search Memory: [".to_string();
        for _ in 0..(if self.frame.get_width() > 21 {
            self.frame.get_width() - 21
        } else {
            0
        }) {
            scmem += " ";
        }
        scmem += "]";
        self.frame.print(&mut vec![
            "[Continue][  Stop  ][  Reset ][  Step  ][  Next  ]".to_string(),
            scmem,
        ]);
    }

    pub fn click(&mut self, x: u16, y: u16) {
        let x = x - self.frame.get_x();
        let y = y - self.frame.get_y();
    }

    pub fn height() -> u16 {
        4
    }
}
