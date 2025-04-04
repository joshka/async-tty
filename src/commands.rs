pub mod cursor;
pub mod erase;

/// Control Sequence Introducer (CSI) macro
///
/// Returns a string that starts with the CSI escape sequence (`ESC` + `[`) followed by a sequence
/// of parameters and a command.
///
/// # Examples
/// ```
/// use async_tty::csi;
///
/// let count = 1;
/// let cursor_up = csi!("{count}A");
/// assert_eq!(cursor_up, "\x1b[1A");
///
/// let x = 10;
/// let y = 20;
/// let cursor_position = csi!("{x};{y}H");
/// assert_eq!(cursor_position, "\x1b[10;20H");
/// ```
#[macro_export]
macro_rules! csi {
    ($fmt:expr $(, $args:expr)*) => {{
        let string = format!("\x1b[{}", format_args!($fmt $(, $args)*));
        string
    }};
}

#[macro_export]
macro_rules! write_csi {
    ($writer:expr, $fmt:expr $(, $args:expr)*) => {{
        write!($writer, "\x1b[{}", format_args!($fmt $(, $args)*))
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn csi_with_single_parameter() {
        let count = 5;
        assert_eq!(csi!("{count}A"), "\x1b[5A");
    }

    #[test]
    fn csi_with_multiple_parameters() {
        let x = 10;
        let y = 20;
        assert_eq!(csi!("{x};{y}H"), "\x1b[10;20H");
    }

    #[test]
    fn csi_without_parameters() {
        assert_eq!(csi!("K"), "\x1b[K");
    }
}
