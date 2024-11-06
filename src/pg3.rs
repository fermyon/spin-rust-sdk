//! Conversions between Rust, WIT and **Postgres** types.
//!
//! # Types
//!
//! | Rust type  | WIT (db-value)                                | Postgres type(s)             |
//! |------------|-----------------------------------------------|----------------------------- |
//! | `bool`     | boolean(bool)                                 | BOOL                         |
//! | `i16`      | int16(s16)                                    | SMALLINT, SMALLSERIAL, INT2  |
//! | `i32`      | int32(s32)                                    | INT, SERIAL, INT4            |
//! | `i64`      | int64(s64)                                    | BIGINT, BIGSERIAL, INT8      |
//! | `f32`      | floating32(float32)                           | REAL, FLOAT4                 |
//! | `f64`      | floating64(float64)                           | DOUBLE PRECISION, FLOAT8     |
//! | `String`   | str(string)                                   | VARCHAR, CHAR(N), TEXT       |
//! | `Vec<u8>`  | binary(list\<u8\>)                            | BYTEA                        |
//! | `Date`     | date(tuple<s32, u8, u8>)                      | DATE                         |
//! | `Time`     | time(tuple<u8, u8, u8, u32>)                  | TIME                         |
//! | `Datetime` | datetime(tuple<s32, u8, u8, u8, u8, u8, u32>) | TIMESTAMP                    |
//! | `Timestamp`| timestamp(s64)                                | BIGINT                       |

#[doc(inline)]
pub use super::wit::pg3::{Error as PgError, *};

/// A pg error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to deserialize [`DbValue`]
    #[error("error value decoding: {0}")]
    Decode(String),
    /// Pg query failed with an error
    #[error(transparent)]
    PgError(#[from] PgError),
}

/// A type that can be decoded from the database.
pub trait Decode: Sized {
    /// Decode a new value of this type using a [`DbValue`].
    fn decode(value: &DbValue) -> Result<Self, Error>;
}

impl<T> Decode for Option<T>
where
    T: Decode,
{
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::DbNull => Ok(None),
            v => Ok(Some(T::decode(v)?)),
        }
    }
}

impl Decode for bool {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Boolean(boolean) => Ok(*boolean),
            _ => Err(Error::Decode(format_decode_err("BOOL", value))),
        }
    }
}

impl Decode for i16 {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Int16(n) => Ok(*n),
            _ => Err(Error::Decode(format_decode_err("SMALLINT", value))),
        }
    }
}

impl Decode for i32 {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Int32(n) => Ok(*n),
            _ => Err(Error::Decode(format_decode_err("INT", value))),
        }
    }
}

impl Decode for i64 {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Int64(n) => Ok(*n),
            _ => Err(Error::Decode(format_decode_err("BIGINT", value))),
        }
    }
}

impl Decode for f32 {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Floating32(n) => Ok(*n),
            _ => Err(Error::Decode(format_decode_err("REAL", value))),
        }
    }
}

impl Decode for f64 {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Floating64(n) => Ok(*n),
            _ => Err(Error::Decode(format_decode_err("DOUBLE PRECISION", value))),
        }
    }
}

impl Decode for Vec<u8> {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Binary(n) => Ok(n.to_owned()),
            _ => Err(Error::Decode(format_decode_err("BYTEA", value))),
        }
    }
}

impl Decode for String {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Str(s) => Ok(s.to_owned()),
            _ => Err(Error::Decode(format_decode_err(
                "CHAR, VARCHAR, TEXT",
                value,
            ))),
        }
    }
}

/// Native representation of the WIT postgres Date value.
#[derive(Clone, Debug, PartialEq)]
pub struct Date(pub chrono::NaiveDate);

impl Decode for Date {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Date((year, month, day)) => {
                let naive_date =
                    chrono::NaiveDate::from_ymd_opt(*year, (*month).into(), (*day).into())
                        .ok_or_else(|| {
                            Error::Decode(format!(
                                "invalid date y={}, m={}, d={}",
                                year, month, day
                            ))
                        })?;
                Ok(Date(naive_date))
            }
            _ => Err(Error::Decode(format_decode_err("DATE", value))),
        }
    }
}

/// Native representation of the WIT postgres Time value.
#[derive(Clone, Debug, PartialEq)]
pub struct Time(pub chrono::NaiveTime);

impl Decode for Time {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Time((hour, minute, second, nanosecond)) => {
                let naive_time = chrono::NaiveTime::from_hms_nano_opt(
                    (*hour).into(),
                    (*minute).into(),
                    (*second).into(),
                    *nanosecond,
                )
                .ok_or_else(|| {
                    Error::Decode(format!(
                        "invalid time {}:{}:{}:{}",
                        hour, minute, second, nanosecond
                    ))
                })?;
                Ok(Time(naive_time))
            }
            _ => Err(Error::Decode(format_decode_err("TIME", value))),
        }
    }
}

/// Native representation of the WIT postgres DateTime value.
#[derive(Clone, Debug, PartialEq)]
pub struct DateTime(pub chrono::NaiveDateTime);

impl Decode for DateTime {
    fn decode(value: &DbValue) -> Result<Self, Error> {
        match value {
            DbValue::Datetime((year, month, day, hour, minute, second, nanosecond)) => {
                let naive_date =
                    chrono::NaiveDate::from_ymd_opt(*year, (*month).into(), (*day).into())
                        .ok_or_else(|| {
                            Error::Decode(format!(
                                "invalid date y={}, m={}, d={}",
                                year, month, day
                            ))
                        })?;
                let naive_time = chrono::NaiveTime::from_hms_nano_opt(
                    (*hour).into(),
                    (*minute).into(),
                    (*second).into(),
                    *nanosecond,
                )
                .ok_or_else(|| {
                    Error::Decode(format!(
                        "invalid time {}:{}:{}:{}",
                        hour, minute, second, nanosecond
                    ))
                })?;
                let dt = chrono::NaiveDateTime::new(naive_date, naive_time);
                Ok(DateTime(dt))
            }
            _ => Err(Error::Decode(format_decode_err("DATETIME", value))),
        }
    }
}

fn format_decode_err(types: &str, value: &DbValue) -> String {
    format!("Expected {} from the DB but got {:?}", types, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean() {
        assert!(bool::decode(&DbValue::Boolean(true)).unwrap());
        assert!(bool::decode(&DbValue::Int32(0)).is_err());
        assert!(Option::<bool>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn int16() {
        assert_eq!(i16::decode(&DbValue::Int16(0)).unwrap(), 0);
        assert!(i16::decode(&DbValue::Int32(0)).is_err());
        assert!(Option::<i16>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn int32() {
        assert_eq!(i32::decode(&DbValue::Int32(0)).unwrap(), 0);
        assert!(i32::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<i32>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn int64() {
        assert_eq!(i64::decode(&DbValue::Int64(0)).unwrap(), 0);
        assert!(i64::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<i64>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn floating32() {
        assert!(f32::decode(&DbValue::Floating32(0.0)).is_ok());
        assert!(f32::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<f32>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn floating64() {
        assert!(f64::decode(&DbValue::Floating64(0.0)).is_ok());
        assert!(f64::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<f64>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn str() {
        assert_eq!(
            String::decode(&DbValue::Str(String::from("foo"))).unwrap(),
            String::from("foo")
        );

        assert!(String::decode(&DbValue::Int32(0)).is_err());
        assert!(Option::<String>::decode(&DbValue::DbNull)
            .unwrap()
            .is_none());
    }

    #[test]
    fn binary() {
        assert!(Vec::<u8>::decode(&DbValue::Binary(vec![0, 0])).is_ok());
        assert!(Vec::<u8>::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<Vec<u8>>::decode(&DbValue::DbNull)
            .unwrap()
            .is_none());
    }

    #[test]
    fn date() {
        assert_eq!(
            Date::decode(&DbValue::Date((1, 2, 4))).unwrap(),
            Date(chrono::NaiveDate::from_ymd_opt(1, 2, 4).unwrap())
        );
        assert_ne!(
            Date::decode(&DbValue::Date((1, 2, 4))).unwrap(),
            Date(chrono::NaiveDate::from_ymd_opt(1, 2, 5).unwrap())
        );
        assert!(Option::<Date>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn time() {
        assert_eq!(
            Time::decode(&DbValue::Time((1, 2, 3, 4))).unwrap(),
            Time(chrono::NaiveTime::from_hms_nano_opt(1, 2, 3, 4).unwrap())
        );
        assert_ne!(
            Time::decode(&DbValue::Time((1, 2, 3, 4))).unwrap(),
            Time(chrono::NaiveTime::from_hms_nano_opt(1, 2, 4, 5).unwrap())
        );
        assert!(Option::<Time>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn datetime() {
        let date = chrono::NaiveDate::from_ymd_opt(1, 2, 3).unwrap();
        let mut time = chrono::NaiveTime::from_hms_nano_opt(4, 5, 6, 7).unwrap();
        assert_eq!(
            DateTime::decode(&DbValue::Datetime((1, 2, 3, 4, 5, 6, 7))).unwrap(),
            DateTime(chrono::NaiveDateTime::new(date, time))
        );

        time = chrono::NaiveTime::from_hms_nano_opt(4, 5, 6, 8).unwrap();
        assert_ne!(
            DateTime::decode(&DbValue::Datetime((1, 2, 3, 4, 5, 6, 7))).unwrap(),
            DateTime(chrono::NaiveDateTime::new(date, time))
        );
        assert!(Option::<DateTime>::decode(&DbValue::DbNull)
            .unwrap()
            .is_none());
    }
}
