
I plan on having characters drive as much of the game as possible.


```rust

struct Character {
    name: Name,
    stats: DataBoxId,
    birthday: Utc,
    teams: Vec<TeamBoxSlotId>,
    memories: Vec<Memories>,
    traits: Vec<Trait>,
    skills: Vec<Skill>
}

struct Skill {
    name: String,
    master: Option(SkillStatImpact),
    secondaries: Vec<SkillStatImpact),
    affinity: HpFloat,
    points: HpFloat
    level: Level
}

pub struct SkillStatImpact {
    base_value: String,
    factor: HpFloat,
    op: ModifierOp,
}

fn calc_final_stat_impact(ssi: SkillStatImpact, aff: HpFloat, level: Level) -> (HpFloat, ModifierOp) {
    let base_level = match level {
        Level::Low(inner)    => 1 + inner,        // 1..=10
        Level::Medium(inner) => 12 + inner,       // 12..=21 (jump from 10 -> 12)
        Level::High(inner)   => 25 + inner,       // 25..=34 (jump from 21 -> 25)
        Level::Ultra(inner)  => 50 + (inner * 5), // 50, 55, 60 (massive jump)
    };
    let level_factor = HpFloat::from(base_level) * aff;
    let final_factor = ssi.factor * level_factor;
    (final_factor, ssi.op)
}
// Numbers are made up
// NOTE: Because of the per level cost formula, the distance between levels should always be COST*2 at a minimum
/// Unfamiliar -> Capable Amateur
const POINT_COST_LOW_LEVEL: u16 = 128;
/// Weak Trainee -> Skilled Professional
const POINT_COST_MEDIUM_LEVEL: u16 = 256;
/// Specialist -> Industry-leading Master
const POINT_COST_HIGH_LEVEL: u16 = 1024;
/// Industry Legend -> Famous Industry Legend
const POINT_COST_ULTRA_LEVEL: u16 = 4096;

// Levels range form 0..=LEVEL_AMOUNT
// The level number would always be converted into the corresponding name before displaying it for the player
const LEVEL_AMOUNT_LOW: u8 = 9;
const LEVEL_AMOUNT_MEDIUM: u8 = 9;
const LEVEL_AMOUNT_HIGH: u8 = 9;
const LEVEL_AMOUNT_ULTRA: u8 = 2;

/// All u8 must be <= LEVEL_AMOUNT
enum Level {
    Low(u8),
    Medium(u8),
    High(u8),
    Ultra(u8),
}

stuct Trait {
    name: String,
    affected_base_value_pairs: Vec<(String, HpFloat)>
}

stuct Name {
    legal_name: String,
    first: String,
    middle: Option<String>,
    last: String
    nickname: String
}

```

## Needs

To factor out some base_value_pairs I copied over from the sims2 wiki:

```rust
pub const COMFORT: ConstBVPair = ("comfort", None);
pub const FULFILMENT: ConstBVPair = ("fulfilment", None);
pub const ENVIRONMENT: ConstBVPair = ("environment", None);
```

These don't *really* make sense as base_values; However I think it does make sense to keep them for a sub system that interacts with the 'Happiness' base_value.

It would also fit nicely with the goals, if the environment or fulfilment of a char in its current team / job is low: why not change from 'get a promotion' to be stuck leading the team one hates (if getting the job of ones boss) to change laterally in the company?

## Memories / Goals

For a much later stage probably.

I would love a memory and goal system.

- Life goals
- Current goal (trying to further the life goal)
- good memories & bad memories
- Friends / Characters the character likes
- Enemies / Characters the character dislikes

My current thinking:

In a sim step, choose random employees.

Then:

Wants to interact with others -> Choose interaction that fits char -> Choose char to interact with (inside the same team etc) -> Interact -> If 'impactful' (dont know how that will work) add as memory (good or bad).

Goals would steer the choosing of interactions / actions in general.

If the Life goal is 'become CEO', each current goal will be a more simple 'get promoted' goal. That would steet the char to improve not only the currently needed skills, but also the skills of the wanted position (be it for the work itself or social / management skills).

## Affinity

### Option 1

Represent how natural a character picks up a specific skill.

It is a fixed value, generated at character init. It takes into account traits and some character stats like IQ.

### Option 2

Just as with research fields.

The base affinity value is generated as in option 1.

It grows whenever the skill is utilised in any way.

It decreases over time continuously.
Maybe the higher the level the faster it decreases? Or statically, influenced by traits?

## CharacterSlotMap

Characters will be stored in a centralised SlotMap.

## Getting Characters

There will be a pool of 1000 characters that are available for use. (Think possible employees).

This list will be filled with a generalised character creation call that will return a Character to be inserted in the SlotMap.

Characters will be removed from the pool when they are hired or until some duration of time has passed.

The higher the reputation / how good of an employer the company is, more skilled characters are added and characters will stay in the pool longer.

## Teams

Each character can be part of one or more teams.
Teams are stored centrally in a SlotMap, each character will hold the ID of the teams they are part of.

Each team will also have a list of CharacterSlotMapId's of the characters in it.

A character can be part of several teams, according to its kind.

Some team kinds (e.g. Research team) are limited to one per character.

A character can be part of several management teams (e.g. Research Team lead & part of the R&D division management level).

### Future Refactor

At some point I think the team field could also be used as a local cache.

I could store a simplified list of what team kinds the char is already a part of to simplify the assignments.


## Maximum Skill

I expect the vast majority of characters and their skills to not exceed the MEDIUM level.
HIGH skill should be hardish (vocational training, searching in schools for promising candidates to support early, etc).
ULTRA skill should be rare / extremely rare (industry legend, etc)
I would expect the player to only get characters with ULTRA skill via events.
This wont be enforced, for more dynamics - if the player is lucky, maybe they can train a character to be on that level. Until they retire of course.

## Per Level Cost

In general the formula would be:

```
final_points_cost_of_level = (1 + (level / LEVEL_AMOUNT)) * POINT_COST
```

Meaning: The very first level has the cost of COST, each additional level has a slightly higher cost with the last level costing 2 * COST.

## Loosing Skill

While skill atrophy is somewhat real and learned knowledge does get outdated, I remain conflicted on how to handle it in game.

It boils down to a simple choice really:

1. Skills can be lost
2. Skills cannot be lost

Pros for 1.

- Realistic
- Simulation depth
- Player choice / agency (keep a character part of some teams just so they still use their skills / Having to decide if a good researcher & manager should focus on one at the expense of the other) 

Pros for 2.

- Easier for the player
- Simpler for the game

### A middle ground?

Let the skill level be permanent, but the points in the current level move towards 0.

## IQ

Each character has an IQ (Intelligence Quotient).
It is generated upon character init.

It follows (As stolen from: https://en.wikipedia.org/wiki/IQ_classification on 2026-11-01; Terman & Merrill 1960, p. 18):

1. 0.03% - 160-169
2. 0.2% - 150-159
3. 1.1% - 140-149 - Genius
4. 3.1% - 130-139 - Astute
5. 8.2% - 120-129 - Shrewd
6. 18.1% - 110-119 - Smart
7. 23.5% - 100-109 - Bright
8. 23.0% - 90-99 - Dim
9. 14.5% - 80-89 - Dull
10. 5.6% - 70-79 - Dense
11. 2.0% - 60-69 - Dimwit
12. 0.4% - 50-59 - Simpleton
13. 0.2% - 40-49
14. 0.03% - 30-39

To simplify further:

```
// Instead of a random range, a random bool would be enough
// I could also extend my RngAPI to support weighted randoms.

// 50%
if 0 == random_range(0, 1) {
    if 0 == random_range(0, 1) {
        // 25%
        BRIGHT (100-109)
    } else {
        // 25%
        DIM (90-99)
    }
// 50%
} else {
    // 25%
    if 0 == random_range(0, 1) {
        if 0 == random_range(0, 1) {
            // 12.5%
            SMART (110-119)
        } else {
            // 12.5%
            DULL (80-89)
        }
    // 25%
    } else {
        // 12.5%
        if 0 == random_range(0, 1) {
            // 6.25%
            if 0 == random_range(0, 1) {
                SHREWD (120-129)
            } else {
            // 6.25%
                DENSE (70-79)
            }
        // 12.5%
        } else {
            // 6.25%
            if 0 == random_range(0, 1) {
                // 3.125%
                if 0 == random_range(0, 1) {
                    ASTUTE (130-139)
                } else {
                    DIMWIT (60-69)
                }
                // 3.125%
            // 6.25%
            } else {
                // Option 1; With a lot larger percantage of the pop having these traits
                if 0 == random_range(0, 1) {
                    // 3.125%
                    GENIUS (140-149)
                } else {
                    // 3.125%
                    SIMPLETON (40-49)
                }
                // Option 2; More realistic
                // 3.125%
                if 0 == random_range(0, 1) {
                    if 0 == random_range(0, 1) {
                        // 1.5625%
                        BRIGHT (100-109)
                    } else {
                        // 1.5625%
                        DIMM (90-99)
                    }
                // 3.125%
                } else {
                    if 0 == random_range(0, 1) {
                        // 1.5625%
                        GENIUS (140-149)
                    } else {
                        // 1.5625%
                        SIMPLETON (40-49)
                    }
                }
            }
        }
    }
}

```

Final numbers (Option 2 looks to be the best fit):

1. Genius, 1.5625% (Opt2) | 3.125% (Opt1) [1.1]
2. Astute, 3.125% [3.1]
3. Shrewd, 6.25% [8.2]
4. Smart, 12.5% [18.1]
5. Bright, 25% (Opt1) | 25% + 1.5625% (Opt2) [23.5]
6. Dim, 25% (Opt1) | 25% + 1.5625% (Opt2) [23.0]
7. Dull, 12.5% [14.5]
8. Dense, 6.25% [5.6]
9. Dimwit, 3.125% [2.0]
10. Simpleton, 1.5625% (Opt2) | 3.125% (Opt1) [0.4]

In essence this still is Terman & Merril, but normalised and capped.
The median of 90-110 is changed the most from their data, but still within 5%.

## Looks

Like IQ, looks are generated at char init.
There are 8 levels:

- Aberrant; 3.125%
- Hideous; 6.25%
- Unsightly; 12.5%
- Appealing; 25%
- Pretty; 25%
- Attractive; 12.5%
- Beautiful; 6.25%
- Gorgeous; 3.125%

They will provide a modifier for all social interactions of that character.

## Memories

At some point, maybe, I would like a simple memory system.

Timed modifiers kind of are like memories, but I want something more complex.

## Health, Death and Retirement

Each character has a health, mental health and pain stat.

#### Pain

Ranging from 0 (No pain) to 1000 (Maximum pain).

Should the pain raise above a threshold (probably player defined and nations state capped) the character will not work at all. Up until that point at a reduced rate.

Most modifiers of pain will be timed, but arthritis and some other things might be permanent.

Company policies would be the primary way of dealing with characters with permanent pain.(e.g Ergonomic chairs, physiotherapy provided, mechanical help etc)

### Health

Straight forward. Is influenced by some traits. If the health is below or reaches 0, the character dies immediately.

#### Injuries

Characters can get injuries, some may be permanent.

These can happen while working or not.

#### Mental Health

Functions largely like pain.

The values work as with health, 1000 is good mental health, 0 is no mental health.

If the value reaches 0 the character will commit suicide (except for some traits).

### Retirement

Characters retire when they hit the legally required (or company defined) age (whatever younger).

I like the idea of not removing them from the simulation entirely, but to mark them.
Maybe a player wants to rehire some badly needed talent at an enormous cost.
-> National forced retirement age

### Death

Complex, but I have dealt with enough loss to make this as impacting as possible.

#### Natural

The normal way to die. Mostly by retirees.

If any character dies by natural causes, nothing of real note will happen.

If not retired:
The team they are part of will loose some stats, but nothing that can be seen as a major problem.

#### Unnatural

Now, for most 'health == 0 && !retired' cases an event will happen.

What exactly happens is defined by the reason the health dropped to 0.

Should it be an accident (injury sustained while working / simulating their days off), its straight forward, see `Accidents`.

Should the health drop because of the pain stat, mental healt stat, or whatever else I can think of, it will be a suicide.

##### Suicide

Again, the reason for the drop largely decides what happens, some traits will play a role however.

If the reason has to do with work (or they have some traits), they will choose to end their life on site.

If it is for 'private' reasons, (or they are shy or whatever) it will be offsite.

###### On Site

Can be made the most horrific for the player.

Enormously massive event, company wide maluses to stats, worse for team members.

PR disaster (think Foxcon employees jumping from factory roofs).
Police investigation.

The way the player can deal with that will range from empathetic 'Shut down the production entirely until we know what happened and fix it' to foxcon's 'just put up netting to catch jumpers'

###### Example

Start:
`Sir, a worker of yard 2 has airlocked himself. It seems it wasn't an accident but intentonal.`

Possible player reactions:

1. 'Shut down the production entirely until we know what happened and fix it'
2. 'Investigate the situation'
3. 'Just put security near the airlocks'
4. 'PR disaster waiting to happen. Surpress the news'

###### Off Site

More similar to Natural causes, for the player.
The team members will get larger maluses, but nothing major for the rest of the company.

Off site suicide will be tracked as a number per year. Only if that number reaches a certain point will it be a major event.

Up until that point, just like in real life, no one really cares that much.

##### Accident

Accident deaths will be a major event.
The team members will get larger maluses, but nothing major for the rest of the company.

An official work safety investigation will be started.
PR disaster, but not as much.

The accident will be tracked as a number per year. Only if that number reaches a certain point will it be a massively major event.
