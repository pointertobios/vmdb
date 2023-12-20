use crate::{
    frame::{Frame, FrameComp},
    gdb::Gdb,
};

pub struct Memory {
    frame: Frame,
}

impl Memory {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            frame: Frame::new("Memory".to_string(), x, y, w, h),
        }
    }

    pub fn width() -> u16 {
        58
    }
}

impl FrameComp for Memory {
    fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    fn print(&mut self, _gdb: &Gdb) {
        self.frame.print(&mut vec![]);
    }

    fn scroll_down(&mut self) {
        todo!()
    }

    fn scroll_up(&mut self) {
        todo!()
    }
}
