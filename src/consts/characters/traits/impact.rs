use crate::{
    consts::{ConstBVPair, EMPTY_BV_PAIR},
    modules::modifier::{op::ModifierOp, r#type::ModifierType},
};
#[derive(Debug, Clone, Copy)]
pub struct TraitStatImpact {
    pub base_value_pair: ConstBVPair,
    pub kind: ModifierType,
    pub op: ModifierOp,
}

impl TraitStatImpact {
    pub(super) const fn const_default() -> TraitStatImpact {
        TraitStatImpact {
            base_value_pair: EMPTY_BV_PAIR,
            kind: ModifierType::Percentage,
            op: ModifierOp::Add,
        }
    }
    /// Creates a new stat impact.
    ///
    /// Overwrites the default value held by `base_value` with `value`.
    /// The kind will be `ModifierType::Percentage`.
    /// The op will be `ModifierOp::Add`.
    ///
    /// For more info see [`StatBox::get_full_stats()`]
    pub const fn new(base_value: ConstBVPair, value: f32) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::const_default();
        tsi.base_value_pair = (base_value.0, Some(value));
        tsi
    }
    /// Create a new stat impact of kind `ModifierType::Flat`.
    ///
    /// Overwrites the default value held by `base_value` with `value`.
    /// The op will be `ModifierOp::Add`.
    pub const fn new_flat(base_value: ConstBVPair, value: f32) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::new(base_value, value);
        tsi.kind = ModifierType::Flat;
        tsi
    }
    /// Create a fully custom stat impact.
    ///
    /// Overwrites the default value held by `base_value` with `value`.
    pub const fn custom(
        base_value: ConstBVPair,
        value: f32,
        kind: ModifierType,
        op: ModifierOp,
    ) -> TraitStatImpact {
        TraitStatImpact {
            base_value_pair: (base_value.0, Some(value)),
            kind,
            op,
        }
    }
}
