use std::collections::BTreeSet;

pub type Ordinal = u32;
// TODO: Make OrdinalSet an enum.
// It should either be a BTreeSet of ordinals or an `All` option to save space.
// `All` can iterate from inclusive_min to inclusive_max and answer membership
// queries
pub type OrdinalSet = BTreeSet<Ordinal>;