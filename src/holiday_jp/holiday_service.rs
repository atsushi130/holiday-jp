
//  Created by Atsushi Miyake.

use std::io::Read;
use chrono::{Local, Datelike, DateTime, Duration, Weekday};
use rustc_serialize::json;
use holiday_jp::holiday::Holiday;
use holiday_jp::date_time_extension::DateTimeToString;
use https_client::HttpsClientBuilder;

pub struct HolidayService;

impl HolidayService {

    pub fn is_holiday(&self, date: DateTime<Local>) -> bool {
        if self.is_weekend(date) { return true }
        if self.is_beginning_of_the_year(date) { return true }
        self.is_public_holiday(date)
    }

    pub fn is_weekend(&self, date: DateTime<Local>) -> bool {
        let weekday = date.weekday();
        weekday == Weekday::Sat || weekday == Weekday::Sun
    }

    pub fn is_public_holiday(&self, date: DateTime<Local>) -> bool {

        let mut response = String::new();

        HttpsClientBuilder::build()
            .get(&self.build_url(date))
            .send()
            .unwrap()
            .read_to_string(&mut response)
            .unwrap();

        let holiday: Holiday = json::decode(&response).unwrap();
        !holiday.items.is_empty()
    }

    pub fn is_beginning_of_the_year(&self, date: DateTime<Local>) -> bool {
        date.month() == 1 && date.day() <= 3
    }

    fn build_url(&self, date: DateTime<Local>) -> String {
        let today    = date.to_format_string();
        let tomorrow = (date + Duration::days(1)).to_format_string();
        format!("https://clients6.google.com/calendar/v3/calendars/ja.japanese%23holiday@group.v.calendar.google.com/events?calendarId=ja.japanese%23holiday%40group.v.calendar.google.com&singleEvents=true&timeZone=Asia%2FTokyo&maxAttendees=1&maxResults=250&sanitizeHtml=true&timeMin={}T00%3A00%3A00%2B09%3A00&timeMax={}T00%3A00%3A00%2B09%3A00&key=AIzaSyBNlYH01_9Hc5S1J9vuFmu2nUqBZJNAXxs", today, tomorrow)
    }
}
