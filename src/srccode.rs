use crate::{
    frame::{Frame, FrameComp},
    gdb::Gdb,
};

pub struct SrcCode {
    frame: Frame,
}

impl SrcCode {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            frame: Frame::new("SrcCode".to_string(), x, y, w, h),
        }
    }
}

impl FrameComp for SrcCode {
    fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    fn print(&mut self, _gdb: &Gdb) {
        self.frame.print(&mut vec!["".to_string()]);
    }

    fn scroll_down(&mut self) {}

    fn scroll_up(&mut self) {}
}
