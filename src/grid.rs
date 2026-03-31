use std::{collections::HashMap, fmt::Display};

// https://stackoverflow.com/a/38461750
fn truncate_pretty(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => s.to_owned(),
        Some((idx, _)) => format!("{}…", &s[..idx - 1]),
    }
}

#[derive(Debug)]
pub struct Cell {
    pub lines: Vec<String>,
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
                    let line = truncate_pretty(
                        lines
                            .get(line_idx)
                            .map(String::as_ref)
                            .unwrap_or_else(|| ""),
                        *cw,
                    );
                    write!(f, "{:<width$}", line, width = cw)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
