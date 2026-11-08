use crate::{consts::EMPTY_BV_PAIR, modules::modifier::r#type::ModifierType, utils::op::Op};
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
    /// Creates a new stat impact.
    ///
    /// Overwrites the default value held by `base_value` with `value`.
    /// The kind will be `ModifierType::Percentage`.
    /// The op will be `ModifierOp::Add`.
    ///
    /// For more info see [`StatBox::get_full_stats()`]
    pub const fn new(base_value_name: &'static str, base_value_value: f32) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::const_default();
        tsi.base_value_name = base_value_name;
        tsi.base_value_value = base_value_value;
        tsi
    }
    /// Create a new stat impact of kind `ModifierType::Flat`.
    ///
    /// Overwrites the default value held by `base_value` with `value`.
    /// The op will be `ModifierOp::Add`.
    pub const fn new_flat(base_value_name: &'static str, value: f32) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::new(base_value_name, value);
        tsi.kind = ModifierType::Flat;
        tsi
    }
    /// Create a fully custom stat impact.
    ///
    /// Overwrites the default value held by `base_value` with `value`.
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
