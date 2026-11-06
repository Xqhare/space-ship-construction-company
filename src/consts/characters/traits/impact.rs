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
    /// The kind will be `ModifierType::Percentage`.
    /// The op will be `ModifierOp::Add`.
    ///
    /// For more info see [`StatBox::get_all_stats()`]
    pub const fn new(base_value_pair: ConstBVPair) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::const_default();
        tsi.base_value_pair = base_value_pair;
        tsi
    }
    /// Create a new stat impact of kind `ModifierType::Flat`.
    ///
    /// The op will be `ModififerOp::Add`.
    pub const fn new_flat(base_value_pair: ConstBVPair) -> TraitStatImpact {
        let mut tsi = TraitStatImpact::new(base_value_pair);
        tsi.kind = ModifierType::Flat;
        tsi
    }
    /// Create a fully custom stat impact
    pub const fn custom(
        base_value_pair: ConstBVPair,
        kind: ModifierType,
        op: ModifierOp,
    ) -> TraitStatImpact {
        TraitStatImpact {
            base_value_pair,
            kind,
            op,
        }
    }
}
