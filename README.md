<p align="center">
  <h1 align="center">date_utils</h1>
  <p align="center">Rust library for parsing, validating, and converting dates in common formats. Developed mainly for personal use to parse dates from string and timestamps into datetime objects.</p>

  <p align="center">
      <a href="https://github.com/mattmingit/date_utils/actions">
        <img src="https://github.com/mattmingit/date_utils/actions/workflows/release.yml/badge.svg" alt="Build Status">
      </a>
      <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
      <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
   </p>
</p>

---

## ✨ Features

- ✅ Parse `YYYY-MM-DD`, `YYYY-MM`, `YYYY-QN` (SDMX-like formats)
- 🕐 Convert between `String`, `Date`, `OffsetDateTime`, and Unix `timestamp`
- 🌐 Handle local and UTC offsets
- 🔐 Validate that dates are not in the future
- 🚫 Typed error handling via `thiserror`

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
date_utils = { git = "https://github.com/your-username/date_utils" }
```

## 📥 Binary Releases

Precompiled binaries are available on the [ releases page ](https://github.com/mattmingit/date_utils/releases) for major platforms.

## 📚 API Overview

| Function                            | Description                                                              |
| ----------------------------------- | ------------------------------------------------------------------------ |
| `parse_to_datetime`                 | Converts `YYYY-MM-DD` to `OffsetDateTime` with optional UTC/local offset |
| `timestamp_to_datetime`             | Converts Unix timestamp (`i64`) to `OffsetDateTime`                      |
| `datetime_to_date`                  | Extracts `Date` from an `OffsetDateTime`                                 |
| `timestamp_to_offset`               | Converts seconds (`i32`) to a `UtcOffset`                                |
| `parse_response_string_to_datetime` | Parses `YYYY-MM-DD`, `YYYY-MM`, or `YYYY-QN` into `OffsetDateTime`       |

## ❗ Error Handling

All functions return rich, descriptive error types through the DateTimeError enum, including:

- InvalidDateFormat — when date format doesn't match the expected pattern

- InvalidTimestamp — if timestamp conversion fails

- InvalidOffset — offset conversion is out of bounds

- InvalidTimeComponent — unsupported time component (e.g., bad quarter)

- DateInFuture — when a date is in the future but shouldn't be

- ParseError — fallback for general parsing issues

## 📅 Supported Formats

| Format       | Description                           | Example      |
| ------------ | ------------------------------------- | ------------ |
| `YYYY-MM-DD` | Full date                             | `2024-05-31` |
| `YYYY-MM`    | Year-month (defaults to 1st of month) | `2024-05`    |
| `YYYY-QN`    | Quarterly format (starts quarter)     | `2024-Q2`    |

🔧 Usage Example

```rust
use date_utils::{parse_to_datetime, DateType, OffsetType};

fn main() -> Result<(), date_utils::DateTimeError> {
    let datetime = parse_to_datetime("2025-01-01", DateType::Start, OffsetType::Utc)?;
    println!("Parsed datetime: {}", datetime);
    Ok(())
}
```

<p align="center"><em>Built with ❤️ and Rust — reliable time handling made simple.</em></p>
