use chrono::{Local, NaiveDateTime};

pub fn now_beijing_time() -> NaiveDateTime {
    // let dt = FixedOffset::east_opt(8 * 3600).unwrap();
    Local::now().naive_local()
}