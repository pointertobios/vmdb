use std::fmt::format;

use crossterm::{cursor::MoveTo, execute, style::Print};

pub struct Frame {
    title: String,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    start_line: u16,
    contl: u16,
}

impl Frame {
    pub fn new(title: String, x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            title,
            x,
            y,
            width: w,
            height: h,
            start_line: 0,
            contl: 0,
        }
    }

    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn in_frame(&self, x: u16, y: u16) -> bool {
        x > self.x && x < self.x + self.width && y > self.y && y < self.y + self.height
    }

    pub fn inc_start(&mut self) {
        if self.contl - 2 >= self.start_line {
            self.start_line += 1;
        }
    }

    pub fn dec_start(&mut self) {
        if self.start_line != 0 {
            self.start_line -= 1;
        }
    }

    pub fn print(&mut self, cont: &mut Vec<String>) {
        self.contl = cont.len() as u16;
        let cont = if self.start_line as usize >= cont.len() {
            vec![]
        } else {
            cont[(self.start_line as usize)..].to_vec()
        };
        for i in 0..self.height {
            if i == 0 {
                execute!(std::io::stdout(), MoveTo(self.x, self.y + i)).unwrap();
                if self.width > 3 {
                    if self.width == 4 {
                        execute!(std::io::stdout(), Print("┌──".to_string())).unwrap();
                    } else {
                        execute!(std::io::stdout(), Print("┌─ ".to_string())).unwrap();
                    }
                    let mut titlechars: Vec<char> = self.title.chars().collect();
                    let mut wchar = false;
                    for i in 0..(titlechars.len().min(self.width as usize - 3) * 2) {
                        if i >= titlechars.len() {
                            break;
                        }
                        if wchar {
                            wchar = false;
                            continue;
                        }
                        execute!(std::io::stdout(), Print(format!("{}", titlechars[i]))).unwrap();
                        if i < titlechars.len() - 1 {
                            if titlechars[i] > '\x7f' {
                                wchar = true;
                                titlechars.insert(i + 1, ' ');
                            }
                        }
                    }
                    if self.width > 3 + titlechars.len() as u16 {
                        execute!(std::io::stdout(), Print(" ".to_string())).unwrap();
                        if self.width > 4 + titlechars.len() as u16 {
                            let l = self.width as i16
                                - 5
                                - self.title.chars().collect::<Vec<char>>().len() as i16;
                            let l = if l < 0 { 0 } else { l as u16 };
                            for _ in 0..l {
                                execute!(std::io::stdout(), Print("─")).unwrap();
                            }
                        }
                    }
                    execute!(
                        std::io::stdout(),
                        MoveTo(self.x + self.width - 1, self.y + i),
                        Print("┐".to_string()),
                    )
                    .unwrap();
                } else {
                    match self.width {
                        1 => execute!(std::io::stdout(), Print("┌".to_string())).unwrap(),
                        2 => execute!(std::io::stdout(), Print("┌─".to_string())).unwrap(),
                        3 => execute!(std::io::stdout(), Print("┌─┐".to_string())).unwrap(),
                        _ => (),
                    }
                }
                continue;
            } else if i == self.height - 1 {
                execute!(std::io::stdout(), MoveTo(self.x, self.y + i)).unwrap();
                if self.width > 0 {
                    execute!(std::io::stdout(), Print("└")).unwrap();
                    if self.width > 1 {
                        let l = self.width as i16 - 2;
                        let l = if l < 0 { 0 } else { l } as u16;
                        for _ in 0..l {
                            execute!(std::io::stdout(), Print("─")).unwrap();
                        }
                        if self.width > 2 {
                            execute!(std::io::stdout(), Print("┘")).unwrap();
                        }
                    }
                }
                continue;
            }
            if i as usize > cont.len() {
                execute!(std::io::stdout(), MoveTo(self.x, self.y + i)).unwrap();
                for j in 0..self.width {
                    if j == 0 || j == self.width - 1 {
                        execute!(std::io::stdout(), Print('│')).unwrap();
                        continue;
                    } else if j == self.width - 2 {
                        let whl = (self.height - 2) as f64;
                        let barl = whl / self.contl as f64;
                        let start = barl * self.start_line as f64;
                        let start = start.floor();
                        let start = start as u16;
                        let end = barl * (self.start_line + 1) as f64;
                        let end = end.floor() + 1.0;
                        let end = end as u16;
                        if i - 1 >= start && i - 1 < end {
                            execute!(std::io::stdout(), Print('│')).unwrap();
                        } else {
                            execute!(std::io::stdout(), Print(' ')).unwrap();
                        }
                        continue;
                    } else {
                        execute!(std::io::stdout(), Print(' ')).unwrap();
                    }
                }
                continue;
            }
            let mut line: Vec<char> = cont[(i - 1) as usize].chars().collect();
            let mut is_wchar = false;
            for j in 0..self.width {
                execute!(std::io::stdout(), MoveTo(self.x + j, self.y + i)).unwrap();
                if j == 0 || j == self.width - 1 {
                    execute!(std::io::stdout(), Print('│')).unwrap();
                    continue;
                }
                if j == self.width - 2 {
                    let whl = (self.height - 2) as f64;
                    let barl = whl / self.contl as f64;
                    let start = barl * self.start_line as f64;
                    let start = start.floor();
                    let start = start as u16;
                    let end = barl * (self.start_line + 1) as f64;
                    let end = end.floor() + 1.0;
                    let end = end as u16;
                    if i - 1 >= start && i - 1 < end {
                        execute!(std::io::stdout(), Print('│')).unwrap();
                    } else {
                        execute!(std::io::stdout(), Print(' ')).unwrap();
                    }
                    continue;
                }
                if j == 1 {
                    execute!(std::io::stdout(), Print(' ')).unwrap();
                    continue;
                }
                if j as usize - 2 >= line.len() {
                    execute!(std::io::stdout(), Print(' ')).unwrap();
                    continue;
                }
                if is_wchar {
                    is_wchar = false;
                    continue;
                }
                if line[j as usize - 2] > '\x7f' {
                    is_wchar = true;
                    line.insert(j as usize - 2, ' ');
                }
                execute!(std::io::stdout(), Print(line[j as usize - 2])).unwrap();
            }
        }
    }
}
