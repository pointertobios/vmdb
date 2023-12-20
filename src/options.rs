use std::{collections::HashMap, sync::mpsc::Receiver};

use crossterm::event::Event;

use crate::{
    frame::{Frame, FrameComp},
    gdb::Gdb,
};

#[derive(PartialEq)]
pub enum State {
    Stopping,
    WaitingForGdb,
}

pub struct Options {
    frame: Frame,
    state: State,

    receiver: Receiver<OptionsGdbInterface>,

    hint: String,
}

impl Options {
    pub fn new(x: u16, y: u16, w: u16, h: u16, receiver: Receiver<OptionsGdbInterface>) -> Self {
        Self {
            frame: Frame::new("Options".to_string(), x, y, w, h),
            state: State::Stopping,
            receiver,
            hint: String::new(),
        }
    }

    pub fn get_receiver(&mut self) -> &mut Receiver<OptionsGdbInterface> {
        &mut self.receiver
    }

    pub fn click(&mut self, x: u16, y: u16, gdb: &mut Gdb) {
        self.hint.clear();
        let x = x - self.frame.get_x() - 2;
        let y = y - self.frame.get_y() - 1;
        if y == 0 {
            let but = x / 10;
            match but {
                0 => {
                    if self.state == State::Stopping {
                        gdb.gdbcontinue();
                        self.state = State::WaitingForGdb;
                        self.hint += "Continuing";
                    } else if self.state == State::WaitingForGdb {
                        gdb.stop();
                        self.state = State::Stopping;
                    }
                }
                1 => {
                    if self.state == State::Stopping {
                        gdb.reset();
                        self.state = State::WaitingForGdb;
                        self.hint += "System reset";
                    }
                }
                2 => {
                    if self.state == State::Stopping {
                        gdb.stepi();
                    }
                }
                3 => {
                    if self.state == State::Stopping {
                        gdb.nexti();
                    }
                }
                _ => (),
            }
        }
    }

    pub fn hit_breakpoint(&mut self, bp: usize, bp_table: &HashMap<usize, u64>) {
        self.state = State::Stopping;
        self.hint.clear();
        self.hint += &format!("Bp {}, 0x{:016x}", bp, bp_table.get(&bp).unwrap());
    }

    pub fn height() -> u16 {
        5
    }

    pub fn min_width() -> u16 {
        44
    }
}

impl FrameComp for Options {
    fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    fn print(&mut self, _gdb: &Gdb) {
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
            format!(
                "[{}][{}][{}][{}]",
                if self.state == State::Stopping {
                    "Continue"
                } else {
                    "  Stop  "
                },
                if self.state == State::Stopping {
                    "  Reset "
                } else {
                    "        "
                },
                if self.state == State::Stopping {
                    "  Step  "
                } else {
                    "        "
                },
                if self.state == State::Stopping {
                    "  Next  "
                } else {
                    "        "
                }
            ),
            scmem,
            self.hint.clone(),
        ]);
    }

    fn scroll_down(&mut self) {}

    fn scroll_up(&mut self) {}
}

pub enum OptionsGdbInterface {
    Event(Event),
    HitBreakpoint(usize), // 断点（断点的编号（从1开始））
}
