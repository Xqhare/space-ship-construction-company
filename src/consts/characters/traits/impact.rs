use crate::{consts::EMPTY_BV_PAIR, modules::modifier::r#type::ModifierType, utils::op::Op};

/// Defines how a [`CharacterTrait`](super::CharacterTrait) modifies a character base value (or
/// base value group key such as `SCI_BONUS_ALL_FIELDS.0`).
///
/// # Expected Value Scale (`Percentage` vs. `Flat`)
/// Evaluated across all of a character's traits via [`accum_char_traits`](crate::needs_home::accum_char_traits):
/// - [`ModifierType::Percentage`] (default in [`TraitStatImpact::new`]): The percentage accumulator
///   starts at `1.0` (`100%`). With [`Op::Add`], a `base_value_value` of `0.1` adds `+10%`
///   (multiplier `1.1`), whereas `-0.05` subtracts `5%` (multiplier `0.95`).
/// - [`ModifierType::Flat`] ([`TraitStatImpact::new_flat`]): The flat accumulator starts at `0.0`
///   and is added to the stat prior to multiplying by the percentage accumulator.
#[derive(Debug, Clone, Copy)]
pub struct TraitStatImpact {
    pub base_value_name: &'static str,
    pub base_value_value: f32,
    pub kind: ModifierType,
    pub op: Op,
}

impl TraitStatImpact {
    pub(super) const fn const_default() -> TraitStatImpact {
        TraitStatImpact {
            base_value_name: EMPTY_BV_PAIR.0,
            base_value_value: 0.0,
            kind: ModifierType::Percentage,
            op: Op::Add,
        }
    }
    /// Creates a new percentage stat impact (`ModifierType::Percentage`, [`Op::Add`]).
    ///
    /// For evaluation details, see [`accum_char_traits`](crate::needs_home::accum_char_traits) and
    /// [`StatBox::get_full_stats`](crate::utils::boxes::data_box::stat_box::StatBox::get_full_stats).
    pub const fn new(base_value_name: &'static str, base_value_value: f32) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::const_default();
        tsi.base_value_name = base_value_name;
        tsi.base_value_value = base_value_value;
        tsi
    }
    /// Creates a new flat stat impact (`ModifierType::Flat`, [`Op::Add`]).
    pub const fn new_flat(base_value_name: &'static str, value: f32) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::new(base_value_name, value);
        tsi.kind = ModifierType::Flat;
        tsi
    }
    /// Creates a custom stat impact with an explicit [`ModifierType`] and [`Op`].
    pub const fn custom(
        base_value_name: &'static str,
        base_value_value: f32,
        kind: ModifierType,
        op: Op,
    ) -> TraitStatImpact {
        TraitStatImpact {
            base_value_name,
            base_value_value,
            kind,
            op,
        }
    }
}
