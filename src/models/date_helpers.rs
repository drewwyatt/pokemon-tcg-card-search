use chrono::format::{DelayedFormat, ParseResult, StrftimeItems};
use chrono::{NaiveDate, NaiveDateTime};

/// Types implementing this trait can convert to and from a string
/// using a format string.
pub trait DateFormatType {
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>>;
    fn parse_from_str(s: &str, fmt: &str) -> ParseResult<Self>
    where
        Self: Sized;
}

impl DateFormatType for NaiveDateTime {
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.format(fmt)
    }
    fn parse_from_str(s: &str, fmt: &str) -> ParseResult<Self>
    where
        Self: Sized,
    {
        Self::parse_from_str(s, fmt)
    }
}

impl DateFormatType for NaiveDate {
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.format(fmt)
    }
    fn parse_from_str(s: &str, fmt: &str) -> ParseResult<Self>
    where
        Self: Sized,
    {
        Self::parse_from_str(s, fmt)
    }
}
