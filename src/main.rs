use chrono::NaiveDate;
use terminal_size::terminal_size;

use std::{
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug)]
struct Item {
    due: NaiveDate,
    text: String,
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (due, text) = s
            .strip_prefix("due:")
            .ok_or(())?
            .split_once(' ')
            .ok_or(())?;
        Ok(Self {
            due: due.parse().map_err(|_| ())?,
            text: text.to_owned(),
        })
    }
}

fn main() {
    let (terminal_size::Width(width), terminal_size::Height(height)) =
        terminal_size::terminal_size().unwrap();
    let items: Vec<Item> = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok().and_then(|x| x.parse().ok()))
        .collect();
    println!("{:?}", (width, height, items));
}
