use crate::consts::characters::{
    flags::{
        ONLY_PROVIDES_NEEDED,
        suicide::{SUICIDE_NEVER, SUICIDE_NEVER_OFF_SITE, SUICIDE_NEVER_ON_SITE},
    },
    traits::CharacterTrait,
};

define_trait_registry! {
    pub static TRAIT_SOCIAL_REGISTRY = [
        TRAIT_SOCIAL_BOOMING_VOICE => CharacterTrait::social("Booming Voice"),
        TRAIT_SOCIAL_MACHIAVELLIAN => CharacterTrait::social("Machiavellian")
            .with_flags([SUICIDE_NEVER_OFF_SITE, ONLY_PROVIDES_NEEDED]),
        TRAIT_SOCIAL_SILVER_TONGUE => CharacterTrait::social("Silver Tongue"),
        TRAIT_SOCIAL_CONCILIATORY  => CharacterTrait::social("Conciliatory"),
        TRAIT_SOCIAL_TIMID         => CharacterTrait::social("Timid"),
        TRAIT_SOCIAL_GROOMED       => CharacterTrait::social("Groomed"),
        TRAIT_SOCIAL_FUSSY         => CharacterTrait::social("Fussy"),
        TRAIT_SOCIAL_AFFECTIONATE  => CharacterTrait::social("Affectionate"),
        TRAIT_SOCIAL_WRATHFUL      => CharacterTrait::social("Wrathful"),
        TRAIT_SOCIAL_VENGEFUL      => CharacterTrait::social("Vengeful"),
        TRAIT_SOCIAL_TRUSTING      => CharacterTrait::social("Trusting"),
        TRAIT_SOCIAL_SHY           => CharacterTrait::social("Shy")
            .with_flags([SUICIDE_NEVER_ON_SITE]),
        TRAIT_SOCIAL_SADISTIC      => CharacterTrait::social("Sadistic"),
        TRAIT_SOCIAL_HONEST        => CharacterTrait::social("Honest"),
        TRAIT_SOCIAL_GREGARIOUS    => CharacterTrait::social("Gregarious"),
        TRAIT_SOCIAL_GENEROUS      => CharacterTrait::social("Generous"),
        TRAIT_SOCIAL_FORGIVING     => CharacterTrait::social("Forgiving"),
        TRAIT_SOCIAL_DECEITFUL     => CharacterTrait::social("Deceitful"),
        TRAIT_SOCIAL_COMPASSIONATE => CharacterTrait::social("Compassionate")
            .with_flags([SUICIDE_NEVER]),
        TRAIT_SOCIAL_CHARMING      => CharacterTrait::social("Charming"),
        TRAIT_SOCIAL_BOSSY         => CharacterTrait::social("Bossy"),
        TRAIT_SOCIAL_CALLOUS       => CharacterTrait::social("Callous"),
        TRAIT_SOCIAL_GREEDY        => CharacterTrait::social("Greedy"),
        TRAIT_SOCIAL_CHARITABLE    => CharacterTrait::social("Charitable"),
        TRAIT_SOCIAL_WROTH         => CharacterTrait::social("Wroth"),
        TRAIT_SOCIAL_KIND          => CharacterTrait::social("Kind"),
        TRAIT_SOCIAL_ARROGANT      => CharacterTrait::social("Arrogant"),
        TRAIT_SOCIAL_HUMBLE        => CharacterTrait::social("Humble"),
    ];
}
