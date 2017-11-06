# holiday-jp

## dependencies
```toml
[dependencies]
holiday-jp = "0.1.0"
```

## Usage
```rust
extern crate holiday_jp;
extern crate chrono;

use holiday_jp::HolidayService;
use chrono::Local;

fun main() {
    let date = Local::now();
    if HolidayService.is_holiday(date) {
        println!("today is a holiday!");
    }
}
```
