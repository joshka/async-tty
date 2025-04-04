use std::time::Duration;

use async_tty::commands::cursor::{CursorDown, CursorPosition, CursorUp};
use tokio::{io::AsyncWriteExt, time::sleep};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = async_tty::Terminal::new()?;
    terminal.switch_to_alternate_screen().await?;
    let cursor_movements = format!(
        "{}at 20,10{}up 1{}down 2",
        CursorPosition {
            row: 10,
            column: 20,
        },
        CursorUp { count: 1 },
        CursorDown { count: 2 }
    );
    terminal.write_all(cursor_movements.as_bytes()).await?;
    sleep(Duration::from_secs(2)).await;
    terminal.switch_to_main_screen().await?;
    Ok(())
}
