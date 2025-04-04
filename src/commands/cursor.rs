use std::fmt;

use crate::write_csi;

pub struct CursorUp {
    pub count: usize,
}

impl fmt::Display for CursorUp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_csi!(f, "{}A", self.count)
    }
}

pub struct CursorDown {
    pub count: usize,
}

impl fmt::Display for CursorDown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_csi!(f, "{}B", self.count)
    }
}

pub struct CursorForward {
    pub count: usize,
}

impl fmt::Display for CursorForward {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_csi!(f, "{}C", self.count)
    }
}

pub struct CursorBackward {
    pub count: usize,
}

impl fmt::Display for CursorBackward {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_csi!(f, "{}D", self.count)
    }
}

pub struct CursorNextLine {
    pub count: usize,
}

impl fmt::Display for CursorNextLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_csi!(f, "{}E", self.count)
    }
}

pub struct CursorPreviousLine {
    pub count: usize,
}

impl fmt::Display for CursorPreviousLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_csi!(f, "{}F", self.count)
    }
}

pub struct CursorPosition {
    pub row: usize,
    pub column: usize,
}

impl fmt::Display for CursorPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_csi!(f, "{};{}H", self.row, self.column)
    }
}
