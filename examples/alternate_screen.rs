use std::time::Duration;

use async_tty::Terminal;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> async_tty::Result<()> {
    let mut terminal = Terminal::new()?;
    terminal.switch_to_alternate_screen().await?;
    println!("alternate screen");
    sleep(Duration::from_secs(2)).await;
    terminal.switch_to_main_screen().await?;
    println!("main screen");
    Ok(())
}
