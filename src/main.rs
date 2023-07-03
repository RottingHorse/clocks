use std::mem::zeroed;
use chrono::{Datelike, DateTime, Local, Timelike, TimeZone};
use clap::{App, Arg};
use winapi::SYSTEMTIME;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);
        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 7,

        };

        let mut ns = t.nanosecond();
        let mut leap = 0;
        let is_leap_second = ns > 1_000_000_000;

        if is_leap_second {
            ns -= 1_000_000_000;
            leap +=1;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = (leap + t.second()) as WORD;
        systime.wMilliseconds = (ns/1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }
}

fn main() {
    let app = App::new("clock")
        .version("0.1")
        .about("Gets and (aspirationaly) sets the time.")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .takes_value(true)
                .possible_values(&[
                    "rfc2822",
                    "rfc3339",
                    "timestamp",
                ])
                .default_value("rfc3339"),
        )
        .arg(Arg::with_name("datetime")
            .help("When <action> is 'set', apply <datetime>. Otherwise, ignore.",
            ));

    let args = app.get_matches();

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        unimplemented!()
    }

    let now = Clock::get();

    match std {
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        "timestamp" => println!("{}", now.timestamp()),
        _ => unreachable!(),
    }
}
