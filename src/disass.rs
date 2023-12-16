use crate::frame::Frame;

pub struct Disassembly {
    frame: Frame,
}

impl Disassembly {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            frame: Frame::new("Disassembly".to_string(), x, y, w, h),
        }
    }

    pub fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    pub fn print(&mut self) {
        self.frame.print(&mut vec!["".to_string()]);
    }

    pub fn max_width() -> u16 {
        76
    }

    pub fn min_width() -> u16 {
        54
    }
}
