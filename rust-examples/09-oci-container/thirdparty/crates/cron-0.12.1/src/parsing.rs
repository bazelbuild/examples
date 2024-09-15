use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace0};
use nom::combinator::{all_consuming, eof, map, map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::IResult;

use std::convert::TryFrom;
use std::str::{self, FromStr};

use crate::error::{Error, ErrorKind};
use crate::schedule::{ScheduleFields, Schedule};
use crate::specifier::*;
use crate::time_unit::*;
use crate::ordinal::*;

impl FromStr for Schedule {
    type Err = Error;
    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        match schedule(expression) {
            Ok((_, schedule_fields)) => {
                Ok(Schedule::new(String::from(expression), schedule_fields))
            } // Extract from nom tuple
            Err(_) => Err(ErrorKind::Expression("Invalid cron expression.".to_owned()).into()), //TODO: Details
        }
    }
}
impl TryFrom<&str> for Schedule {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub specifiers: Vec<RootSpecifier>, // TODO: expose iterator?
}

trait FromField
where
    Self: Sized,
{
    //TODO: Replace with std::convert::TryFrom when stable
    fn from_field(field: Field) -> Result<Self, Error>;
}

impl<T> FromField for T
where
    T: TimeUnitField,
{
    fn from_field(field: Field) -> Result<T, Error> {
        if field.specifiers.len() == 1 && 
            field.specifiers.get(0).unwrap() == &RootSpecifier::from(Specifier::All) 
            { return Ok(T::all()); }
        let mut ordinals = OrdinalSet::new(); 
        for specifier in field.specifiers {
            let specifier_ordinals: OrdinalSet = T::ordinals_from_root_specifier(&specifier)?;
            for ordinal in specifier_ordinals {
                ordinals.insert(T::validate_ordinal(ordinal)?);
            }
        }
        Ok(T::from_ordinal_set(ordinals))
    }
}

fn ordinal(i: &str) -> IResult<&str, u32> {
    map_res(delimited(multispace0, digit1, multispace0), u32::from_str)(i)
}

fn name(i: &str) -> IResult<&str, String> {
    map(
        delimited(multispace0, alpha1, multispace0),
        ToOwned::to_owned,
    )(i)
}

fn point(i: &str) -> IResult<&str, Specifier> {
    let (i, o) = ordinal(i)?;
    Ok((i, Specifier::Point(o)))
}

fn named_point(i: &str) -> IResult<&str, RootSpecifier> {
    let (i, n) = name(i)?;
    Ok((i, RootSpecifier::NamedPoint(n)))
}

fn period(i: &str) -> IResult<&str, RootSpecifier> {
    map(
        separated_pair(specifier, tag("/"), ordinal),
        |(start, step)| RootSpecifier::Period(start, step),
    )(i)
}

fn period_with_any(i: &str) -> IResult<&str, RootSpecifier> {
    map(
        separated_pair(specifier_with_any, tag("/"), ordinal),
        |(start, step)| RootSpecifier::Period(start, step),
    )(i)
}

fn range(i: &str) -> IResult<&str, Specifier> {
    map(
        separated_pair(ordinal, tag("-"), ordinal),
        |(start, end)| Specifier::Range(start, end),
    )(i)
}

fn named_range(i: &str) -> IResult<&str, Specifier> {
    map(separated_pair(name, tag("-"), name), |(start, end)| {
        Specifier::NamedRange(start, end)
    })(i)
}

fn all(i: &str) -> IResult<&str, Specifier> {
    let (i, _) = tag("*")(i)?;
    Ok((i, Specifier::All))
}

fn any(i: &str) -> IResult<&str, Specifier> {
    let (i, _) = tag("?")(i)?;
    Ok((i, Specifier::All))
}

fn specifier(i: &str) -> IResult<&str, Specifier> {
    alt((all, range, point, named_range))(i)
}

fn specifier_with_any(i: &str) -> IResult<&str, Specifier> {
    alt((any, specifier))(i)
}

fn root_specifier(i: &str) -> IResult<&str, RootSpecifier> {
    alt((period, map(specifier, RootSpecifier::from), named_point))(i)
}

fn root_specifier_with_any(i: &str) -> IResult<&str, RootSpecifier> {
    alt((
        period_with_any,
        map(specifier_with_any, RootSpecifier::from),
        named_point,
    ))(i)
}

fn root_specifier_list(i: &str) -> IResult<&str, Vec<RootSpecifier>> {
    let list = separated_list1(tag(","), root_specifier);
    let single_item = map(root_specifier, |spec| vec![spec]);
    delimited(multispace0, alt((list, single_item)), multispace0)(i)
}

fn root_specifier_list_with_any(i: &str) -> IResult<&str, Vec<RootSpecifier>> {
    let list = separated_list1(tag(","), root_specifier_with_any);
    let single_item = map(root_specifier_with_any, |spec| vec![spec]);
    delimited(multispace0, alt((list, single_item)), multispace0)(i)
}

fn field(i: &str) -> IResult<&str, Field> {
    let (i, specifiers) = root_specifier_list(i)?;
    Ok((i, Field { specifiers }))
}

fn field_with_any(i: &str) -> IResult<&str, Field> {
    let (i, specifiers) = root_specifier_list_with_any(i)?;
    Ok((i, Field { specifiers }))
}

fn shorthand_yearly(i: &str) -> IResult<&str, ScheduleFields> {
    let (i, _) = tag("@yearly")(i)?;
    let fields = ScheduleFields::new(
        Seconds::from_ordinal(0),
        Minutes::from_ordinal(0),
        Hours::from_ordinal(0),
        DaysOfMonth::from_ordinal(1),
        Months::from_ordinal(1),
        DaysOfWeek::all(),
        Years::all(),
    );
    Ok((i, fields))
}

fn shorthand_monthly(i: &str) -> IResult<&str, ScheduleFields> {
    let (i, _) = tag("@monthly")(i)?;
    let fields = ScheduleFields::new(
        Seconds::from_ordinal(0),
        Minutes::from_ordinal(0),
        Hours::from_ordinal(0),
        DaysOfMonth::from_ordinal(1),
        Months::all(),
        DaysOfWeek::all(),
        Years::all(),
    );
    Ok((i, fields))
}

fn shorthand_weekly(i: &str) -> IResult<&str, ScheduleFields> {
    let (i, _) = tag("@weekly")(i)?;
    let fields = ScheduleFields::new(
        Seconds::from_ordinal(0),
        Minutes::from_ordinal(0),
        Hours::from_ordinal(0),
        DaysOfMonth::all(),
        Months::all(),
        DaysOfWeek::from_ordinal(1),
        Years::all(),
    );
    Ok((i, fields))
}

fn shorthand_daily(i: &str) -> IResult<&str, ScheduleFields> {
    let (i, _) = tag("@daily")(i)?;
    let fields = ScheduleFields::new(
        Seconds::from_ordinal(0),
        Minutes::from_ordinal(0),
        Hours::from_ordinal(0),
        DaysOfMonth::all(),
        Months::all(),
        DaysOfWeek::all(),
        Years::all(),
    );
    Ok((i, fields))
}

fn shorthand_hourly(i: &str) -> IResult<&str, ScheduleFields> {
    let (i, _) = tag("@hourly")(i)?;
    let fields = ScheduleFields::new(
        Seconds::from_ordinal(0),
        Minutes::from_ordinal(0),
        Hours::all(),
        DaysOfMonth::all(),
        Months::all(),
        DaysOfWeek::all(),
        Years::all(),
    );
    Ok((i, fields))
}

fn shorthand(i: &str) -> IResult<&str, ScheduleFields> {
    let keywords = alt((
        shorthand_yearly,
        shorthand_monthly,
        shorthand_weekly,
        shorthand_daily,
        shorthand_hourly,
    ));
    delimited(multispace0, keywords, multispace0)(i)
}

fn longhand(i: &str) -> IResult<&str, ScheduleFields> {
    let seconds = map_res(field, Seconds::from_field);
    let minutes = map_res(field, Minutes::from_field);
    let hours = map_res(field, Hours::from_field);
    let days_of_month = map_res(field_with_any, DaysOfMonth::from_field);
    let months = map_res(field, Months::from_field);
    let days_of_week = map_res(field_with_any, DaysOfWeek::from_field);
    let years = opt(map_res(field, Years::from_field));
    let fields = tuple((seconds, minutes, hours, days_of_month, months, days_of_week, years));

    map(
        terminated(fields, eof),
        |(seconds, minutes, hours, days_of_month, months, days_of_week, years)| {
            let years = years.unwrap_or_else(Years::all);
            ScheduleFields::new(
                seconds,
                minutes,
                hours,
                days_of_month,
                months,
                days_of_week,
                years,
            )
        },
    )(i)
}

fn schedule(i: &str) -> IResult<&str, ScheduleFields> {
    all_consuming(alt((shorthand, longhand)))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nom_valid_number() {
        let expression = "1997";
        point(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_point() {
        let expression = "a";
        assert!(point(expression).is_err());
    }

    #[test]
    fn test_nom_valid_named_point() {
        let expression = "WED";
        named_point(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_named_point() {
        let expression = "8";
        assert!(named_point(expression).is_err());
    }

    #[test]
    fn test_nom_valid_period() {
        let expression = "1/2";
        period(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_period() {
        let expression = "Wed/4";
        assert!(period(expression).is_err());
    }

    #[test]
    fn test_nom_valid_number_list() {
        let expression = "1,2";
        field(expression).unwrap();
        field_with_any(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_number_list() {
        let expression = ",1,2";
        assert!(field(expression).is_err());
        assert!(field_with_any(expression).is_err());
    }

    #[test]
    fn test_nom_field_with_any_valid_any() {
        let expression = "?";
        field_with_any(expression).unwrap();
    }

    #[test]
    fn test_nom_field_invalid_any() {
        let expression = "?";
        assert!(field(expression).is_err());
    }

    #[test]
    fn test_nom_valid_range_field() {
        let expression = "1-4";
        range(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_period_all() {
        let expression = "*/2";
        period(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_period_range() {
        let expression = "10-20/2";
        period(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_period_named_range() {
        let expression = "Mon-Thurs/2";
        period(expression).unwrap();

        let expression = "February-November/2";
        period(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_period_point() {
        let expression = "10/2";
        period(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_period_any() {
        let expression = "?/2";
        assert!(period(expression).is_err());
    }

    #[test]
    fn test_nom_invalid_period_named_point() {
        let expression = "Tues/2";
        assert!(period(expression).is_err());

        let expression = "February/2";
        assert!(period(expression).is_err());
    }

    #[test]
    fn test_nom_invalid_period_specifier_range() {
        let expression = "10-12/*";
        assert!(period(expression).is_err());
    }

    #[test]
    fn test_nom_valid_period_with_any_all() {
        let expression = "*/2";
        period_with_any(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_period_with_any_range() {
        let expression = "10-20/2";
        period_with_any(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_period_with_any_named_range() {
        let expression = "Mon-Thurs/2";
        period_with_any(expression).unwrap();

        let expression = "February-November/2";
        period_with_any(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_period_with_any_point() {
        let expression = "10/2";
        period_with_any(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_period_with_any_any() {
        let expression = "?/2";
        period_with_any(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_period_with_any_named_point() {
        let expression = "Tues/2";
        assert!(period_with_any(expression).is_err());

        let expression = "February/2";
        assert!(period_with_any(expression).is_err());
    }

    #[test]
    fn test_nom_invalid_period_with_any_specifier_range() {
        let expression = "10-12/*";
        assert!(period_with_any(expression).is_err());
    }

    #[test]
    fn test_nom_invalid_range_field() {
        let expression = "-4";
        assert!(range(expression).is_err());
    }

    #[test]
    fn test_nom_valid_named_range_field() {
        let expression = "TUES-THURS";
        named_range(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_named_range_field() {
        let expression = "3-THURS";
        assert!(named_range(expression).is_err());
    }

    #[test]
    fn test_nom_valid_schedule() {
        let expression = "* * * * * *";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_schedule() {
        let expression = "* * * *";
        assert!(schedule(expression).is_err());
    }

    #[test]
    fn test_nom_valid_seconds_list() {
        let expression = "0,20,40 * * * * *";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_seconds_range() {
        let expression = "0-40 * * * * *";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_seconds_mix() {
        let expression = "0-5,58 * * * * *";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_seconds_range() {
        let expression = "0-65 * * * * *";
        assert!(schedule(expression).is_err());
    }

    #[test]
    fn test_nom_invalid_seconds_list() {
        let expression = "103,12 * * * * *";
        assert!(schedule(expression).is_err());
    }

    #[test]
    fn test_nom_invalid_seconds_mix() {
        let expression = "0-5,102 * * * * *";
        assert!(schedule(expression).is_err());
    }

    #[test]
    fn test_nom_valid_days_of_week_list() {
        let expression = "* * * * * MON,WED,FRI";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_days_of_week_list() {
        let expression = "* * * * * MON,TURTLE";
        assert!(schedule(expression).is_err());
    }

    #[test]
    fn test_nom_valid_days_of_week_range() {
        let expression = "* * * * * MON-FRI";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_days_of_week_range() {
        let expression = "* * * * * BEAR-OWL";
        assert!(schedule(expression).is_err());
    }

    #[test]
    fn test_nom_invalid_period_with_range_specifier() {
        let expression = "10-12/10-12 * * * * ?";
        assert!(schedule(expression).is_err());
    }

    #[test]
    fn test_nom_valid_days_of_month_any() {
        let expression = "* * * ? * *";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_days_of_week_any() {
        let expression = "* * * * * ?";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_days_of_month_any_days_of_week_specific() {
        let expression = "* * * ? * Mon,Thu";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_days_of_week_any_days_of_month_specific() {
        let expression = "* * * 1,2 * ?";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_valid_dom_and_dow_any() {
        let expression = "* * * ? * ?";
        schedule(expression).unwrap();
    }

    #[test]
    fn test_nom_invalid_other_fields_any() {
        let expression = "? * * * * *";
        assert!(schedule(expression).is_err());

        let expression = "* ? * * * *";
        assert!(schedule(expression).is_err());

        let expression = "* * ? * * *";
        assert!(schedule(expression).is_err());

        let expression = "* * * * ? *";
        assert!(schedule(expression).is_err());
    }

    #[test]
    fn test_nom_invalid_trailing_characters() {
        let expression = "* * * * * *foo *";
        assert!(schedule(expression).is_err());

        let expression = "* * * * * * * foo";
        assert!(schedule(expression).is_err());
    }

    /// Issue #86
    #[test]
    fn shorthand_must_match_whole_input() {
        let expression = "@dailyBla";
        assert!(schedule(expression).is_err());
        let expression = " @dailyBla ";
        assert!(schedule(expression).is_err());
    }
}
