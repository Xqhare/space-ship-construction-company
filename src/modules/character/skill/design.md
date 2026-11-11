# Character Skill Design

Just like `CharacterTrait`, a skill is split into two parts:

1. **Static Definition (`ConstSkill`)**: Single source of truth defined in a compile-time registry, containing the skill's name, its optional `master` stat impact, and any `secondary` stat impacts.
2. **Dynamic State (`CharacterSkill`)**: Stored on `Character.skill`, tracking only what is needed for a specific character's progression (`skill: &'static ConstSkill`, `affinity`, `points`, `level`).

---

## 1. Data Structures & Formulas

```rust
pub struct ConstSkill {
    pub name: &'static str,
    pub master: Option<SkillStatImpact>,
    active_secondaries_count: usize,
    secondaries: [SkillStatImpact; MAX_SKILL_SECONDARIES],
}

pub struct SkillStatImpact {
    pub base_value_name: &'static str,
    pub factor: f32,
    pub op: Op,
}

pub struct CharacterSkill {
    pub skill: &'static ConstSkill,
    pub affinity: HpFloat,
    pub points: HpFloat,
    pub level: Level,
}

fn calc_final_stat_impact(
    ssi: &SkillStatImpact,
    aff: HpFloat,
    level: Level,
) -> (HpFloat, Op) {
    let base_level = match level {
        Level::Low(inner)    => 1 + inner,        // 1..=10
        Level::Medium(inner) => 12 + inner,       // 12..=21 (jump from 10 -> 12)
        Level::High(inner)   => 25 + inner,       // 25..=34 (jump from 21 -> 25)
        Level::Ultra(inner)  => 50 + (inner * 5), // 50, 55, 60 (massive jump)
    };
    let base_level = apply_rust_to_skill_level(base_level);
    let level_factor = HpFloat::from(base_level) * aff;
    let final_factor = HpFloat::from(ssi.factor) * level_factor;
    (final_factor, ssi.op)
}
```

### Master vs. Secondary Skills & Registry Lookup
- Most `consts::characters::base_values` (and science fields) have a **Master** skill.
- When a character performs an action for a stat they do not yet have, the relevant base value is added to their `StatBox` along with the associated Master skill at `Level::Low(0)` (and a calculated `affinity`).
- Some base values (`HEALTH`, `MENTAL_HEALTH`, `PAIN`, `MONEY`, `HAPPINESS`) do not have a Master skill.
- Other base values (like `SOCIABILITY`) are influenced as a secondary stat by many specific skills (à la Dwarf Fortress: *Intimidation*, *Flattery*, *Lying*, etc.).
- **Helpers to provide on the registry:**
  - `all_affecting_skills(base_value_name)` -> returns all `&'static ConstSkill` that impact `base_value_name` (either as `master` or in `secondaries`), which can be filtered at the callsite.
  - `master_skill(base_value_name)` -> returns the `Option<&'static ConstSkill>` that is the master skill for `base_value_name`.
- **`CharacterSkill` constructors:**
  - `CharacterSkill::new(skill_name)` (looks up `&'static ConstSkill` from the registry).
  - `CharacterSkill::from_skill(&'static ConstSkill)`.

---

## 2. Levels, Costs & Rarity

```rust
// Numbers are placeholders.
// NOTE: Because of the per-level cost formula, the distance between tiers should always be COST * 2 at a minimum.

/// Unfamiliar -> Capable Amateur
const POINT_COST_LOW_LEVEL: u16 = 128;
/// Weak Trainee -> Skilled Professional
const POINT_COST_MEDIUM_LEVEL: u16 = 256;
/// Specialist -> Industry-leading Master
const POINT_COST_HIGH_LEVEL: u16 = 1024;
/// Industry Legend -> Famous Industry Legend
const POINT_COST_ULTRA_LEVEL: u16 = 4096;

// Levels range from 0..=LEVEL_AMOUNT.
// The level number is converted into its corresponding display title for the player.
const LEVEL_AMOUNT_LOW: u8 = 9;
const LEVEL_AMOUNT_MEDIUM: u8 = 9;
const LEVEL_AMOUNT_HIGH: u8 = 9;
const LEVEL_AMOUNT_ULTRA: u8 = 2;

/// All inner `u8` values must be `<= LEVEL_AMOUNT_*` for their tier.
pub enum Level {
    Low(u8),
    Medium(u8),
    High(u8),
    Ultra(u8),
}
```

### Per-Level Point Cost Formula
```text
final_points_cost_of_level = (1 + (level / LEVEL_AMOUNT)) * POINT_COST
```
The first sub-level (`0`) of a tier costs `1.0 * POINT_COST`, scaling linearly up to `2.0 * POINT_COST` at the final sub-level of that tier.

### Expected Skill Distribution (Maximum Skill)
- **Low / Medium:** The vast majority of characters and their skills will not exceed `Medium`.
- **High:** Hard to reach (requires vocational training, scouting promising candidates early in school, etc.).
- **Ultra:** Extremely rare (*Industry Legend*). Players will mostly obtain `Ultra` characters via events, though nothing hard-caps a lucky player from training a character up to `Ultra` before they retire.

---

## 3. Open Design Decisions to Settle

### Open Decision A: Skill Affinity (Fixed vs. Dynamic)

How naturally a character picks up and applies a specific skill. Base affinity is generated at skill initialization taking into account character traits and base stats (like `IQ`).

| Option | Behavior | Pros | Cons |
| :--- | :--- | :--- | :--- |
| **Option 1: Fixed Affinity** | Generated once when the skill is initialized on the character and remains constant. | - Zero per-tick overhead.<br>- Clear, predictable talent ceiling per character.<br>- Avoids double-dipping with skill `points`/`level` (which already grow with use). | - Doesn't model "getting into a groove" or "getting rusty" via affinity. |
| **Option 2: Dynamic Affinity** *(like Research Fields)* | Starts at the generated base value, grows whenever the skill is used, and decays continuously over time (faster at higher levels, or modified by traits). | - Mirrors the `ResearchField` affinity mechanic.<br>- Rewards keeping characters actively practicing their specialty. | - Overlaps with **Open Decision B (Skill Atrophy)** below.<br>- Extra per-tick bookkeeping for every skill on every character. |

### Open Decision B: Skill Atrophy (Losing Skill Over Time)

Should unused skills decay over time?

| Option | Behavior | Pros | Cons |
| :--- | :--- | :--- | :--- |
| **Option 1: Full Decay** | Unused skills lose points and can drop down levels (`High` -> `Medium`). | - Realistic simulation depth.<br>- Creates tough player trade-offs (e.g., whether a great researcher + manager should focus on one role at the expense of the other). | - Can feel punishing/frustrating to micro-manage.<br>- Requires tracking last-used timestamps or decay ticks for all skills. |
| **Option 2: No Decay** | Skills never lose points or levels once earned. | - Simplest for both player and simulation.<br>- Zero idle-tick cost. | - No "rustiness" trade-off when reassigning specialists for years. |
| **Option 3: Middle Ground (Sub-level Decay Only)** | **Skill `Level` is permanent**, but `points` toward the *next* level slowly decay toward `0` when unused. | - Never deletes a hard-earned `Level` milestone.<br>- Still discourages leaving a half-trained character idle mid-level.<br>- Simple to clamp at `points = 0`. | - Once a character reaches the cap they care about, decay no longer matters. |

### Coming Up With a Solution

1. Zero Action Lockout (EU5 / anti-Software Inc & Zomboid): Any character can be assigned to any job or action; low skill just means low output or costly mistakes rather than greyed-out UI buttons.
2. Affinity Works on Both Input and Output:
    - Input (Learning): gained_points = raw_points * affinity
    - Output (Performance): final_impact = ssi.factor * (base_level * affinity)
    - And because affinity is derived from the character's traits + base stats (and only recalculated when a trait changes), it has zero per-tick overhead.
3. Affinity-Governed Rust & Skill Floor (DF + RimWorld synthesis):
    - High-affinity characters naturally keep up with their field (reading literature, tinkering in their spare time), which both slows down rust, caps maximum rust (high enough affinity = unrustable), and sets a level_floor below which they can never fall.

#### Getting Rusty

For simplicity I will describe only the rusting system, it may just live directly on a CharacterSkill.

```rust
// No public fields to guard against edits
struct SkillRust {
    /// Scale will be from 0.0..=1.0 (softcapped)
    affinity: HpFloat
    /// Increments on each idle check (e.g. weekly/monthly); resets to 0 on use.
    unused_counter: u8,
    lost_rust_this_sim_tick: bool,
    /// Current rust stacks (0..=max_rust).
    rust: u8,
    /// Calculated from affinity only. Always updated when affinity changes
    max_rust: u8,
}

const MAX_LEVEL_AMOUNT: u8 = 3*10+3;

impl SkillRust {
    pub fn update_trait(&mut self) {
        self.affinity = calculate_affinity(traits);
        self.max_rust = calc_max_rust_for_affinity(self.affinity);
        if self.rust > self.max_rust {
            self.rust = self.max_rust
        }
    }
    pub fn simulation_tick(&mut self) {
        self.unused_counter += 1;
        if self.unused_counter >= 6 {
            self.unused_counter = 0;
            self.rust += 1;
            if self.rust > self.max_rust {
                self.rust = self.max_rust
            }
        }
        self.lost_rust_this_sim_tick = false;
    }
    pub fn apply_rust_to_skill_level(skill_level: usize) -> usize {
        skill_level.saturating_sub(self.rust).max(1)
    }
    pub fn skill_used(&mut self) {
        self.unused_counter = 0;
        if !lost_rust_this_sim_tick {
            self.rust = self.rust.saturating_sub(1);
            self.lost_rust_this_sim_tick = true;
        }
    }
    pub fn reset_rust(&mut self) {
        self.unused_counter = 0;
        self.rust = 0;
        self.lost_rust_this_sim_tick = false;
    }
    pub fn max_rust(&self) -> u8 {
        self.max_rust
    }
    pub fn rust(&self) -> u8 {
        self.rust
    }
}

fn scale_affinity_for_rust(affinity: HpFloat) -> HpFloat {
    let out;
    if affinity <= 0.0 {
        out = 0;
    } else if affinity >= 1.0 {
        out = max_level_amount.into();
    } else {
        // In range aff: > 0.0 && < 1.0
        out = round_up(max_level_amount * affinity);
    }
    out
}

fn calc_max_rust_for_affinity(affinity: Hpfloat) -> u8 {
    let max_level_amount = 3*10+3; // Low(0) through Ultra(2)
    round_down(max_level_amount.saturating_sub(scale_affinity_for_rust(affinity)))
}

```

### How Other Games Handle Skills

All in all there are not that many different approaches to handling skill.

Most often, it is just a number that only ever goes up, never down.

Even of a decrease of skill is present, its either very leniant (DF) or built in (RimWorld).

Also, if there is a kind of affinity present, (like RimWorlds flames) it affects only the rate at which skill points are gained.

In general the way of:

Doing_Action -> Gain Points -> Affinity affects Point Gain -> Add points to level

And:

Doing_Action -> Outcome only cares about the level number

Makes it all a one-way street of sorts.

I would make this more like:

Doing_Action -> Gain Points -> Affinity affects Point Gain -> Add points to level

Doing_Action -> Points_Calc (with level number) -> Affinity affects Points Gain -> Final Number

This way affinity affects points both on their way into and out of the char.
If a char has high affinity for Managing, it should stand to reason that the char performs better than a char of the same level with less affinity. Both for learning from it and in how to get the best out of it.

Just as I intuitively disliked taking away levels from a character to not punish the player, most games I have looked at shy away from it completely.

RimWorld tries to find a middle ground (just as I with my Idea of letting the current, in level, progress tick down), but does go further by allowing a pawn to forget a skill entirely (over a long enught timeframe).

Now, I like the approach DF takes, by modeling the natural skill atrophy through non-use.

I think I can use my affinity for that.

If I generate the affinity for a skill from the traits present at its creation (and updating it whenever a trait changes), I could use it to define a level_floor for each skill on a per character basis. If a char is a great engineer, the floor the capabilites of that char falls to will be higher if the affinity is higher (the char reads industry literature / builds in spare time / etc). The affinity could also impact the rate of decrease.

I also like the 'rust' system of DF, but I think it can be expanded:

Firstly, the max_level of rust a skill of a specific char can be is dependend on the affinity (A high enough level could make the skill unrustable at all).

Of course I then need an entire sub system to track rust. DF just uses incr counters on timers, and each use resets those. Could be a good fit here.

#### Europa Universalis 5

In contrast to the previous 4 titles, Eu5 is a character driven game at its core.

The population sim can be ignored.

However, the Cabinet is the closest to a skill system.

Any Cabinet Slot can be assigned a specific action with a base efficiency.
The Slot can then be filled with a character. Each action benefits from one of three skills, choosing a char strong in the skill to boost the action one wants is important.

There is no lockout! Even if a char has a dip skill of 1 (out of 100) he can still be assigned to any dip action. It will just be worse than if he had 10 or 50 skill.

#### Aurora 4x

Way too nebulous and deep in the simulation as to matter to a noob like me (never even met aliens in that game).

But as I understand:

Skills just give percentage boni on certain actions if that character is in command.

But all of that only applies to civil governance (sectors & planetary govenors) as well as the chain of command (down to ship components).

#### Timberborn

Does away completely with Skills. Each beaver can do any job. The only difference between how well two beavers do a job is their happiness level (higher = better).

#### Oxygen Not Included

Wiki:

> Duplicants earn experience over time, which in turn earns Skill Points. Each Skill Point earned can be spent in the Skills menu to learn a Skill. Each Skill bestows benefits on the Duplicant such as increased Attributes or the ability to perform new kinds of tasks. Once a Skill is learned, the Duplicant gains access to the Skill's hat as a cosmetic option. The Skill will remain learned unless the Duplicant uses a Skill Scrubber, which refunds all spent Skill Points and relinquishes all hats. 

#### Software Inc

Skills are split.

- Leadership
- Development
- Service

Each SkillKind has its own sub set of skills.
A character levels up the 'SkillKind', and the player can then allocate a point into any of the sub skills of that SkillKind.

Major pain point of mine: Hiring a manager with less than 3 (max) levels in automation & HR means that the team the char is leading CANNOT be fully automated (what they work on OR hiring & training). No manager was ever stopped by his stupididy from hiring or training. He would just do the wrong thing (training a designer for a programming job by accident; Giving the wrong employee the training another would need etc)

#### XCOM

Really quite basic.

When promoted they get a few points (weighted for their profession, Snipers tend to get more aim; Heavies more hp).
Skill loss is only ever temporary.

#### Project Zomboid

Gaining skill exp is largely the same as in the other games mentioned.

In contrast to many others, it is impossible to lose exp in a skill. (Except by dying of course; But thats more of a hard char reset anyways)

Zomboid, along with RPGs and other single person survival games like Valheim, tends to gate actions / interactions behind specific levels.
This specific design choice is one I am trying to reverse: You can tell any character to do any action. How good the result turns out to be however is heavily dependent on the specific skill.

#### Crusader Kings I-III

Instead of skills, ck generaly uses Attributes. The core way has not changed much:

> Each character has base attributes which are impacted by their traits. The total is displayed on the character screen, while the base is visible on mouseover. Base attributes are mostly acquired during childhood through education, but can also be altered by events.

#### Dwarf Fortress

Wiki:

> Skills are used […] to accomplish almost every task in the game. Higher levels of a skill allow a dwarf to accomplish the respective task more quickly and/or more effectively. Whenever a skill is used, experience is gained for that skill, allowing the dwarf to progress to higher skill levels. […] 
> If a dwarf does not use a skill for a prolonged period of time, the skill will be labeled "rusty." If the rusty skill continues to remain unused, it will eventually be labeled "very rusty," or "V rusty" in-game. Skills remaining at 'very rusty' for (even more) prolonged periods of time will gradually suffer permanent experience loss.
> Skills are never referred to in-game by "level number", but for all practical purposes, that is how they are treated by the game. […]
> Labors with or without quality often have a time period associated with them, and skill levels reduce this significantly. Legendary skill can eliminate all time required to do a job down to a single action, exponentially increasing productivity.
>
> ## Skill Rust
> 
> Once per day, each skill's "unused" counter increments by 1, and if it reaches the cap (without the skill ever being used), it resets to zero and increments the rust counter. Once the rust counter reaches its cap, it resets to zero, adds a "layer" of rust to the skill (to a maximum of 6), and increments the demotion counter. When the demotion counter reaches its cap, it resets to zero and the skill level is permanently reduced by 1 (and must be re-earned). With the default numbers, it takes about 4 and a half months before rust sets in, 2 years and 3 months before rust maxes out, and 6 years before actual skill levels begin to decay. 


#### RimWorld

Wiki:

> Performing relevant work awards Experience Points (XP) towards improving a pawn's skill level. Pawns also slowly lose XP over time, so constant practice at a skill is necessary to keep one's skill levels high. […]
> High skill levels grant improved speed or success in relevant jobs, up to Level 20. Conversely, a low skill level will result in slow or poor performance in related work. […]
> Skill leveling in RimWorld is very similar to many role playing games: experience points are progressively earned, and then the skill is 'leveled up' when a certain XP threshold is reached. Importantly, the effect of a skill improves as level increases, but the experience point threshold for leveling up also increases. This means that leveling up a skill takes more XP and thus a longer time at high levels.
> Experience points are generally gained at a consistent rate per unit time […] This is not true for all tasks however, as some will simply give a lump sum of experience for each task completed, such as doctoring a wound.
> Regardless of how XP is gained, it is multiplied by Passion and Global Learning Factor […] Additionally, there is a soft cap of 4000 net XP gained per day per skill. Any XP gains past this point are multiplied by 20%.
