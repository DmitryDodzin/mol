use std::convert::TryInto;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{
  de::{self, Deserializer},
  Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct WrappedSource<T> {
  from: T,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FlexibleDate {
  String(String),
  Int(i64),
}

impl TryInto<DateTime<Utc>> for FlexibleDate {
  type Error = chrono::format::ParseError;
  fn try_into(self) -> Result<DateTime<Utc>, Self::Error> {
    match self {
      FlexibleDate::String(value) => DateTime::parse_from_rfc3339(&value).map(|date| date.into()),
      FlexibleDate::Int(value) => Ok(DateTime::from_utc(
        NaiveDateTime::from_timestamp(value, 0),
        Utc,
      )),
    }
  }
}

pub fn parse_flexible_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let option = FlexibleDate::deserialize(deserializer)?;

  option.try_into().map_err(de::Error::custom)
}

pub fn parse_flexible_timestamp_option<'de, D>(
  deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
  D: Deserializer<'de>,
{
  let raw = Option::<FlexibleDate>::deserialize(deserializer)?;

  println!("{:?}", raw);

  raw
    .map(|value| value.try_into().map_err(de::Error::custom))
    .transpose()
}
