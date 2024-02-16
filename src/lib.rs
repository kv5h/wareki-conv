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
}
