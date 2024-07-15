fn has_data(time_of_day: u16) -> Option<u16> {
    if time_of_day < 600 { // Before 10:00 AM (10 * 60 = 600 minutes)
        Some(100)
    } else if time_of_day <= 1200 { // Between 10:01 AM and 8:00 PM (20 * 60 = 1200 minutes)
        Some(50)
    } else if time_of_day > 1320 { // After 11:00 PM (23 * 60 = 1320 minutes)
        None
    } else {
        Some(50) // The case between 8:01 PM and 11:00 PM
    }
}
    // return match time_of_day {
    //     0..=1000 => Some(100),
    //     1001..=2000 =>Some(50),
    //     _=>None
    // };
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[test]
    fn test_before_10_am() {
        assert_eq!(has_data(500), Some(100));
        assert_eq!(has_data(599), Some(100));
    }

    #[test]
    fn test_between_10_01_am_and_8_pm() {
        assert_eq!(has_data(601), Some(50));
        assert_eq!(has_data(1200), Some(50));
    }

    #[test]
    fn test_between_8_01_pm_and_11_pm() {
        assert_eq!(has_data(1201), Some(50));
        assert_eq!(has_data(1319), Some(50));
    }

    #[test]
    fn test_after_11_pm() {
        assert_eq!(has_data(1321), None);
        assert_eq!(has_data(1400), None);
    }
}
