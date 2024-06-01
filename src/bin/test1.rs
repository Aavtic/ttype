use std::io::Write;
use crossterm::style::{Print, Color, Stylize};
use crossterm::queue;
use std::time::Duration;

fn main() {
    let mut stdout = std::io::stdout();
    let stylized = "Hello There"
        .with(Color::Red)
        .on(Color::Grey);
    queue!(stdout, 
           Print(stylized)).unwrap();
    queue!(stdout, 
           Print(format!("Hello Friend {}", 0 > 0).with(Color::Red))).unwrap();

    // std::thread::sleep(Duration::from_secs(5));
    println!("{},{}", 0 < 0, 0 > 0 );
    stdout.flush().unwrap();
}
