use std::cmp::min;
use std::env;
use std::io;
use std::io::prelude::*;

const TUX: [&'static str; 13] = [
    r"  \\",
    r"   \\   _____",
    r"    ` /       \",
    r"     ; _    _  ;",
    r"     |(_)__(_) |",
    r"     | |   /   |",
    r"     | `--'    ;",
    r"    / /      \  \",
    r"   | |        |  |",
    r"   ( |        |  )",
    r"   /``\__   __/ ``\",
    r"  (      \-/       )",
    r"   \_____/-\______/",
];

const THRESH_LEN: usize = 50;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        print_tux(&args[1..].join("\n"));
    } else {
        let mut string = String::new();

        io::stdin().read_to_string(&mut string)?;
        print_tux(&string);
    }
    Ok(())
}

fn print_tux(text: &str) {
    let mut lines = Vec::with_capacity(text.len() / THRESH_LEN + 1);
    // let mut lines = vec![String::with_capacity(THRESH_LEN); text.len() / THRESH_LEN + 1];
    let mut index = 0;

    for line in text.lines() {
        let mut i = 0;
        lines.push(String::with_capacity(THRESH_LEN));

        for word in line.split_whitespace() {
            if i + word.len() <= THRESH_LEN {
                lines[index].push_str(word);

                i += word.len();
                if i < THRESH_LEN {
                    lines[index].push_str(" ");
                    i += 1;
                }
            } else if word.len() > THRESH_LEN {
                let mut j = 0;
                loop {
                    let end = j + min(word.len() - j, THRESH_LEN) - i;

                    if end >= word.len() {
                        lines[index].push_str(&word[j..]);
                        i = end - j;
                        if i < THRESH_LEN {
                            lines[index].push_str(" ");
                            i += 1;
                        }
                        break;
                    } else {
                        lines[index].push_str(&word[j..end]);
                        lines.push(String::with_capacity(THRESH_LEN));
                        index += 1;
                        j = end;
                        i = 0;
                    }
                }
            } else {
                lines.push(String::with_capacity(THRESH_LEN));
                index += 1;
                lines[index].push_str(word);

                i = word.len();
                if i < THRESH_LEN {
                    lines[index].push_str(" ");
                    i += 1;
                }
            }
        }

        index += 1;
    }

    let mut len = 0;
    for line in lines.iter() {
        if line.len() > len {
            len = line.len();
        }
    }

    len += 2;
    let half_len = len / 2;

    println!("   {}", "_".repeat(len));
    println!("  /{}\\", " ".repeat(len));

    for line in lines {
        println!(" |  {}{}  |", line, " ".repeat(len - line.len() - 2));
    }

    println!(
        "  \\{} {}/",
        "_".repeat(half_len),
        "_".repeat(len - half_len - 1)
    );

    for line in &TUX {
        println!("{} {}", " ".repeat(half_len), line);
    }
}
