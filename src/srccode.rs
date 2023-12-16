use crate::frame::Frame;

pub struct SrcCode {
    frame: Frame,
}

impl SrcCode {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            frame: Frame::new("SrcCode".to_string(), x, y, w, h),
        }
    }

    pub fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    pub fn print(&mut self) {
        self.frame.print(&mut vec!["".to_string()]);
    }
}
