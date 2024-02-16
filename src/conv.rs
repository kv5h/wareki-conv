//! Converts Japanese Wareki date into ISO based format.
//!

use chrono::prelude::*;
use chrono::DateTime;
use chrono::Utc;
use log::error;
use regex::Regex;
use std::process;

const START_YEAR_OF_MEIJI: i32 = 1868;
const START_YEAR_OF_TAISHO: i32 = 1912;
const START_YEAR_OF_SHOWA: i32 = 1926;
const START_YEAR_OF_HEISEI: i32 = 1989;
const START_YEAR_OF_REIWA: i32 = 2019;

/// Struct for date
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Date {
    /// Year
    pub year: i32,
    /// Month
    pub month: u32,
    /// Day
    pub day: u32,
}

impl Date {
    /// Returns new Date
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Returns year
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Returns month
    pub fn month(&self) -> u32 {
        self.month
    }

    /// Returns day
    pub fn day(&self) -> u32 {
        self.day
    }
}

/// List of Gengo
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Gengo {
    /// Meiji
    Meiji,
    /// Taisho
    Taisho,
    /// Showa
    Showa,
    /// Heisei
    Heisei,
    /// Reiwa
    Reiwa,
}

impl Gengo {
    /// Returns the first year of the Gengo
    ///
    /// ```rust
    /// use wareki_conv::conv::Gengo;
    ///
    /// assert_eq!(Gengo::Meiji.first_year(), 1868)
    /// ```
    pub const fn first_year(&self) -> i32 {
        match *self {
            Gengo::Meiji => START_YEAR_OF_MEIJI,
            Gengo::Taisho => START_YEAR_OF_TAISHO,
            Gengo::Showa => START_YEAR_OF_SHOWA,
            Gengo::Heisei => START_YEAR_OF_HEISEI,
            Gengo::Reiwa => START_YEAR_OF_REIWA,
        }
    }

    /// Get the name of the Gengo
    ///
    /// ```rust
    /// use wareki_conv::conv::Gengo;
    ///
    /// assert_eq!(Gengo::Meiji.name(), "Meiji")
    /// ```
    pub const fn name(&self) -> &'static str {
        match *self {
            Gengo::Meiji => "Meiji",
            Gengo::Taisho => "Taisho",
            Gengo::Showa => "Showa",
            Gengo::Heisei => "Heisei",
            Gengo::Reiwa => "Reiwa",
        }
    }
}

/// Date type
///
/// Each type has following format:
///
/// |                Type                 | Format Example  |
/// | :---------------------------------- | :-------------- |
/// |           `JisX0301Basic`           |   `01.02.03`    |
/// |         `JisX0301Extended`          |   `R01.02.03`   |
/// |     `JisX0301ExtendedWithKanji`     |  `令01.02.03`   |
/// |        `SeparetedWithKanji`         | `令和1年2月3日` |
///
/// ## Remark
/// JIS X 0301 requires each value (year, month and day) must be padded with 0
/// if it is 1-digit velue.
///
/// Ref: https://kikakurui.com/x0/X0301-2019-02.html
///
/// This library also accepts unpadded value because 0-padding is not always
/// complied even in an official document.
///
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, PartialOrd, Ord)]
pub enum DateType {
    JisX0301Basic,
    JisX0301Extended,
    JisX0301ExtendedWithKanji,
    SeparetedWithKanji,
}

/// Finds structure type by pattern matching
///
/// ## Example:
/// ```rust
/// use wareki_conv::conv::find_type;
/// use wareki_conv::conv::DateType;
///
/// assert_eq!(find_type("R01.02.03"), DateType::JisX0301Extended)
/// ```
///
pub fn find_type(wareki: &str) -> DateType {
    let elm: Vec<&str> = wareki.split('.').collect();

    if elm.len() == 1 {
        return DateType::SeparetedWithKanji;
    }

    assert_eq!(elm.len(), 3);

    let re_begin_with_digit = Regex::new(r"^\d").unwrap();
    let re_begin_with_char = Regex::new(r"^(M|T|S|H|R)").unwrap();
    let re_begin_with_kanji = Regex::new(r"^(明|大|昭|平|令)").unwrap();
    let date_type = match elm.get(0).unwrap() {
        x if re_begin_with_digit.is_match(x) => DateType::JisX0301Basic,
        x if re_begin_with_char.is_match(x) => DateType::JisX0301Extended,
        x if re_begin_with_kanji.is_match(x) => DateType::JisX0301ExtendedWithKanji,
        _ => {
            error!("Failed to convert {}", elm.get(0).unwrap());
            process::exit(1)
        }
    };

    date_type
}

/// Maps meta charactor to corresponding Gengo
///
/// ## Example
/// ```rust
/// use wareki_conv::conv::gengo_resolve;
/// use wareki_conv::conv::Gengo;
///
/// assert_eq!(gengo_resolve("R01.02.03"), Gengo::Reiwa)
/// ```
pub fn gengo_resolve(wareki: &str) -> Gengo {
    let meiji = vec!['M', '明'];
    let taisho = vec!['T', '大'];
    let showa = vec!['S', '昭'];
    let heisei = vec!['H', '平'];

    let first_char = wareki.chars().nth(0).unwrap();
    // If no meta attribute is appended, the Gengo is assumed to be the current one.
    let gengo = match first_char {
        x if meiji.contains(&x) => Gengo::Meiji,
        x if taisho.contains(&x) => Gengo::Taisho,
        x if showa.contains(&x) => Gengo::Showa,
        x if heisei.contains(&x) => Gengo::Heisei,
        _ => Gengo::Reiwa,
    };

    gengo
}

/// Converts Wareki (JIS X 0301) based date into ISO based one.
///
/// ## Example
/// ```rust
///
/// ```
pub fn convert(wareki: &str) -> DateTime<Utc> {
    let date_type = find_type(wareki);
    let gengo = gengo_resolve(wareki);
    let ymd_elements: Vec<u32>;
    if date_type == DateType::SeparetedWithKanji {
        let tmp: String = wareki
            .chars()
            .skip(2)
            .filter(|x| x != &'日')
            .map(|x| if x.is_ascii_digit() { x } else { '.' })
            .collect();
        ymd_elements = tmp
            .split('.')
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(ymd_elements.len(), 3);
    } else if date_type == DateType::JisX0301Basic {
        ymd_elements = wareki
            .split('.')
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(ymd_elements.len(), 3);
    } else {
        ymd_elements = wareki
            .chars()
            .skip(1)
            .collect::<String>()
            .split('.')
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(ymd_elements.len(), 3);
    }

    // Converts year corresponding to Gengo
    let year = match gengo {
        Gengo::Meiji => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Meiji) - 1
        }
        Gengo::Taisho => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Taisho) - 1
        }
        Gengo::Showa => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Showa) - 1
        }
        Gengo::Heisei => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Heisei) - 1
        }
        Gengo::Reiwa => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Reiwa) - 1
        }
    };

    let date = Date::new(
        year,
        *ymd_elements.get(1).unwrap(),
        *ymd_elements.get(2).unwrap(),
    );

    let date_time: DateTime<Utc> = Utc
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 00, 00, 00)
        .unwrap();

    date_time
}
