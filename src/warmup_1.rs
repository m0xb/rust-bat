pub fn sleep_in(weekday: bool, vacation: bool) -> bool {
    return !weekday || vacation;
}

