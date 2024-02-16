//! Converts Japanese Wareki date into ISO based format.

use chrono::prelude::*;
use chrono::DateTime;
use chrono::Utc;
use kana::*;
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
/// |        `SeparatedWithKanji`         | `令和1年2月3日` |
///
/// ## Remark
/// JIS X 0301 requires each value (year, month and day) to be padded with 0
/// if it is 1-digit value.
///
/// Ref: <https://kikakurui.com/x0/X0301-2019-01.html>
///
/// This library also accepts un-padded value because 0-padding is not always
/// complied even in official documents.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, PartialOrd, Ord)]
pub enum DateType {
    JisX0301Basic,
    JisX0301Extended,
    JisX0301ExtendedWithKanji,
    SeparatedWithKanji,
}

/// Normalize input data
///
/// Japanese character has Zenkaku(Full-width) and Hankaku(Half-width) mode. For
/// example,
///
/// | Zenkaku | Hankaku |
/// | :-----: | :-----: |
/// |  `１`   |   `1`   |
/// |   `Ａ`   |   `A`  |
///
/// Input data should be normalized beforehand because both are often used in
/// common.
///
/// ## Example
/// ```rust
/// use wareki_conv::conv::to_half_width;
///
/// assert_eq!(to_half_width("Ｒ０１．０２．０３"), "R01.02.03")
/// ```
pub fn to_half_width(input: &str) -> String {
    wide2ascii(input)
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
pub fn find_type(wareki: &str) -> DateType {
    let wareki_half = to_half_width(wareki);
    let elm: Vec<&str> = wareki_half.split('.').collect();

    if elm.len() == 1 {
        return DateType::SeparatedWithKanji;
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

/// Maps meta character to corresponding Gengo
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

    let wareki_half = to_half_width(wareki);
    let first_char = wareki_half.chars().nth(0).unwrap();
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

/// Converts Wareki (JIS X 0301) based date into ISO based one
///
/// Adding to the JIS X 0301 standard, some additional features are
/// implemented for utility. such as:
/// * Accepts Full-width numbers
///   * Full-width numbers are also used along with Half-width.
/// * Accepts Non 0-padded patterns
///   * A leading 0 is generally omitted in practical use.
/// * Accepts first year notation in `"元年"`
///   * NOTE: In Japanese calendar system, the first year of each Gengo(元号; An
///     era name) is sometimes noted in `"元年"` instead of `<Era name>1`.
///
/// ## Example
/// ```rust
/// use chrono::prelude::*;
/// use chrono::DateTime;
/// use chrono::Utc;
/// use wareki_conv::conv::convert;
///
/// assert_eq!(
///     convert("明治1年2月3日"),
///     Utc.with_ymd_and_hms(1868, 2, 3, 0, 0, 0).unwrap()
/// );
///
/// assert_eq!(
///     convert("明治元年2月3日"),
///     Utc.with_ymd_and_hms(1868, 2, 3, 0, 0, 0).unwrap()
/// );
///
/// assert_eq!(
///     convert("令01.02.03"),
///     Utc.with_ymd_and_hms(2019, 2, 3, 0, 0, 0).unwrap()
/// );
/// ```
///
/// ## Remark
/// Actually, the first day of each era is not January 1 and it differs for each
/// era. For example, the first day of the Heisei is January 8. This
/// library does not take such conditions into account and assumes that the
/// input values are correct.
pub fn convert(wareki: &str) -> DateTime<Utc> {
    let mut wareki_half = to_half_width(wareki);
    // Replace `"元年"` to `"1年"`
    wareki_half = wareki_half.replace("元", "1");
    let date_type = find_type(&wareki_half);
    let gengo = gengo_resolve(&wareki_half);
    let ymd_elements: Vec<u32>;
    if date_type == DateType::SeparatedWithKanji {
        let tmp: String = wareki_half
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
        ymd_elements = wareki_half
            .split('.')
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(ymd_elements.len(), 3);
    } else {
        ymd_elements = wareki_half
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
