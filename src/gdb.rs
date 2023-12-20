use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStderr, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, RwLock};
use std::thread;

use crate::options::OptionsGdbInterface;

pub struct Gdb {
    proc: Child,
    input: ChildStdin,
    _error: ChildStderr,

    regs: Registers,

    sender: SyncSender<OptionsGdbInterface>,
}

impl Gdb {
    pub fn new(
        hostname: &str,
        port: u16,
    ) -> (
        Arc<RwLock<Self>>,
        Receiver<OptionsGdbInterface>,
        Receiver<String>,
    ) {
        let mut proc = Command::new("gdb")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = proc.stdin.take().unwrap();
        let output = proc.stdout.take().unwrap();
        let mut output = BufReader::new(output);
        let error = proc.stderr.take().unwrap();
        let mut t = String::new();
        for _ in 0..15 {
            output.read_line(&mut t).unwrap();
            println!("{}", t);
        }
        writeln!(input, "target remote {}:{}", hostname, port).unwrap();
        output.read_line(&mut t).unwrap();
        for _ in 0..4 {
            println!("{}", t);
        }
        let (sender, receiver) = mpsc::sync_channel(8);
        let (bridge_sender, bridge_receiver) = mpsc::channel();
        thread::spawn(move || loop {
            let mut t = String::new();
            output.read_line(&mut t).unwrap();
            bridge_sender.send(t).unwrap();
        });
        let gdb = Self {
            proc,
            input,
            _error: error,
            regs: Registers::new(),
            sender,
        };
        let gdb = Arc::new(RwLock::new(gdb));
        (gdb, receiver, bridge_receiver)
    }

    pub fn thr_gdb_sender(&mut self, bridg_receiver: &Receiver<String>) {
        let s = bridg_receiver.try_recv();
        if let Ok(s) = s {
            if s.starts_with("Breakpoint") {
                let s: Vec<&str> = s.split(',').collect::<Vec<&str>>()[0].split(' ').collect();
                let bp: usize = s[1].parse().unwrap();
                self.sender
                    .send(OptionsGdbInterface::HitBreakpoint(bp))
                    .unwrap();
            }
        }
    }

    pub fn get_sender(&mut self) -> &mut SyncSender<OptionsGdbInterface> {
        &mut self.sender
    }

    pub fn gdbcontinue(&mut self) {
        writeln!(self.input, "continue").unwrap();
    }

    pub fn stop(&mut self) {
        kill(Pid::from_raw(self.proc.id() as i32), Signal::SIGINT).unwrap();
    }

    pub fn reset(&mut self) {
        writeln!(self.input, "monitor system_reset").unwrap();
        self.gdbcontinue();
    }

    pub fn stepi(&mut self) {
        writeln!(self.input, "stepi").unwrap();
    }

    pub fn nexti(&mut self) {
        writeln!(self.input, "nexti").unwrap();
    }

    pub fn get_registers(&self) -> &Registers {
        &self.regs
    }
}

impl Drop for Gdb {
    fn drop(&mut self) {
        self.input.write(&[4]).unwrap();
        writeln!(self.input, "y").unwrap();
        self.proc.kill().unwrap();
    }
}

pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            rsp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rip: 0,
            rflags: 0,
            cr0: 0,
            cr2: 0,
            cr3: 0,
            cr4: 0,
            cr8: 0,
            efer: 0,
        }
    }
}
