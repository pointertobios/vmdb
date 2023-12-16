use std::{io::Write, process::exit};

use crossterm::{
    cursor::{Hide, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyModifiers, MouseEvent,
        MouseEventKind,
    },
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use disass::Disassembly;
use gdb::Gdb;
use memory::Memory;
use options::Options;
use register::Register;
use srccode::SrcCode;

pub mod disass;
pub mod frame;
pub mod gdb;
pub mod memory;
pub mod options;
pub mod register;
pub mod srccode;

pub fn run() {
    enable_raw_mode().unwrap();
    execute!(std::io::stdout(), EnableMouseCapture, Hide).unwrap();
    let gdb = Gdb::new("localhost", 1234);
    let (mut width, mut height) = terminal::size().unwrap();
    let mut reg = Register::new(0, 0, Register::width(), height);

    let resw = width as i16 - Register::width() as i16 - Memory::width() as i16;
    let resw = if resw < 0 { 0 } else { resw } as u16;

    let mut disas = Disassembly::new(
        Register::width(),
        Options::height(),
        if resw > Disassembly::min_width() {
            Disassembly::max_width()
                .min(resw / 2)
                .max(Disassembly::min_width())
        } else {
            resw
        },
        height - Options::height(),
    );

    let mut opt = Options::new(
        Register::width(),
        0,
        if resw > Disassembly::min_width() {
            Disassembly::max_width()
                .min(resw / 2)
                .max(Disassembly::min_width())
        } else {
            resw
        },
        Options::height(),
    );

    let scw = if resw / 2 < Disassembly::max_width() {
        resw / 2
    } else {
        resw / 2 * 2 - Disassembly::max_width()
    } as i16
        - if resw / 2 < Disassembly::min_width() {
            Disassembly::min_width() - resw / 2
        } else {
            0
        } as i16
        + resw as i16 % 2;
    let mut scode = SrcCode::new(
        Register::width()
            + Disassembly::max_width()
                .min(resw / 2)
                .max(Disassembly::min_width()),
        0,
        if scw < 0 { 0 } else { scw as u16 },
        height,
    );

    let mut mem = Memory::new(
        Register::width() + resw / 2 * 2 + resw % 2,
        0,
        Memory::width().min(width - Register::width()),
        height,
    );

    reg.print(&gdb);
    opt.print();
    disas.print();
    scode.print();
    mem.print(width);
    loop {
        match event::read().unwrap() {
            event::Event::FocusGained => (),
            event::Event::FocusLost => (),
            event::Event::Key(eve) => {
                let KeyEvent {
                    code,
                    modifiers,
                    kind,
                    state,
                } = eve;
                match code {
                    event::KeyCode::Backspace => (),
                    event::KeyCode::Enter => (),
                    event::KeyCode::Left => (),
                    event::KeyCode::Right => (),
                    event::KeyCode::Up => (),
                    event::KeyCode::Down => (),
                    event::KeyCode::Home => (),
                    event::KeyCode::End => (),
                    event::KeyCode::PageUp => (),
                    event::KeyCode::PageDown => (),
                    event::KeyCode::Tab => (),
                    event::KeyCode::BackTab => (),
                    event::KeyCode::Delete => (),
                    event::KeyCode::Insert => (),
                    event::KeyCode::F(_) => (),
                    event::KeyCode::Char(c) => {
                        if modifiers.contains(KeyModifiers::CONTROL) && c == 'd' {
                            execute!(std::io::stdout(), Show, DisableMouseCapture).unwrap();
                            disable_raw_mode().unwrap();
                            std::io::stdout().flush().unwrap();
                            exit(0);
                        }
                    }
                    event::KeyCode::Null => (),
                    event::KeyCode::Esc => (),
                    event::KeyCode::CapsLock => (),
                    event::KeyCode::ScrollLock => (),
                    event::KeyCode::NumLock => (),
                    event::KeyCode::PrintScreen => (),
                    event::KeyCode::Pause => (),
                    event::KeyCode::Menu => (),
                    event::KeyCode::KeypadBegin => (),
                    event::KeyCode::Media(_) => (),
                    event::KeyCode::Modifier(_) => (),
                }
            }
            event::Event::Mouse(eve) => {
                let MouseEvent {
                    kind,
                    column,
                    row,
                    modifiers,
                } = eve;
                if reg.get_frame().in_frame(column, row) {
                    if kind == MouseEventKind::ScrollDown {
                        reg.get_frame().inc_start();
                    } else if kind == MouseEventKind::ScrollUp {
                        reg.get_frame().dec_start();
                    }
                }
            }
            event::Event::Paste(_) => (),
            event::Event::Resize(column, row) => {
                width = column;
                let resw = width as i16 - Register::width() as i16 - Memory::width() as i16;
                let resw = if resw < 0 { 0 } else { resw } as u16;
                disas
                    .get_frame()
                    .set_width(if resw > Disassembly::min_width() {
                        Disassembly::max_width()
                            .min(resw / 2)
                            .max(Disassembly::min_width())
                    } else {
                        resw
                    });
                opt.get_frame()
                    .set_width(if resw > Disassembly::min_width() {
                        Disassembly::max_width()
                            .min(resw / 2)
                            .max(Disassembly::min_width())
                    } else {
                        resw
                    });
                let scw = if resw / 2 < Disassembly::max_width() {
                    resw / 2
                } else {
                    resw / 2 * 2 - Disassembly::max_width()
                } as i16
                    - if resw / 2 < Disassembly::min_width() {
                        Disassembly::min_width() - resw / 2
                    } else {
                        0
                    } as i16
                    + resw as i16 % 2;
                scode
                    .get_frame()
                    .set_width(if scw < 0 { 0 } else { scw as u16 });
                scode.get_frame().set_x(
                    Register::width()
                        + Disassembly::max_width()
                            .min(resw / 2)
                            .max(Disassembly::min_width()),
                );
                mem.get_frame()
                    .set_x(Register::width() + resw / 2 * 2 + resw % 2);
                mem.get_frame()
                    .set_width(Memory::width().min(width - Register::width()));

                height = row;
                reg.get_frame().set_height(height);
                disas.get_frame().set_height(if height as i16 - 3 >= 0 {
                    height - Options::height()
                } else {
                    0
                });
                scode.get_frame().set_height(height);
                mem.get_frame().set_height(height);
            }
        }

        reg.print(&gdb);
        opt.print();
        disas.print();
        scode.print();
        mem.print(width);
    }
}

pub struct Config {
    host: String,
    port: u16,
}
