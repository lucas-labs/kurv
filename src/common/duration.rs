use chrono::Duration;

pub fn humanize_duration(duration: Duration) -> String {
    if duration.num_days() >= 30 {
        let months = duration.num_days() / 30;
        return format!("{} month{}", months, if months > 1 { "s" } else { "" });
    }

    if duration.num_days() > 0 {
        let days = duration.num_days();
        return format!("{}d", days);
    }

    if duration.num_hours() > 0 {
        let hours = duration.num_hours();
        return format!("{}h", hours);
    }

    if duration.num_minutes() > 0 {
        let minutes = duration.num_minutes();
        return format!("{}m", minutes);
    }

    if duration.num_seconds() > 0 {
        let seconds = duration.num_seconds();
        return format!("{}s", seconds);
    }

    "< 1 second".to_string()
}