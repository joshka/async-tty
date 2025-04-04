use async_tty::Terminal;

fn main() -> async_tty::Result<()> {
    let terminal = Terminal::new()?;
    println!("cooked mode\n1234567890");
    terminal.set_raw_mode()?;
    println!("raw mode\n1234567890");
    terminal.set_cooked_mode()?;
    println!("cooked mode\n1234567890");
    Ok(())
}
