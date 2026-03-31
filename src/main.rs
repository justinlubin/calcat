mod grid;
mod item;

use ansi_term::Colour::Fixed;
use chrono::{Datelike, Local, Months, NaiveDate};
use grid::{Cell, Grid};
use item::Item;

use std::{
    collections::HashMap,
    io::{self, BufRead, IsTerminal},
};

use crate::grid::Text;

use clap::{
    CommandFactory, Parser,
    builder::{Styles, styling::AnsiColor},
};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(AnsiColor::Cyan.on_default().bold())
        .placeholder(AnsiColor::Cyan.on_default())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Yellow.on_default())
}

#[derive(Parser)]
#[command(
    version,
    about = "calcat - display a calendar of to-do items",
    long_about = None,
    styles = styles(),
)]
struct Cli {
    /// Display the number of months after the current month
    #[arg(short = 'A', long, value_name = "number", default_value_t = 0)]
    after: u8,
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
    let mut cmd = Cli::command();
    let cli = Cli::parse();

    let accent = 3;

    if io::stdin().is_terminal() {
        cmd.print_help().unwrap();
        return;
    }

    let items: Vec<Item> = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok()?.parse().ok())
        .collect();

    let (terminal_size::Width(width), terminal_size::Height(height)) =
        terminal_size::terminal_size().unwrap();

    let w = width as usize / 7;
    let ws = vec![w - 5, w + 2, w + 2, w + 2, w + 2, w + 2, w - 5];

    let now = Local::now();
    let now = now
        .checked_add_months(Months::new(cli.after.into()))
        .unwrap();
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
                lines: vec![Text {
                    text: format!(
                        "{:^width$}",
                        wd.to_owned(),
                        width = ws[i] - 1
                    ),
                    color: accent,
                }],
            },
        );
    }

    for d in 1..days_in_month(now.year(), now.month()) {
        cells.insert(
            offset(first_offset, d),
            Cell {
                lines: vec![Text {
                    text: d.to_string(),
                    color: 8,
                }],
            },
        );
    }

    for it in items {
        if it.due.year() != now.year() || it.due.month() != now.month() {
            continue;
        }
        let c = cells.get_mut(&offset(first_offset, it.due.day())).unwrap();
        c.lines.push(Text {
            text: it.text,
            color: 15,
        });
    }

    let h = height as usize / 5 - 1;

    let g = Grid::new(cells, vec![1, h, h, h, h, h], ws).unwrap();

    let header = format!(
        "{:—^width$}",
        now.format("[ %B %Y ]").to_string().to_uppercase(),
        width = w * 7
    );

    println!("{}\n{}", Fixed(accent).bold().paint(header), g);
}
