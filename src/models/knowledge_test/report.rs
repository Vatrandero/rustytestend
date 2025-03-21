use super::KnolewdgeTest;

pub struct TestReport {
    test: KnolewdgeTest,
    time_start_unix: i64, // TODO: Consider change type to chrono::dateTime
    time_end_unix: i64,   // TODO: Consider change type to chrono::dateTime
}
