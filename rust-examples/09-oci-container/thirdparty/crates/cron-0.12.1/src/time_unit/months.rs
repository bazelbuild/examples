use crate::error::*;
use crate::ordinal::{Ordinal, OrdinalSet};
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;
use once_cell::sync::Lazy;

static ALL: Lazy<OrdinalSet> = Lazy::new(|| { Months::supported_ordinals() });

#[derive(Clone, Debug, Eq)]
pub struct Months{
    ordinals: Option<OrdinalSet>
}

impl TimeUnitField for Months {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>) -> Self {
        Months{
            ordinals: ordinal_set
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Months")
    }
    fn inclusive_min() -> Ordinal {
        1
    }
    fn inclusive_max() -> Ordinal {
        12
    }
    fn ordinal_from_name(name: &str) -> Result<Ordinal, Error> {
        //TODO: Use phf crate
        let ordinal = match name.to_lowercase().as_ref() {
            "jan" | "january" => 1,
            "feb" | "february" => 2,
            "mar" | "march" => 3,
            "apr" | "april" => 4,
            "may" => 5,
            "jun" | "june" => 6,
            "jul" | "july" => 7,
            "aug" | "august" => 8,
            "sep" | "september" => 9,
            "oct" | "october" => 10,
            "nov" | "november" => 11,
            "dec" | "december" => 12,
            _ => {
                return Err(
                    ErrorKind::Expression(format!("'{}' is not a valid month name.", name)).into(),
                )
            }
        };
        Ok(ordinal)
    }
    fn ordinals(&self) -> &OrdinalSet {
        match &self.ordinals {
            Some(ordinal_set) => ordinal_set,
            None => &ALL
        }
    }
}

impl PartialEq for Months {
    fn eq(&self, other: &Months) -> bool {
        self.ordinals() == other.ordinals()
    }
}