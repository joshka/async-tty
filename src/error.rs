use rustix::io::Errno;
use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Failed to get terminal attributes"))]
    GetTerminalAttributes { source: Errno },
    #[snafu(display("Failed to set terminal attributes"))]
    SetTerminalAttributes { source: Errno },
    #[snafu(display("Failed to switch to alternate screen"))]
    SwitchToAlternateScreen { source: std::io::Error },
    #[snafu(display("Failed to switch to main screen"))]
    SwitchToMainScreen { source: std::io::Error },
}
