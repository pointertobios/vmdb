use std::{
    io::Write,
    process::exit,
    sync::{mpsc::SyncSender, Arc},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyModifiers, MouseButton,
        MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use disass::Disassembly;
use frame::FrameComp;
use gdb::Gdb;
use memory::Memory;
use options::{Options, OptionsGdbInterface};
use register::Register;
use srccode::SrcCode;

pub mod disass;
pub mod frame;
pub mod gdb;
pub mod memory;
pub mod options;
pub mod register;
pub mod srccode;

pub fn run(config: Config) {
    let (gdb, option_receiver, bridge_receiver) = Gdb::new(&config.host, config.port);
    let gdb_clone = Arc::clone(&gdb);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(2));
        {
            gdb_clone.write().unwrap().thr_gdb_sender(&bridge_receiver);
        }
    });
    enable_raw_mode().unwrap();
    execute!(std::io::stdout(), EnableMouseCapture, Hide).unwrap();
    let (mut width, mut height) = terminal::size().unwrap();
    let mut reg = Register::new(0, 0, Register::width(), height);

    let resw = width as i16 - Register::width() as i16 - Memory::width() as i16;
    let resw = if resw < 0 { 0 } else { resw } as u16;

    let mut disas = Disassembly::new(
        &config.kernel_elf,
        Register::width(),
        Options::height(),
        if resw > Options::min_width() {
            Disassembly::max_width()
                .min(resw / 2)
                .max(Options::min_width())
        } else {
            resw
        },
        height - Options::height(),
    );

    let mut opt = Options::new(
        Register::width(),
        0,
        if resw > Options::min_width() {
            Disassembly::max_width()
                .min(resw / 2)
                .max(Options::min_width())
        } else {
            resw
        },
        Options::height(),
        option_receiver,
    );

    let scw = if resw / 2 < Disassembly::max_width() {
        resw / 2
    } else {
        resw / 2 * 2 - Disassembly::max_width()
    } as i16
        - if resw / 2 < Options::min_width() {
            Options::min_width() - resw / 2
        } else {
            0
        } as i16
        + resw as i16 % 2;
    let mut scode = SrcCode::new(
        Register::width()
            + Disassembly::max_width()
                .min(resw / 2)
                .max(Options::min_width()),
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

    let eve_disp_sender = { SyncSender::clone(gdb.write().unwrap().get_sender()) };
    thread::spawn(move || loop {
        eve_disp_sender
            .send(OptionsGdbInterface::Event(event::read().unwrap()))
            .unwrap();
    });

    loop {
        {
            reg.print(&gdb.read().unwrap());
            opt.print(&gdb.read().unwrap());
            disas.print(&gdb.read().unwrap());
            scode.print(&gdb.read().unwrap());
            mem.print(&gdb.read().unwrap());
        }
        thread::sleep(Duration::from_millis(5));
        let event = opt.get_receiver().try_recv();
        if let Err(_) = event {
            continue;
        }
        let event = event.unwrap();
        if let OptionsGdbInterface::Event(event) = event {
            match event {
                event::Event::FocusGained => (),
                event::Event::FocusLost => (),
                event::Event::Key(eve) => {
                    let KeyEvent {
                        code,
                        modifiers,
                        kind: _,
                        state: _,
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
                                drop(gdb);
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
                        modifiers: _,
                    } = eve;
                    if reg.get_frame().in_frame(column, row) {
                        if kind == MouseEventKind::ScrollDown {
                            reg.scroll_down();
                        } else if kind == MouseEventKind::ScrollUp {
                            reg.scroll_up();
                        }
                    } else if opt.get_frame().in_frame(column, row) {
                        if kind == MouseEventKind::Down(MouseButton::Left) {
                            opt.click(column, row, &mut gdb.write().unwrap());
                        }
                    } else if disas.get_frame().in_frame(column, row) {
                        if kind == MouseEventKind::ScrollDown {
                            disas.scroll_down();
                        } else if kind == MouseEventKind::ScrollUp {
                            disas.scroll_up();
                        }
                    }
                }
                event::Event::Paste(_) => (),
                event::Event::Resize(column, row) => {
                    width = column;
                    let resw = width as i16 - Register::width() as i16 - Memory::width() as i16;
                    let resw = if resw < 0 { 0 } else { resw } as u16;
                    disas.get_frame().set_width(if resw > Options::min_width() {
                        Disassembly::max_width()
                            .min(resw / 2)
                            .max(Options::min_width())
                    } else {
                        resw
                    });
                    opt.get_frame().set_width(if resw > Options::min_width() {
                        Disassembly::max_width()
                            .min(resw / 2)
                            .max(Options::min_width())
                    } else {
                        resw
                    });
                    let scw = if resw / 2 < Disassembly::max_width() {
                        resw / 2
                    } else {
                        resw / 2 * 2 - Disassembly::max_width()
                    } as i16
                        - if resw / 2 < Options::min_width() {
                            Options::min_width() - resw / 2
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
                                .max(Options::min_width()),
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
        } else if let OptionsGdbInterface::HitBreakpoint(bp) = event {
            opt.hit_breakpoint(bp, disas.get_breakpoints());
            disas.set_rip(gdb.read().unwrap().get_registers().rip);
        }
    }
}

pub struct Config {
    pub host: String,
    pub port: u16,

    pub kernel_elf: String,
}
