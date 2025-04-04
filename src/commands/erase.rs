use std::fmt;

use crate::write_csi;

pub enum EraseInDisplay {
    FromCursorToEnd,
    FromCursorToBeginning,
    EntireScreen,
}

impl fmt::Display for EraseInDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EraseInDisplay::FromCursorToEnd => write_csi!(f, "J"),
            EraseInDisplay::FromCursorToBeginning => write_csi!(f, "1J"),
            EraseInDisplay::EntireScreen => write_csi!(f, "2J"),
        }
    }
}
