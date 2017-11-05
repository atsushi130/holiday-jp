
//  Created by Atsushi Miyake.

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Holiday {
    pub summary: String,
    pub items: Vec<HolidayItem>
}

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct HolidayItem {
    pub summary: String
}
