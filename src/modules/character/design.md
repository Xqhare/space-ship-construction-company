
I plan on having characters drive as much of the game as possible.


```rust

struct {
    name: Name,
    stats: DataBoxId,
    birthday: Utc,
    traits: Vec<Trait>,
    skills: Vec<Skill>
}

struct Skill {
    name: String,
    affected_base_value_pairs: Vec<(String, HpFloat)>,
    affinity: HpFloat,
    points: HpFloat
    level: Level
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

const LEVEL_AMOUNT_LOW: u8 = 10;
const LEVEL_AMOUNT_MEDIUM: u8 = 10;
const LEVEL_AMOUNT_HIGH: u8 = 10;
const LEVEL_AMOUNT_ULTRA: u8 = 3;

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

## IQ

Each character has an IQ (Intelligence Quotient).
It is generated upon character init.

It follows (As stolen from: https://en.wikipedia.org/wiki/IQ_classification on 2026-11-01; Terman & Merrill 1960, p. 18):

1. 0.03% - 160-169
2. 0.2% - 150-159
3. 1.1% - 140-149
4. 3.1% - 130-139
5. 8.2% - 120-129
6. 18.1% - 110-119
7. 23.5% - 100-109
8. 23.0% - 90-99
9. 14.5% - 80-89
10. 5.6% - 70-79
11. 2.0% - 60-69
12. 0.4% - 50-59
13. 0.2% - 40-49
14. 0.03% - 30-39

## Memories

At some point, maybe, I would like a simple memory system.

Timed modifiers kind of are like memories, but I want something more complex.

## Health, Death and Retirement

Each character has a health, mental health and pain stat.

#### Pain

Ranging from 0 (No pain) to 100 (Maximum pain).

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

The values are flipped, 100 is good mental health, 0 is no mental health.

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
