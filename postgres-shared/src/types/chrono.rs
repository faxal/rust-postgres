extern crate chrono;

use postgres_protocol::types;
use self::chrono::{Duration, NaiveDate, NaiveTime, NaiveDateTime, DateTime, UTC, Local,
                   FixedOffset};
use std::error::Error;

use types::{FromSql, ToSql, IsNull, Type};

fn base() -> NaiveDateTime {
    NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0)
}

impl FromSql for NaiveDateTime {
    fn from_sql(_: &Type,
                raw: &[u8])
                -> Result<NaiveDateTime, Box<Error + Sync + Send>> {
        let t = types::timestamp_from_sql(raw)?;
        Ok(base() + Duration::microseconds(t))
    }

    accepts!(Type::Timestamp);
}

impl ToSql for NaiveDateTime {
    fn to_sql(&self,
              _: &Type,
              w: &mut Vec<u8>)
              -> Result<IsNull, Box<Error + Sync + Send>> {
        let time = match self.signed_duration_since(base()).num_microseconds() {
            Some(time) => time,
            None => return Err("value too large to transmit".into()),
        };
        types::timestamp_to_sql(time, w);
        Ok(IsNull::No)
    }

    accepts!(Type::Timestamp);
    to_sql_checked!();
}

impl FromSql for DateTime<UTC> {
    fn from_sql(type_: &Type,
                raw: &[u8])
                -> Result<DateTime<UTC>, Box<Error + Sync + Send>> {
        let naive = NaiveDateTime::from_sql(type_, raw)?;
        Ok(DateTime::from_utc(naive, UTC))
    }

    accepts!(Type::Timestamptz);
}

impl ToSql for DateTime<UTC> {
    fn to_sql(&self,
              type_: &Type,
              w: &mut Vec<u8>)
              -> Result<IsNull, Box<Error + Sync + Send>> {
        self.naive_utc().to_sql(type_, w)
    }

    accepts!(Type::Timestamptz);
    to_sql_checked!();
}

impl FromSql for DateTime<Local> {
    fn from_sql(type_: &Type,
                raw: &[u8])
                -> Result<DateTime<Local>, Box<Error + Sync + Send>> {
        let utc = DateTime::<UTC>::from_sql(type_, raw)?;
        Ok(utc.with_timezone(&Local))
    }

    accepts!(Type::Timestamptz);
}

impl ToSql for DateTime<Local> {
    fn to_sql(&self,
              type_: &Type,
              mut w: &mut Vec<u8>)
              -> Result<IsNull, Box<Error + Sync + Send>> {
        self.with_timezone(&UTC).to_sql(type_, w)
    }

    accepts!(Type::Timestamptz);
    to_sql_checked!();
}

impl FromSql for DateTime<FixedOffset> {
    fn from_sql(type_: &Type,
                raw: &[u8])
                -> Result<DateTime<FixedOffset>, Box<Error + Sync + Send>> {
        let utc = DateTime::<UTC>::from_sql(type_, raw)?;
        Ok(utc.with_timezone(&FixedOffset::east(0)))
    }

    accepts!(Type::Timestamptz);
}

impl ToSql for DateTime<FixedOffset> {
    fn to_sql(&self,
              type_: &Type,
              w: &mut Vec<u8>)
              -> Result<IsNull, Box<Error + Sync + Send>> {
        self.with_timezone(&UTC).to_sql(type_, w)
    }

    accepts!(Type::Timestamptz);
    to_sql_checked!();
}

impl FromSql for NaiveDate {
    fn from_sql(_: &Type,
                raw: &[u8])
                -> Result<NaiveDate, Box<Error + Sync + Send>> {
        let jd = types::date_from_sql(raw)?;
        Ok(base().date() + Duration::days(jd as i64))
    }

    accepts!(Type::Date);
}

impl ToSql for NaiveDate {
    fn to_sql(&self,
              _: &Type,
              w: &mut Vec<u8>)
              -> Result<IsNull, Box<Error + Sync + Send>> {
        let jd = self.signed_duration_since(base().date()).num_days();
        if jd > i32::max_value() as i64 || jd < i32::min_value() as i64 {
            return Err("value too large to transmit".into());
        }

        types::date_to_sql(jd as i32, w);
        Ok(IsNull::No)
    }

    accepts!(Type::Date);
    to_sql_checked!();
}

impl FromSql for NaiveTime {
    fn from_sql(_: &Type,
                raw: &[u8])
                -> Result<NaiveTime, Box<Error + Sync + Send>> {
        let usec = types::time_from_sql(raw)?;
        Ok(NaiveTime::from_hms(0, 0, 0) + Duration::microseconds(usec))
    }

    accepts!(Type::Time);
}

impl ToSql for NaiveTime {
    fn to_sql(&self,
              _: &Type,
              w: &mut Vec<u8>)
              -> Result<IsNull, Box<Error + Sync + Send>> {
        let delta = self.signed_duration_since(NaiveTime::from_hms(0, 0, 0));
        let time = match delta.num_microseconds() {
            Some(time) => time,
            None => return Err("value too large to transmit".into()),
        };
        types::time_to_sql(time, w);
        Ok(IsNull::No)
    }

    accepts!(Type::Time);
    to_sql_checked!();
}
