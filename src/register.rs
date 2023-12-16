use crate::{frame::Frame, gdb::Gdb};

pub struct Register {
    frame: Frame,
}

impl Register {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            frame: Frame::new("Register".to_string(), x, y, width, height),
        }
    }

    pub fn print(&mut self, gdb: &Gdb) {
        let mut cont = self.get_content(gdb);
        self.frame.print(&mut cont);
    }

    pub fn get_frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    fn get_content(&mut self, gdb: &Gdb) -> Vec<String> {
        let mut res = vec![];
        res.push("rax".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rax));
        res.push("rbx".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rbx));
        res.push("rcx".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rcx));
        res.push("rdx".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rdx));
        res.push("rsi".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rsi));
        res.push("rdi".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rdi));
        res.push("rbp".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rbp));
        res.push("rsp".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rsp));
        res.push("r8".to_string());
        res.push(format!("{:016x}", gdb.get_registers().r8));
        res.push("r9".to_string());
        res.push(format!("{:016x}", gdb.get_registers().r9));
        res.push("r10".to_string());
        res.push(format!("{:016x}", gdb.get_registers().r10));
        res.push("r11".to_string());
        res.push(format!("{:016x}", gdb.get_registers().r11));
        res.push("r12".to_string());
        res.push(format!("{:016x}", gdb.get_registers().r12));
        res.push("r13".to_string());
        res.push(format!("{:016x}", gdb.get_registers().r13));
        res.push("r14".to_string());
        res.push(format!("{:016x}", gdb.get_registers().r14));
        res.push("r15".to_string());
        res.push(format!("{:016x}", gdb.get_registers().r15));
        res.push("rip".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rip));
        res.push("rflags".to_string());
        res.push(format!("{:016x}", gdb.get_registers().rflags));
        res.push("cr0".to_string());
        res.push(format!("{:016x}", gdb.get_registers().cr0));
        res.push("cr2".to_string());
        res.push(format!("{:016x}", gdb.get_registers().cr2));
        res.push("cr3".to_string());
        res.push(format!("{:016x}", gdb.get_registers().cr3));
        res.push("cr4".to_string());
        res.push(format!("{:016x}", gdb.get_registers().cr4));
        res.push("cr8".to_string());
        res.push(format!("{:016x}", gdb.get_registers().cr8));
        res.push("efer".to_string());
        res.push(format!("{:016x}", gdb.get_registers().efer));
        res
    }

    pub fn width() -> u16 {
        19
    }
}
