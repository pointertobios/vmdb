use std::io::{Read, Write};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};

pub struct Gdb {
    proc: Child,
    input: ChildStdin,
    output: ChildStdout,
    error: ChildStderr,

    regs: Registers,
}

impl Gdb {
    pub fn new(hostname: &str, port: u16) -> Self {
        let mut proc = Command::new("gdb")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = proc.stdin.take().unwrap();
        let output = proc.stdout.take().unwrap();
        let error = proc.stderr.take().unwrap();
        writeln!(input, "target remote {}:{}", hostname, port).unwrap();
        Self {
            proc,
            input,
            output,
            error,
            regs: Registers::new(),
        }
    }

    pub fn get_registers(&self) -> &Registers {
        &self.regs
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
