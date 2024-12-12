use super::KnolewdgeTest;
use chrono::DateTime;

pub struct TestReport {
    test: KnolewdgeTest,
    time_start_unix: i64, // TODO: Consider change type to chrono::dateTime
    time_end_unix: i64,   // TODO: Consider change type to chrono::dateTime
}
