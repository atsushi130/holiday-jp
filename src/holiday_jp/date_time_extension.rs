
//  Created by Atsushi Miyake.

use chrono::{Datelike, DateTime, TimeZone};

pub trait DateTimeToString {
    fn to_format_string(&self) -> String;
    fn to_two_digits(&self, value: u32) -> String;
}

impl<Tz: TimeZone> DateTimeToString for DateTime<Tz> {

    fn to_format_string(&self) -> String {
        let month = self.to_two_digits(self.month());
        let day   = self.to_two_digits(self.day());
        format!("{}-{}-{}", self.year(), month, day)
    }

    fn to_two_digits(&self, value: u32) -> String {
        let string: String = value.to_string();
        match string.len() == 1 {
            true  => "0".to_string() + &string,
            false => string
        }
    }
}