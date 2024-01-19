use rand;
use clap::Parser;
use crossterm::{event::{read, Event, KeyCode}, style::{Stylize, StyledContent}};

use std::{fs::{self, DirEntry}, io::{self, Write}, usize};

mod stats;
use stats::Stats;

#[derive(Default, Parser, Debug)]
#[clap(author="Brody Keane", version, about)]
/// Typing trainer that uses your code for practice
struct Arguments {
    /// File that will be sourced for your typing practice
    #[clap(short, long)]
    file: Option<String>,

    /// Practice with random file from given directory
    #[clap(default_value = ".", short, long)]
    directory: String,
}

fn main() {
    let args = Arguments::parse();
    if let Some(path) = args.file {
        train(path);
    } else {
        let path = random_file(args.directory);
        train(path);
    }
}

fn random_file(directory: String) -> String {
    let entries: Vec<DirEntry> = fs::read_dir(&directory)
        .expect(&format!("Unable to access directory: {}", directory))
        .filter_map(|e| e.ok())
        .collect();

    let mut attempts = 0;

    while attempts < 3 {
        let idx = rand::random::<usize>() % entries.len();
        let filetype = match entries[idx].file_type() {
            Ok(filetype) => filetype,
            Err(_) => {
                attempts += 1;
                continue
            },
        };

        let path = match filetype.is_file() {
            true => entries[idx].path().display().to_string(),
            false => random_file(entries[idx].path().display().to_string())
        };
        return path
    }
    panic!("Failed to find file. This is usually caused by symlinks.")
}

fn train(path: String) {
    let file = fs::read_to_string(&path)
        .expect(&format!("Unable to read file: {}", path));

    let lines = file.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && l.len() > 5);

    let mut file_stats = Stats::default();
    crossterm::terminal::enable_raw_mode().unwrap();

    for mut line in lines {
       match display_line(&mut line) {
            Some(line_stats) => file_stats.add(line_stats),
            None => break,
        }
    }
    crossterm::terminal::disable_raw_mode().unwrap();
    println!();
}

fn display_line(mut line: &str) -> Option<Stats> {
    clear_terminal();
    print!("{}\r", line);
    io::stdout().flush().unwrap();

    let mut idx: usize = 0;
    let mut chars = line.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    loop {
        print!("\r{}", chars.join(""));
        io::stdout().flush().unwrap();

        let keycode = match read().unwrap() {
            Event::Key(event) => event.code,
            _ => continue
        };

        let key = match keycode {
            KeyCode::Esc => return None,
            KeyCode::Enter => return Some(Stats::default()),
            KeyCode::F(c) => c.to_string(),
            KeyCode::Char(c) => c.to_string(),
            KeyCode::Backspace => {
                if idx > 0 { idx -= 1; }
                chars[idx] = default_str(&chars[idx]);
                continue
            }
            _ => continue
        };

        if idx >= chars.len() {
            continue
        }

        chars[idx] = match chars[idx] == key {
            true => green_str(&chars[idx]),
            false => red_str(&chars[idx]),

        };

        idx += 1;
    }
}

fn green_str(x: &str) -> String {
    format!("\u{001b}[32m{}\u{001b}[0m", x)
}

fn red_str(x: &str) -> String {
    format!("\u{001b}[31m{}\u{001b}[0m", x)
}

fn default_str(x: &str) -> String {
    match x.chars().nth(5) {
        Some(c) => c.to_string(),
        None => x.to_string(),
    }
}

fn clear_terminal() {
    println!("\x1B[2J\x1B[H");
}
