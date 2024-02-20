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
    use std::collections::HashMap;
    use test::Bencher;

    #[test]
    fn assert() {
        fn test_assert(s: &str, ymd: (i32, u32, u32)) {
            assert_eq!(
                convert(s).unwrap().unwrap(),
                Utc.with_ymd_and_hms(ymd.0, ymd.1, ymd.2, 0, 0, 0).unwrap()
            )
        }

        // DateType::JisX0301Basic
        test_assert("01.02.03", (2019, 2, 3));
        test_assert("1.2.3", (2019, 2, 3));
        test_assert("10.02.03", (2028, 2, 3));
        test_assert("06.2.3", (2024, 2, 3));
        test_assert("０６．０２．０３", (2024, 2, 3));

        // DateType::JisX0301Extended
        test_assert("R01.02.03", (2019, 2, 3));
        test_assert("R10.2.3", (2028, 2, 3));
        test_assert("M01.02.03", (1868, 2, 3));
        test_assert("M45.2.3", (1912, 2, 3));
        test_assert("T01.02.03", (1912, 2, 3));
        test_assert("S01.2.3", (1926, 2, 3));
        test_assert("H01.02.03", (1989, 2, 3));
        test_assert("Ｈ０１．０２．０３", (1989, 2, 3));

        // DateType::JisX0301ExtendedWithKanji
        test_assert("令01.02.03", (2019, 2, 3));
        test_assert("令1.2.3", (2019, 2, 3));
        test_assert("明01.02.03", (1868, 2, 3));
        test_assert("大01.2.3", (1912, 2, 3));
        test_assert("昭01.02.03", (1926, 2, 3));
        test_assert("平01.2.3", (1989, 2, 3));
        test_assert("平０１．０２．０３", (1989, 2, 3));

        // DateType::SeparatedWithKanji
        test_assert("令和1年2月3日", (2019, 2, 3));
        test_assert("明治1年2月3日", (1868, 2, 3));
        test_assert("大正1年2月3日", (1912, 2, 3));
        test_assert("昭和1年2月3日", (1926, 2, 3));
        test_assert("平成1年2月3日", (1989, 2, 3));
        test_assert("平成１年２月３日", (1989, 2, 3));
        test_assert("平成元年２月３日", (1989, 2, 3));
    }

    #[test]
    #[ignore]
    // cargo test -- --ignored --show-output
    fn _test_perf() {
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

        let start = Utc::now();
        println!("Start: {}", start);

        (0..test_count).into_iter().for_each(|i| {
            convert(&format!("{}{}.1.2", map.get(&(i % 10)).unwrap(), i % 20)).unwrap();
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
                convert(&format!("{}{}.1.2", map.get(&(i % 10)).unwrap(), i % 20)).unwrap();
            })
        });
    }
}
