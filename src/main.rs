mod grid;
mod item;

use chrono::{DateTime, Datelike, Local, NaiveDate};
use grid::{Cell, Grid};
use item::Item;

use std::{
    collections::HashMap,
    io::{self, BufRead},
};

// https://stackoverflow.com/a/38461750
fn truncate_pretty(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => s.to_owned(),
        Some((idx, _)) => format!("{}…", &s[..idx - 1]),
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    // Wrap to January of the next year if month is December
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd_opt(y, m, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day()
}

fn offset(first_offset: u32, day: u32) -> u32 {
    first_offset + 6 + day
}

// offset(3, 1) == 10
// offset(3, 2) == 11
// offset(3, 5) == 14
//
//

// 0 1 2 3 4 5 6
//       1 2 3 4
// 5 6 7 8
//
// for 1

// let it: Item = line.parse().ok()?;
// if it.due.year() != now.year() || it.due.month() != now.month() {
//     return None;
// }
// Some((Cell {
//     lines: vec![it.text],
// },))
fn main() {
    let items: Vec<Item> = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok()?.parse().ok())
        .collect();

    let (terminal_size::Width(width), terminal_size::Height(height)) =
        terminal_size::terminal_size().unwrap();

    let w = width as usize / 7;

    let now = Local::now();
    let first_offset = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
        .unwrap()
        .weekday()
        .num_days_from_sunday();

    let mut cells: HashMap<u32, Cell> = HashMap::new();

    let weekdays = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    for (i, wd) in weekdays.into_iter().enumerate() {
        cells.insert(
            i as u32,
            Cell {
                lines: vec![format!("{:^width$}", wd.to_owned(), width = w)],
            },
        );
    }

    for d in 1..days_in_month(now.year(), now.month()) {
        cells.insert(
            offset(first_offset, d),
            Cell {
                lines: vec![d.to_string()],
            },
        );
    }

    for it in items {
        let c = cells.get_mut(&offset(first_offset, it.due.day())).unwrap();
        c.lines.push(it.text);
    }

    let h = height as usize / 5 - 1;

    let g = Grid::new(cells, vec![1, h, h, h, h, h], [w].repeat(7)).unwrap();

    println!("{:=^width$}", now.format(" %B %Y "), width = w * 7);

    println!("{}", g);
}
