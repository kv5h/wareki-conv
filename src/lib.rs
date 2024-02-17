#![doc(html_root_url = "https://docs.rs/wareki-conv/0.1.0")]
//! Converts Wareki (JIS X 0301) based date into ISO 8601 based one

#![feature(test)]
extern crate test;

pub mod conv;

/// tests
#[cfg(test)]
mod tests {
    use super::conv::*;
    use chrono::prelude::*;
    use chrono::Utc;
    use std::collections::HashMap;
    use test::Bencher;

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
        let input_5 = "０６．０２．０３";
        assert_eq!(
            convert(input_5),
            Utc.with_ymd_and_hms(2024, 2, 3, 0, 0, 0).unwrap()
        );
    }
    #[test]
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
        let input_8 = "Ｈ０１．０２．０３";
        assert_eq!(
            convert(input_8),
            Utc.with_ymd_and_hms(1989, 2, 3, 0, 0, 0).unwrap()
        );
    }
    #[test]
    fn test_jis_x0301_extended_with_kanji() {
        let input_1 = "令01.02.03";
        assert_eq!(
            convert(input_1),
            Utc.with_ymd_and_hms(2019, 2, 3, 0, 0, 0).unwrap()
        );
        let input_2 = "令1.2.3";
        assert_eq!(
            convert(input_2),
            Utc.with_ymd_and_hms(2019, 2, 3, 0, 0, 0).unwrap()
        );
        let input_3 = "明01.02.03";
        assert_eq!(
            convert(input_3),
            Utc.with_ymd_and_hms(1868, 2, 3, 0, 0, 0).unwrap()
        );
        let input_4 = "大01.2.3";
        assert_eq!(
            convert(input_4),
            Utc.with_ymd_and_hms(1912, 2, 3, 0, 0, 0).unwrap()
        );
        let input_5 = "昭01.02.03";
        assert_eq!(
            convert(input_5),
            Utc.with_ymd_and_hms(1926, 2, 3, 0, 0, 0).unwrap()
        );
        let input_6 = "平01.2.3";
        assert_eq!(
            convert(input_6),
            Utc.with_ymd_and_hms(1989, 2, 3, 0, 0, 0).unwrap()
        );
        let input_7 = "平０１．０２．０３";
        assert_eq!(
            convert(input_7),
            Utc.with_ymd_and_hms(1989, 2, 3, 0, 0, 0).unwrap()
        );
    }
    #[test]
    fn test_separated_with_kanji() {
        let input_1 = "令和1年2月3日";
        assert_eq!(
            convert(input_1),
            Utc.with_ymd_and_hms(2019, 2, 3, 0, 0, 0).unwrap()
        );
        let input_3 = "明治1年2月3日";
        assert_eq!(
            convert(input_3),
            Utc.with_ymd_and_hms(1868, 2, 3, 0, 0, 0).unwrap()
        );
        let input_4 = "大正1年2月3日";
        assert_eq!(
            convert(input_4),
            Utc.with_ymd_and_hms(1912, 2, 3, 0, 0, 0).unwrap()
        );
        let input_5 = "昭和1年2月3日";
        assert_eq!(
            convert(input_5),
            Utc.with_ymd_and_hms(1926, 2, 3, 0, 0, 0).unwrap()
        );
        let input_6 = "平成1年2月3日";
        assert_eq!(
            convert(input_6),
            Utc.with_ymd_and_hms(1989, 2, 3, 0, 0, 0).unwrap()
        );
        let input_7 = "平成１年２月３日";
        assert_eq!(
            convert(input_7),
            Utc.with_ymd_and_hms(1989, 2, 3, 0, 0, 0).unwrap()
        );
        let input_8 = "平成元年２月３日";
        assert_eq!(
            convert(input_8),
            Utc.with_ymd_and_hms(1989, 2, 3, 0, 0, 0).unwrap()
        );
    }
    #[test]
    #[ignore]
    // cargo test -- --ignored --show-output
    fn _test_perf() {
        let test_count = 10_000;
        let map = HashMap::from([
            (0, 'M'),
            (1, 'T'),
            (2, 'S'),
            (3, 'H'),
            (4, 'R'),
            (5, '明'),
            (6, '大'),
            (7, '昭'),
            (8, '平'),
            (9, '令'),
        ]);

        let start = Utc::now();
        println!("Start: {}", start);

        (0..test_count).into_iter().for_each(|i| {
            convert(&format!("{}{}.1.2", map.get(&(i % 10)).unwrap(), i % 20));
        });

        let end = Utc::now();
        let dur = end.signed_duration_since(start);
        println!("End: {}", end);
        println!(
            "Duration per sec: {}",
            test_count as f64 / (dur.num_milliseconds() as f64 / 1000.0)
        );
    }
    #[bench]
    fn bench_convert(b: &mut Bencher) {
        let test_count = 100;
        let map = HashMap::from([
            (0, 'M'),
            (1, 'T'),
            (2, 'S'),
            (3, 'H'),
            (4, 'R'),
            (5, '明'),
            (6, '大'),
            (7, '昭'),
            (8, '平'),
            (9, '令'),
        ]);
        b.iter(|| {
            (0..test_count).into_iter().for_each(|i| {
                convert(&format!("{}{}.1.2", map.get(&(i % 10)).unwrap(), i % 20));
            })
        });
    }
}
