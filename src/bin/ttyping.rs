use std::io::{self, Write};
use crossterm::{QueueableCommand, cursor, queue};
use crossterm::terminal::{self, ClearType};
use crossterm::event::{Event, poll, read, KeyCode, KeyModifiers};
use crossterm::style::{Print, Colors, Color, SetColors, Stylize};
use std::thread;
use std::time::Duration;
use std::fs;


fn _debug<T: std::fmt::Display + std::convert::AsRef<[u8]>>(content: T) {
    fs::write("debug.txt", content).unwrap()
}

// 167, 39

fn process_text(text: &mut String, w: u16) -> Vec<String>{
    let width =  w as usize - 2;
    let mut res_lines:Vec<String> = vec![];
    let mut text_iter = text.chars();

    while let Some(line) = text_iter.by_ref().take(width).collect::<String>().into() {
        if line != "" {
            if line.contains('\n') | line.contains("\r\n") {
                let mut nl_lines = if line.contains('\n') {
                    line.split("\n")
                }else {
                    line.split("\r\n")
                };
                while let Some(nl_line) = nl_lines.next() {
                    let rem = width - nl_line.len();
                    let to_push = nl_line.to_string() + &" ".repeat(rem);
                    res_lines.push(to_push);
                } 
            }else {
                res_lines.push(line);
            }
        }else {
            break
        }
    }

    *text = res_lines.join("");
    return res_lines;
}

fn render_box(w: u16, h: u16, text: &mut String, curr_x: &mut u16, curr_y: &mut u16, ix: usize, def_text: &str,stdout: &mut impl Write) {
    let hbar = "━";
    let vbar = "┃";
    let cornerul = "┏";
    let cornerur = "┓";
    let cornerdl = "┗";
    let cornerdr = "┛";
    let width = ((70f32/100f32) * w as f32) as u16;
    let height = ((70f32/100f32) * h as f32) as u16;
    let bar = hbar.repeat(width as usize - 2);
    let mut drawn_right = false;
    let bar0 = format!("{}{}{}", cornerul, &bar, cornerur);
    let bar1 = format!("{}{}{}", cornerdl, &bar, cornerdr);
    let (tlc, tlr) = (((h as f32 - height as f32)/2f32) as u16, (((w as f32 - width as f32))/2f32) as u16);
    let mut current_line  = tlc + 1;
    let mut def_text = def_text.to_string();
    let text_iter = process_text(text, width).into_iter();
    let def_iter = process_text(&mut def_text, width).into_iter();

    if (ix as u16) > (width - 3) {
        *curr_x = (tlr + 1) + (ix as u16 % (width - 2));
        *curr_y = tlc + 1 + (ix as u16 / (width - 2));
    } else {
        *curr_x = (tlr + 1) + ix as u16;
        *curr_y = tlc + 1;
    }

    stdout.queue(cursor::MoveTo(tlr, tlc)).unwrap();
    stdout.write(bar0.as_bytes()).unwrap();

    for _ in 0..=1 {
        for _ in 0..height {
            stdout.queue(cursor::MoveDown(1)).unwrap();
            stdout.queue(cursor::MoveLeft(1)).unwrap();
            stdout.write(vbar.as_bytes()).unwrap();
        }
        if !drawn_right {
            stdout.queue(cursor::MoveTo(tlr , tlc)).unwrap();
            stdout.queue(cursor::MoveRight(1)).unwrap();
            drawn_right = true;
        }
    }
    stdout.queue(cursor::MoveLeft(1)).unwrap();
    stdout.write(bar1.as_bytes()).unwrap();

    stdout.queue(cursor::MoveTo(tlr -1 , tlc + 1)).unwrap();    

    let mut i = 0;
    for (user_line, def_line) in text_iter.zip(def_iter) {
        stdout.queue(cursor::MoveTo(tlr + 1, current_line as u16)).unwrap();
        for (usr_chr, def_chr) in user_line.chars().zip(def_line.chars()) {
            if i < ix {
                if usr_chr == def_chr {
                    queue!(stdout, Print(usr_chr.with(Color::Yellow))).unwrap();
                } else {
                    queue!(stdout, Print(def_chr.with(Color::Red))).unwrap();
                }
            } else if i == ix {
                queue!(stdout, Print(usr_chr.with(Color::Grey))).unwrap();
            } else {
                queue!(stdout, Print(usr_chr.with(Color::Grey))).unwrap();
            }
            i += 1;
        }
                // stdout.write(line.as_bytes()).unwrap();
        current_line  += 1;
    }
    stdout.queue(SetColors(Colors::new(Color::Grey, Color::Black))).unwrap();
    stdout.queue(cursor::MoveTo(*curr_x, *curr_y)).unwrap();
    
    stdout.flush().unwrap();
}


fn main()  -> std::io::Result<()>{
    let mut running = true;
    // let texts:&str = r#"“I shall one of America’s greatest poets, wrote this adventure poem which has no doubt inspired more travels than we could ever know. It’s a call to courage, to face the unknown, and to break from the crowd and follow your own path wherever it may lead. "#;
    let texts = "This festival celebrates Lord Shiva's marriage to Parvati, symbolising the union of the conscious (Shiva) and the unconscious (Parvati), and the creation of the universe. Devotees observe a day-long fast, engage in prayers, chant ‘Om Namah Shivaya,’ and participate in night-long vigils. Mahashivratri is also considered an auspicious time for spiritual progress, self-reflection, and seeking blessings for good fortune, peace, and liberation.";
    let mut mutated_text = texts.to_owned();
    let mut stdout = io::stdout();
    let (mut ix, _iy): (usize, usize) = (0, 0);
    let (mut cx, mut cy) = (0u16, 0u16);
    let _ = terminal::enable_raw_mode().unwrap();
    let (mut w,mut h) = terminal::size().unwrap();
    let mut state_change = true;
    stdout.queue(SetColors(Colors::new(Color::Grey, Color::Black))).unwrap();
    stdout.flush().unwrap();

    while running {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    state_change = true;
                }// you 
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char(c) => {
                            state_change = true;
                            if c == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                                running = false;
                            }else  {
                                if ix < mutated_text.len() {
                                    mutated_text.remove(ix);
                                    mutated_text.insert(ix, c);
                                    ix += 1;
                                }
                            }
                        },
                        KeyCode::Backspace => {
                            if !(ix == 0) { 
                                let last_char = texts.chars().nth(ix - 1).unwrap();
                                let _ = mutated_text.remove(ix - 1);
                                mutated_text.insert(ix - 1, last_char);
                                ix -= 1;
                                state_change = true;
                            }
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }

        if state_change {
            stdout.queue(terminal::Clear(ClearType::All)).unwrap();
            render_box(w, h, &mut mutated_text, &mut cx, &mut cy, ix, texts, &mut stdout);
            state_change = false;
        }
        
        thread::sleep(Duration::from_millis(16));
    }
    let _ = terminal::disable_raw_mode();
    Ok(())
}
