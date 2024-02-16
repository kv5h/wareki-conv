#![doc(html_root_url = "https://docs.rs/wareki-conv/0.1.0")]
//! Converts Japanese Wareki date into ISO based format.
//!

pub mod conv;

/// tests
#[cfg(test)]
mod tests {
    use super::conv::*;
    use chrono::prelude::*;
    use chrono::Utc;

    #[test]
    fn test_jis_x0301_basic() {
        let input_1 = "01.02.03";
        assert_eq!(
            convert(input_1),
            Utc.with_ymd_and_hms(2019, 2, 3, 0, 0, 0).unwrap()
        );
        let input_2 = "1.2.3";
        assert_eq!(
            convert(input_2),
            Utc.with_ymd_and_hms(2019, 2, 3, 0, 0, 0).unwrap()
        );
        let input_3 = "10.02.03";
        assert_eq!(
            convert(input_3),
            Utc.with_ymd_and_hms(2028, 2, 3, 0, 0, 0).unwrap()
        );
        let input_4 = "06.2.3";
        assert_eq!(
            convert(input_4),
            Utc.with_ymd_and_hms(2024, 2, 3, 0, 0, 0).unwrap()
        );
    }
    fn test_jis_x0301_extended() {
        let input_1 = "R01.02.03";
        assert_eq!(
            convert(input_1),
            Utc.with_ymd_and_hms(2019, 2, 3, 0, 0, 0).unwrap()
        );
        let input_2 = "R10.2.3";
        assert_eq!(
            convert(input_2),
            Utc.with_ymd_and_hms(2028, 2, 3, 0, 0, 0).unwrap()
        );
        let input_3 = "M01.02.03";
        assert_eq!(
            convert(input_3),
            Utc.with_ymd_and_hms(1868, 2, 3, 0, 0, 0).unwrap()
        );
        let input_4 = "M45.2.3";
        assert_eq!(
            convert(input_4),
            Utc.with_ymd_and_hms(1912, 2, 3, 0, 0, 0).unwrap()
        );
        let input_5 = "T01.02.03";
        assert_eq!(
            convert(input_5),
            Utc.with_ymd_and_hms(1912, 2, 3, 0, 0, 0).unwrap()
        );
        let input_6 = "S01.2.3";
        assert_eq!(
            convert(input_6),
            Utc.with_ymd_and_hms(1926, 2, 3, 0, 0, 0).unwrap()
        );
        let input_7 = "H01.02.03";
        assert_eq!(
            convert(input_7),
            Utc.with_ymd_and_hms(1989, 2, 3, 0, 0, 0).unwrap()
        );
    }
}
