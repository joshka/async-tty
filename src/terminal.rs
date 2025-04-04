use std::io::{Stdin, Stdout, stdin, stdout};
use std::os::fd::AsFd;
use std::task::Poll;

use rustix::io::write;
use rustix::termios::{OptionalActions, Termios, tcgetattr, tcsetattr};
use snafu::ResultExt;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::Result;
use crate::error::{
    GetTerminalAttributesSnafu, SetTerminalAttributesSnafu, SwitchToAlternateScreenSnafu,
    SwitchToMainScreenSnafu,
};

pub struct Terminal<In: AsFd, Out: AsFd> {
    pub(crate) stdin: In,
    pub(crate) stdout: Out,
    pub(crate) original_termios: Termios,
}

// Implement Unpin to allow the struct to be safely unpinned
impl<In: AsFd, Out: AsFd> Unpin for Terminal<In, Out> {}

impl Terminal<Stdin, Stdout> {
    pub fn new() -> Result<Self> {
        let stdin = stdin();
        let stdout = stdout();
        let original_termios = tcgetattr(&stdin).context(GetTerminalAttributesSnafu)?;
        Ok(Terminal {
            stdin,
            stdout,
            original_termios,
        })
    }
}

impl<In: AsFd, Out: AsFd> Terminal<In, Out> {
    /// Set the terminal to raw mode
    pub fn set_raw_mode(&self) -> Result<()> {
        let mut raw = self.original_termios.clone();
        raw.make_raw();
        // TODO work out whether we should be using Drain or Flush here instead of Now
        tcsetattr(&self.stdin, OptionalActions::Now, &raw).context(SetTerminalAttributesSnafu)?;
        Ok(())
    }

    /// Set the terminal to cooked mode
    pub fn set_cooked_mode(&self) -> Result<()> {
        // TODO work out whether we should be using Drain or Flush here instead of Now
        tcsetattr(&self.stdin, OptionalActions::Now, &self.original_termios)
            .context(SetTerminalAttributesSnafu)?;
        Ok(())
    }

    /// Switch to alternate screen buffer
    pub async fn switch_to_alternate_screen(&mut self) -> Result<()> {
        self.write(b"\x1b[?1049h")
            .await
            .context(SwitchToAlternateScreenSnafu)?;
        Ok(())
    }

    /// Switch back to main screen buffer
    pub async fn switch_to_main_screen(&mut self) -> Result<()> {
        self.write(b"\x1b[?1049l")
            .await
            .context(SwitchToMainScreenSnafu)?;
        Ok(())
    }
}

impl<In: AsFd, Out: AsFd> Drop for Terminal<In, Out> {
    fn drop(&mut self) {
        // Restore the original terminal attributes when the Terminal struct is dropped
        if let Err(e) = self.set_cooked_mode() {
            eprintln!("Failed to restore terminal attributes: {}", e);
        }
    }
}

impl<In: AsFd, Out: AsFd> AsyncWrite for Terminal<In, Out> {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let result = write(&this.stdout, buf);
        match result {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(e) => Poll::Ready(Err(std::io::Error::other(e))),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        // Implement flush if needed
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        // Implement shutdown if needed
        Poll::Ready(Ok(()))
    }
}
