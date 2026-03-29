use std::str::FromStr;

use chrono::NaiveDate;

#[derive(Debug)]
pub struct Item {
    pub due: NaiveDate,
    pub text: String,
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
