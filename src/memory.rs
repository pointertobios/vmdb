use crate::frame::Frame;

pub struct Memory {
    frame: Frame,
}

impl Memory {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            frame: Frame::new("Memory".to_string(), x, y, w, h),
        }
    }

    pub fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    pub fn print(&mut self, width: u16) {
        self.frame.print(&mut vec![]);
    }

    pub fn width() -> u16 {
        58
    }
}
