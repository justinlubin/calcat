use std::{collections::HashMap, fmt::Display};

use ansi_term::Colour::Fixed;

// https://stackoverflow.com/a/38461750
fn truncate_pretty(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => s.to_owned(),
        Some((idx, _)) => format!("{}…", &s[..idx - 1]),
    }
}

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub color: u8, // 0-15
    pub bg: u8,    // 0-15
}

#[derive(Debug)]
pub struct Cell {
    pub lines: Vec<Text>,
}

#[derive(Debug)]
pub struct Grid {
    cells: HashMap<u32, Cell>,
    row_heights: Vec<usize>,
    col_widths: Vec<usize>,
}

impl Grid {
    pub fn new(
        cells: HashMap<u32, Cell>,
        row_heights: Vec<usize>,
        col_widths: Vec<usize>,
    ) -> Option<Self> {
        if cells.keys().max().cloned().unwrap_or(0) as usize
            > row_heights.len() * col_widths.len()
        {
            return None;
        }

        Some(Self {
            cells,
            row_heights,
            col_widths,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cw_len = self.col_widths.len();

        for (i, rh) in self.row_heights.iter().enumerate() {
            for line_idx in 0..*rh {
                for (j, cw) in self.col_widths.iter().enumerate() {
                    let cell_idx = i * cw_len + j % cw_len;
                    let lines = match self.cells.get(&(cell_idx as u32)) {
                        Some(c) => &c.lines,
                        None => &vec![],
                    };
                    let blank = Text {
                        text: "".to_owned(),
                        color: 0,
                        bg: 0,
                    };
                    let text = lines.get(line_idx).unwrap_or(&blank);
                    let line = truncate_pretty(&text.text, *cw - 1);
                    write!(
                        f,
                        "{}",
                        Fixed(text.color).on(Fixed(text.bg)).paint(format!(
                            "{:<width$}",
                            line,
                            width = cw
                        ))
                    )?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
