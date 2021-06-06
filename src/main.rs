use std::cmp::min;
use std::env;
use std::io;
use std::io::prelude::*;

const TUX: [&str; 13] = [
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

    for text_line in text.lines() {
        let mut line = String::with_capacity(THRESH_LEN);
        let mut line_len = 0;

        for word in text_line.split_whitespace() {
            let word_len = word.chars().count();

            if line_len + word_len <= THRESH_LEN {
                line.push_str(word);
                line_len += word_len;

                if line_len < THRESH_LEN {
                    line.push(' ');
                    line_len += 1;
                }
            } else if word_len > THRESH_LEN {
                // very long word, split into multiple lines
                let mut j = 0;

                loop {
                    let end = j + min(word_len - j, THRESH_LEN) - line_len;

                    if end >= word_len {
                        line.push_str(&word[j..]);
                        line_len = end - j;

                        if line_len < THRESH_LEN {
                            line.push(' ');
                            line_len += 1;
                        }
                        break;
                    } else {
                        line.push_str(&word[j..end]);
                        line_len = end - j;
                        lines.push((line, line_len));

                        line = String::with_capacity(THRESH_LEN);
                        j = end;
                        line_len = 0;
                    }
                }
            } else {
                lines.push((line, line_len));
                line = String::with_capacity(THRESH_LEN);
                line.push_str(word);
                line_len = word_len;

                if line_len < THRESH_LEN {
                    line.push(' ');
                    line_len += 1;
                }
            }
        }

        while line.ends_with(' ') {
            line.pop();
            line_len -= 1;
        }

        lines.push((line, line_len));
    }

    let max_len = lines.iter().map(|&(_, len)| len).max().unwrap_or(0);

    println!("   {:_>len$}", "", len = max_len + 2);
    println!("  /{:>len$}\\", "", len = max_len + 2);

    for (line, _) in lines {
        println!(" |  {:len$}  |", line, len = max_len);
    }

    println!("  \\{:_^len$}/", ' ', len = max_len + 2);

    for line in &TUX {
        println!("{:len$} {}", "", line, len = (max_len + 1) / 2);
    }
}
