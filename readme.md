# holiday-jp
[![MIT / Apache2.0 dual licensed](https://img.shields.io/badge/dual%20license-MIT%20/%20Apache%202.0-blue.svg)](./license-mit.md)
[![crates.io](https://img.shields.io/crates/v/holiday_jp.svg)](https://crates.io/crates/holiday_jp)
[![Document](https://img.shields.io/badge/Cryptor-Document-3B5998.svg)](https://docs.rs/holiday_jp/0.1.2/holiday_jp/)

## dependencies
```toml
[dependencies]
holiday_jp = "0.1.2"
```

## Usage
```rust
extern crate holiday_jp;
extern crate chrono;

use holiday_jp::HolidayService;
use chrono::Local;

fn main() {
    let date = Local::now();
    if HolidayService.is_holiday(date) {
        println!("today is a holiday!");
    }
}
```

**HolidayService API**
```rust
fn is_holiday(&self, date: DateTime<Local>) -> bool;
fn is_weekend(&self, date: DateTime<Local>) -> bool;
fn is_public_holiday(&self, date: DateTime<Local>) -> bool;
fn is_beginning_of_the_year(&self, date: DateTime<Local>) -> bool;
```

**DateTimeToString trait**
```rust
use holiday_jp::DateTimeToString;

let date_time_string = Local::now().to_format_string();
```

## License
**This project is dual-licensed under MIT and Apache 2.0.**
