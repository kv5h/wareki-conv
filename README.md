# wareki-conv

Converts Japanese Wareki date ( `JIS X 0301` ) into `ISO 8601` based format.

> [!NOTE]
>
> Wareki(和暦) refers to a calendar peculiar to Japan, where time is divided
> into periods based on an imperial era name called "Gengo" and following years.

## How is Wareki defined by `JIS X 0301`?

According to
[Wikipedia](<https://ja.wikipedia.org/wiki/ISO_8601#%E6%97%A5%E6%9C%AC_(JIS_X_0301)>),[^1]

[^1]: As of `Fri Feb 16 02:41:00 AM JST 2024`.

> 日本 (JIS X 0301)
>
> 日本産業規格 JIS X 0301（旧 JIS C 6262）があり、ISO 8601:2000 の翻訳が JIS X
> 0301:2002「情報交換のためのデータ要素及び交換形式 ― 日付及び時刻の表記」（日本
> 産業標準調査会、経済産業省）（英語題は ISO 8601 に同じ）に収められている。
>
> 規格では、「元号による日付」（和暦）が規定されている。 元号は「明」「大」「昭
> 」「平」「令」または「M」「T」「S」「H」「R」であり、これらをメタ文字 N で表す
> 。元治、慶応など明治よりも前の元号についての規定はない。
>
> 日付は、基本形式「YY.MM.DD」または拡張形式「NYY.MM.DD」で表される（元号での年
> も「YY」である）。このとき、年月日は 2 桁とし、1 桁目のゼロは省略できない。年
> 月日の区切り記号はハイフンではなくピリオドである（西暦年月日の場合は
> 、2019-06-23 のようにハイフンで区切る）。このピリオドは、基本形式においても省
> 略できない。
>
> - 例:
>   - `H16.04.01` 、 `R02.06.23`
>   - `平16.04.01` 、 `令02.06.23`
>
> 西暦を用いた場合の日付と時刻を併せた表現（例：2021-10-27T15:48:10.78）は、元号
> を用いる場合には規定されていない。つまり、R02.06.23T15:48:10.78 のような表現が
> 許されるかどうかは規定されていない。
>
> グレゴリオ暦に改暦される M06.01.01（1873-01-01）以前の和暦は、太陰太陽暦（旧暦
> ）であり、この規格の適用範囲外である。M01.01.01 から M05.12.02 までは 1868 年
> 1 月 25 日から 1872 年 12 月 31 日までを表すとされており、（ユリウス暦時代の西
> 暦日付の扱いとは異なり）グレゴリオ暦として解釈されることはない。なお、立年改元
> に基づき、明治の初日は M01.01.01 である。

For more details, See
[https://kikakurui.com/x0/X0301-2019-01.html](https://kikakurui.com/x0/X0301-2019-01.html).

## Scope of this library

Based on `JIS X 0301` definition, we will convert formats such as:

- `YY.MM.DD` (JIS X 0301 Basic format)
- `NYY.MM.DD` (JIS X 0301 Extended format where `N` is a meta character)

into:

`YYYY-MM-DD` (ISO 8601 Extended format).

Adding to the JIS X 0301 standard, some additional features are implemented for
utility.

- Accepts Full-width numbers
  - Full-width numbers are also used along with Half-width.
- Accepts Non 0-padded patterns
  - A leading 0 is generally omitted in practical use.
- Accepts first year notation in `"元年"`
  - NOTE: In Japanese calendar system, the first year of each Gengo(元号; An era
    name) is sometimes noted in `"元年"` instead of `<Era name>1年`.

Also, most commonly used format such as `平成5年2月3日 (<Gengo>YY年MM月DD日)`
can be converted.

## Example

```rust
fn test_assert(s: &str, ymd: (i32, u32, u32)) {
    assert_eq!(
        convert(s).unwrap(),
        Some(Utc.with_ymd_and_hms(ymd.0, ymd.1, ymd.2, 0, 0, 0).unwrap())
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
```

## Remark

Actually, the first day of each era is not January 1 and it differs for each
era. For example, the first day of the Heisei is January 8. This library does
not take such conditions into account and assumes that the input values are
correct.
