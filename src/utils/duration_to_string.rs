use std::time::Duration;

pub fn duration_to_string(duration: Duration) -> String {
    format!(
        "{:02}:{:02}.{:02}",
        duration.as_secs() / 60,
        duration.as_secs() % 60,
        duration.subsec_millis() / 10,
    )
}
