extern crate chrono;

use chrono::Duration;
use chrono::TimeZone;
use chrono::DateTime;

pub fn after <T> (st: DateTime<T> )-> DateTime<T> where T: TimeZone{
   let result = st + Duration::seconds(1_000_000_000);
   result
}