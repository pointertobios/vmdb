use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::{
    frame::{Frame, FrameComp},
    gdb::Gdb,
};

pub struct Disassembly {
    frame: Frame,
    disass: Vec<String>,
    rip: u64,
    scroll: isize,
    bp_table: HashMap<usize, u64>,
}

impl Disassembly {
    pub fn new(elf: &str, x: u16, y: u16, w: u16, h: u16) -> Self {
        let mut objdump = Command::new("objdump")
            .arg("-D")
            .arg(&format!("{}", elf))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let chout = objdump.stdout.take().unwrap();
        let chout = BufReader::new(chout);
        let mut disass = Vec::new();
        for line in chout.lines() {
            if let Ok(line) = line {
                disass.push(line);
            } else {
                break;
            }
        }
        Self {
            frame: Frame::new("Disassembly".to_string(), x, y, w, h),
            disass,
            rip: 0,
            scroll: 0,
            bp_table: HashMap::new(),
        }
    }

    pub fn get_breakpoints(&self) -> &HashMap<usize, u64> {
        &self.bp_table
    }

    pub fn set_rip(&mut self, rip: u64) {
        self.rip = rip;
        self.scroll = 0;
    }

    pub fn max_width() -> u16 {
        76
    }
}

impl FrameComp for Disassembly {
    fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    fn print(&mut self, _gdb: &Gdb) {
        let mut ind = 0usize;
        while !self.disass[ind].split(":").collect::<Vec<&str>>()[0]
            .ends_with(&format!("{:x}", self.rip))
        {
            ind += 1;
        }
        ind -= 1;
        let start = ind as isize + self.scroll - 5;
        let ustart = if start >= 0 { start as usize } else { 0 };
        let end = start + self.frame.get_height() as isize - 2;
        let end = end as usize;
        let mut printed = self.disass[ustart..end].to_vec();
        if start < 0 {
            for _ in 0..((-start) as usize) {
                printed.insert(0, "".to_string());
            }
        }
        self.frame.print(&mut printed);
    }

    fn scroll_down(&mut self) {
        self.scroll += 1;
    }

    fn scroll_up(&mut self) {
        self.scroll -= 1;
    }
}
