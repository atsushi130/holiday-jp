
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate holiday_jp;
extern crate chrono;

#[cfg(test)]
mod holiday_service_tests {

    use holiday_jp::HolidayService;
    use chrono::{Local, Datelike, DateTime, TimeZone};

    #[test]
    fn is_holiday() {
        let test_date = Local.ymd(2017, 1, 9).and_hms(0, 0, 0);
        let result    = HolidayService.is_holiday(test_date);
        assert_eq!(true, result);
    }

    #[test]
    fn is_not_holiday() {
        let test_date = Local.ymd(2017, 1, 4).and_hms(0, 0, 0);
        let result    = HolidayService.is_holiday(test_date);
        assert_eq!(false, result);
    }

    #[test]
    fn is_public_holiday() {
        let test_date = Local.ymd(2017, 1, 1).and_hms(0, 0, 0);
        let result    = HolidayService.is_public_holiday(test_date);
        assert_eq!(true, result);
    }

    #[test]
    fn is_not_public_holiday() {
        let test_date = Local.ymd(2017, 1, 3).and_hms(0, 0, 0);
        let result    = HolidayService.is_public_holiday(test_date);
        assert_eq!(false, result);
    }

    #[test]
    fn is_weekend() {
        let test_date = Local.ymd(2017, 1, 1).and_hms(0, 0, 0);
        let result    = HolidayService.is_weekend(test_date);
        assert_eq!(true, result);
    }

    #[test]
    fn is_not_weekend() {
        let test_date = Local.ymd(2017, 1, 2).and_hms(0, 0, 0);
        let result    = HolidayService.is_weekend(test_date);
        assert_eq!(false, result);
    }

    #[test]
    fn is_beginning_of_the_year() {
        let test_date = Local.ymd(2017, 1, 3).and_hms(0, 0, 0);
        let result    = HolidayService.is_beginning_of_the_year(test_date);
        assert_eq!(true, result);
    }

    #[test]
    fn is_not_beginning_of_the_year() {
        let test_date = Local.ymd(2017, 1, 4).and_hms(0, 0, 0);
        let result    = HolidayService.is_beginning_of_the_year(test_date);
        assert_eq!(false, result);
    }
}