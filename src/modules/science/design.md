
```rust
struct ResearchFields {
    HashMap<ResearchFieldKind, Field>
}

struct Field {
    affinity: HpFloat,
    points: HpFloat,
    /// A nation can restrict the tech level of a field.
    /// This is done by setting `Some` value that is bigger than 0.
    /// Tech levels that exceed `current_year + restricted` will not give points to higher academia levels
    /// This means the number works counterintuitively: Small == big restriction; Big == small restriction
    restricted: Option<u8>
}

struct Academia {
    world: ResearchFields,
    player: ResearchFields,
    nations: HashMap<StateName, ResearchFields>
    ai: HashMap<AiCompName, ResearchFields>
    // Maybe with the correct State for faster lookups:
    ai: HashMap<AiCompName, (StateName, ResearchFields)>
}
```

## Restriction

### Nation level

Fields restricted here mean that any component / ship / whatever that uses that tech level or greater can only be sold to that nation state or on its market.

### Company level

Fields restricted here mean that any component / ship / whatever that uses that tech level or greater can only be sold to that companies parent nation state.

Selling on the national market would open up the tech to that market.

## Affinity

Whenever points are added to the `points` field of a `Field`, the `affinity` field is also increased.

Decreasing the affinity will happen every year.

As a result, the more research is done on a field, the higher the affinity. As soon as the work stops, it slowly decreases.

When reaching 0 the decrease is slowed even more, but negative is possible!

### Simulation Tick Choice

To make it seem more fluid, it would be nice to move the decrease of affinity to lower (monthly, quarterly maybe), however that does impact performace; Propably unneccesarely so.

## Points

Points, their scale as yet undefined, are the basic bonus to the tech level of a field.
They are used whenever an integer is needed for simulation calculations.

Points may never be negative.

# Simple Final Points Formula

For any given field, the final tech level is calculated as:

```
final = current_year + round_down(points + (points * affinity))
final_restricted = lim(restricted, final)
```

For internal calculations the round_down is omitted:

```
final_sim = current_year + points + (points * affinity)
```

# Academia

First, the game is split into Academia levels.

## World

Represents the world baseline. It is not possible to fall further 'behind' in technology.

All other Academia levels are relative to the World Academia level.

Any field stored in the World Academia cannot be restricted.

## Nations

Come after the World Academia.

Each nation has its own Academia level.
Fields in the nation Academia can be restricted.

On the sim tick, each nation Academia level is compared to the World Academia level field by field.

```
for field in nation_academia {
    if field.final_restricted == world_academia.get(field.name).final {
        continue;
    } else if field.final_restricted < world_academia.get(field.name).final {
        // That function would check IF if the nation can really learn from the world and if so do so
        field.learn_from(world_academia.get(field.name));
    } else {
        // That function would check how much the world can learn from the nation
        world_academia.get(field.name).learn_from_restricted(field);
    }
}
```

## Player / AI

Each player and AI has its own Academia level.
They come after a specific nation Academia level.

They interact with their respective Academia parent as `nations` do with `world` Academia.

Important: They too can restrict their fields.
(Or at least try to) -> Events / Espionage etc.

# How Research Works for the Player

The player can create a R&D division.
It will be split into two parts, a research division and a development division.

The 'basic' building block will be a team.

## Research

Driven by characters as much as possible. 

Each team will iterate through its members and add together their points for each field (using their `StatBox`es).
It then can add its own modifiers to the sums.

Each team will be able to work in one of three modes:

1. General
2. Specialised
3. Focused

Each mode will change how the sums calculated are further modified.

### General Research

Team may focus on personal interests / hobbies.

Gives small bonuses to team members (satisfaction, happiness, etc.).

The calculated sums are passed along without modification.

### Specialised Research

Team must focus on a specific field / Task.

No bonuses or maluses to members.

The calculated sums are scaled; 10-20% of the 'wanted' fields (fields that are priortised by the team or the assigned special project), and a 20-30% reduction in irrelevant fields.

### Focused Research

Team must grind the specific field / Task.

Members get maluses to their stats.
These should stack! E.g. just add the same malus as a modifier to the statbox over and over (timed of course).

All output of irrelevant fields is converted to the 'wanted' fields with a 10-15% efficiency.
All output of relevant fields is scaled by at a generous 30%.

### Special Projects

Will unlock capabilites / modifiers. Some for ships, some for constructing ships, some for research, etc.

Will need a specific kind of resource / research point kinds to progress them.

When points are generated for irrelevant fields, about 10% of these points are added to the players academia level relevant.

### General Research

The team produces points that go straight to the player's academia level.

Maybe a flat 10% bonus to all or something.

## Player Interaction

### Indirect by doing things

When integrating purchased components into a ship, the player gets points for the components (well the tech levels used in them).

Some employees may have traits that allow them to passively generate points whatever their occupation.

### Direct via R&D-Division

#### Development

## Designing (part of Development)

On its own special projects only exist 'on paper'. The player will need to design components to include the capability unlocked by the special project.

[Special Project]->[ComponentDesign]->[ComponentPrototype]->[Iterations]->[Integration]->[Component]

# Field Interdependence

Fields are organised into tiers.

- Tier 0: Logic
- Tier 1: Formal
- Tier 2: Natural & Social
- Tier 3: Applied
- Tier 4: Disciplines
- Tier 5: Sub-Disciplines

Each tier is made up of a sub-set of the tier above. These parents provide a soft cap for the tier below.

Should a tier 3 field be behind a tier 2 field, the tier 3 field gets a bonus on its growth.
Should the tier 3 field be ahead of a tier 2 field, the tier 3 field gets a malus on its growth.

The cap is scaling with how much behind / ahead the fields are relative to each other.

In general the player would only ever really interact with tier 3+ fields.
Most interaction would be with tier 4+ fields.

## Tier 0, 1 & 2

These are somewhat special tiers.

They are intended to be the anchor to keep the levels of the rest of the fields in check.
They should largely stay around 'base' (current_year) levels.
They should also not differ largely between nations, nor have restrictions.
