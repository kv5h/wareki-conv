//! Converts Wareki (JIS X 0301) based date into ISO 8601 based one

use chrono::prelude::*;
use kana::*;
use regex::Regex;

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
/// assert_eq!(to_half_width("Ｒ０１．０２．０３"), "R01.02.03");
/// assert_eq!(to_half_width("昭和１５年１２月３１日"), "昭和15年12月31日");
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
/// assert_eq!(
///     find_type("R01.02.03").unwrap(),
///     Some(DateType::JisX0301Extended)
/// )
/// ```
pub fn find_type(wareki: &str) -> Result<Option<DateType>, regex::Error> {
    let wareki_half = to_half_width(wareki);
    let elm: Vec<&str> = wareki_half.split('.').collect();
    let re_begin_with_digit = Regex::new(r"^\d")?;
    let re_begin_with_char = Regex::new(r"^(M|T|S|H|R)")?;
    let re_begin_with_kanji = Regex::new(r"^(明|大|昭|平|令)")?;
    let re_separated_with_kanji = Regex::new(r"^(明治|大正|昭和|平成|令和)\d+年\d+月\d+日")?;

    if elm.len() == 1 {
        // A minimum syntax assertion
        assert!(re_separated_with_kanji.is_match(elm.get(0).unwrap()));
        return Ok(Some(DateType::SeparatedWithKanji));
    }

    assert_eq!(elm.len(), 3);
    let date_type = match elm.get(0) {
        Some(x) if re_begin_with_digit.is_match(x) => Some(DateType::JisX0301Basic),
        Some(x) if re_begin_with_char.is_match(x) => Some(DateType::JisX0301Extended),
        Some(x) if re_begin_with_kanji.is_match(x) => Some(DateType::JisX0301ExtendedWithKanji),
        _ => None,
    };

    Ok(date_type)
}

/// Maps meta character to corresponding Gengo
///
/// ## Example
/// ```rust
/// use wareki_conv::conv::gengo_resolve;
/// use wareki_conv::conv::Gengo;
///
/// assert_eq!(gengo_resolve("R01.02.03"), Some(Gengo::Reiwa))
/// ```
pub fn gengo_resolve(wareki: &str) -> Option<Gengo> {
    let meiji = vec!['M', '明'];
    let taisho = vec!['T', '大'];
    let showa = vec!['S', '昭'];
    let heisei = vec!['H', '平'];
    #[allow(unused_variables)]
    // Currently, date with no meta attribute is mapped to this value.
    let reiwa = vec!['R', '令'];

    let wareki_half = to_half_width(wareki);
    let first_char = wareki_half.chars().nth(0);
    let gengo = match first_char {
        Some(x) if meiji.contains(&x) => Some(Gengo::Meiji),
        Some(x) if taisho.contains(&x) => Some(Gengo::Taisho),
        Some(x) if showa.contains(&x) => Some(Gengo::Showa),
        Some(x) if heisei.contains(&x) => Some(Gengo::Heisei),
        // If no meta attribute is appended, the Gengo is assumed to be the current one.
        _ => Some(Gengo::Reiwa),
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
///     era name) is sometimes noted in `"元年"` instead of `<Era name>1年`.
///
/// ## Example
/// ```rust
/// use chrono::prelude::*;
/// use wareki_conv::conv::convert;
///
/// assert_eq!(
///     convert("明治1年2月3日").unwrap(),
///     Some(Utc.with_ymd_and_hms(1868, 2, 3, 0, 0, 0).unwrap())
/// );
///
/// assert_eq!(
///     convert("明治元年2月3日").unwrap(),
///     Some(Utc.with_ymd_and_hms(1868, 2, 3, 0, 0, 0).unwrap())
/// );
///
/// assert_eq!(
///     convert("令01.02.03").unwrap(),
///     Some(Utc.with_ymd_and_hms(2019, 2, 3, 0, 0, 0).unwrap())
/// );
/// ```
///
/// ## Remark
/// Actually, the first day of each era is not January 1 and it differs for each
/// era. For example, the first day of the Heisei is January 8. This
/// library does not take such conditions into account and assumes that the
/// input values are correct.
pub fn convert(wareki: &str) -> Result<Option<DateTime<Utc>>, regex::Error> {
    let mut wareki_half = to_half_width(wareki);
    // Replace `"元年"` to `"1年"`
    wareki_half = wareki_half.replace("元", "1");
    let date_type = match find_type(&wareki_half) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let gengo = gengo_resolve(&wareki_half);
    let ymd_elements: Vec<u32>;

    match date_type {
        Some(DateType::SeparatedWithKanji) => {
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
        }
        Some(DateType::JisX0301Basic) => {
            ymd_elements = wareki_half
                .split('.')
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
            assert_eq!(ymd_elements.len(), 3);
        }
        Some(DateType::JisX0301Extended) | Some(DateType::JisX0301ExtendedWithKanji) => {
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
        None => return Ok(None),
    }

    // Converts year corresponding to Gengo
    let year = match gengo {
        Some(Gengo::Meiji) => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Meiji) - 1
        }
        Some(Gengo::Taisho) => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Taisho) - 1
        }
        Some(Gengo::Showa) => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Showa) - 1
        }
        Some(Gengo::Heisei) => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Heisei) - 1
        }
        Some(Gengo::Reiwa) => {
            ymd_elements.get(0).unwrap().clone() as i32 + Gengo::first_year(&Gengo::Reiwa) - 1
        }
        None => return Ok(None),
    };

    let date = Date::new(
        year,
        *ymd_elements.get(1).unwrap(),
        *ymd_elements.get(2).unwrap(),
    );

    let date_time: DateTime<Utc> = Utc
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 00, 00, 00)
        .unwrap();

    Ok(Some(date_time))
}
