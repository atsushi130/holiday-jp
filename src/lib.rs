
extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate chrono;

pub mod holiday_jp;
pub use holiday_jp::HolidayService;
mod https_client;