use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::Position;
use termion::color; //termion终端颜色的包

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
                //saturating_sub/add:https://doc.rust-lang.org/std/intrinsics/fn.saturating_sub.html
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    //设置终端的RGB颜色
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    //重置终端的RGB颜色
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    //设置status bar文件名的颜色
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    //重置status bar文件名的颜色
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
