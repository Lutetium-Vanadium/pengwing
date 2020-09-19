use std::cmp::{max, min};
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
        for arg in &args[1..] {
            print_tux(arg);
        }
    } else {
        let mut string = String::new();

        io::stdin().read_to_string(&mut string)?;
        print_tux(&string);
    }
    Ok(())
}

fn print_tux(text: &str) {
    let mut word_len_max = 0;
    let mut line_len_max = 0;

    for word in text.split_whitespace() {
        if word_len_max < word.len() {
            word_len_max = word.len();
        }
    }

    for line in text.lines() {
        if line_len_max < line.len() {
            line_len_max = line.len();
        }
    }

    let text_len = if line_len_max > THRESH_LEN {
        min(word_len_max, THRESH_LEN)
    } else {
        line_len_max
    };
    // word_len_max = min(word_len_max, THRESH_LEN);
    // line_len_max = min(line_len_max, THRESH_LEN);

    // let text_len = max(word_len_max, line_len_max);

    let len = text_len + 2;
    let half_len = len / 2;
    println!("   {}", "_".repeat(len));
    println!("  /{}\\", " ".repeat(len));

    for line in text.lines() {
        let mut i = 0;
        print!(" |  ");

        for word in line.split_whitespace() {
            if i + word.len() <= text_len {
                print!("{}", word);

                i += word.len();
                if i < text_len {
                    print!(" ");
                    i += 1;
                }
            } else if word.len() > text_len {
                let mut j = 0;
                loop {
                    let end = j + min(word.len() - j, text_len) - i;

                    if end >= word.len() {
                        print!("{}", &word[j..]);
                        i = end - j;
                        if i < text_len {
                            print!(" ");
                            // adding 1 for the space
                            i += 1;
                        }
                        break;
                    } else {
                        print!("{}  |\n |  ", &word[j..end]);
                        j = end;
                        i = 0;
                    }
                }
            } else {
                print!("{}  |\n |  {}", " ".repeat(text_len - i), word);

                i = word.len();
                if i < text_len {
                    print!(" ");
                    i += 1;
                }
            }
        }

        println!("{}  |", " ".repeat(text_len - i));
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
