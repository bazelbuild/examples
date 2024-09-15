use crate::ordinal::{Ordinal, OrdinalSet};
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;
use once_cell::sync::Lazy;

static ALL: Lazy<OrdinalSet> = Lazy::new(|| { Years::supported_ordinals() });

#[derive(Clone, Debug, Eq)]
pub struct Years{
    ordinals: Option<OrdinalSet>
}

impl TimeUnitField for Years {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>) -> Self {
        Years{
            ordinals: ordinal_set
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Years")
    }

    // TODO: Using the default impl, this will make a set w/100+ items each time "*" is used.
    // This is obviously suboptimal.
    fn inclusive_min() -> Ordinal {
        1970
    }
    fn inclusive_max() -> Ordinal {
        2100
    }
    fn ordinals(&self) -> &OrdinalSet {
        match &self.ordinals {
            Some(ordinal_set) => ordinal_set,
            None => &ALL
        }
    }
}

impl PartialEq for Years {
    fn eq(&self, other: &Years) -> bool {
        self.ordinals() == other.ordinals()
    }
}