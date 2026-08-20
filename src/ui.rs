pub mod app;
pub mod camera_2d;

use std::time::Duration;

pub fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();

    let days = secs / 86_400;
    let hours = (secs % 86_400) / 3_600;
    let minutes = (secs % 3_600) / 60;
    let seconds = secs % 60;

    if days > 0 {
        format!("{days:>3}d {hours:02}h")
    } else if hours > 0 {
        format!("{hours:>2}h {minutes:02}m")
    } else if minutes > 0 {
        format!("{minutes:>2}m {seconds:02}s")
    } else {
        format!("{seconds:>2}s")
    }
}
