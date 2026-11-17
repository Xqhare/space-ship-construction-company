# Character System Design

Characters drive as much of the game simulation as possible.

---

## 1. Core Structure Overview

*(Implemented across [`src/modules/character/`](file:///home/xqhare/Adytum/rust/space-ship-construction-company/src/modules/character/mod.rs) and [`src/consts/characters/`](file:///home/xqhare/Adytum/rust/space-ship-construction-company/src/consts/characters/mod.rs); see [`skill/design.md`](file:///home/xqhare/Adytum/rust/space-ship-construction-company/src/modules/character/skill/design.md) for the Skill system.)*

```rust
pub struct Character {
    pub name: CharacterName,
    data_box: DataBoxSlotId,
    pub birthday: Utc,
    pub retirement: Utc,
    pub teams: Vec<TeamBoxSlotId>,
    traits: CharacterTraitStore,
    skill: Vec<CharacterSkill>,
}
```

- **Base Values ([`src/consts/characters/base_values/mod.rs`](file:///home/xqhare/Adytum/rust/space-ship-construction-company/src/consts/characters/base_values/mod.rs)):**
  - Fixed starting defaults: `HEALTH` (`1000.0`), `MENTAL_HEALTH` (`1000.0`), `PAIN` (`0.0`), `MONEY` (`0.0`), `HAPPINESS` (`1000.0`).
  - Generated at character init: `IQ`, `SOCIABILITY`, `STRENGTH`.
  - Added dynamically when used/needed: Science fields (`sci_*`).
- **Traits ([`CharacterTraitStore`](file:///home/xqhare/Adytum/rust/space-ship-construction-company/src/modules/character/trait.rs)):**
  - Mandatory single-slot traits: `intellect` and `looks` (rolled at character initialization).
  - Optional single-slot traits: `health`, `mental_health`, `social`, `work_ethic`.
  - Multi-slot traits: `normal_traits` (checked against symmetric `conflicting_traits()`).

---

## 2. Character Pool & Hiring

- Characters live in a centralized `SlotMap<Character>`.
- **Hiring Pool:** A pool of ~1,000 available characters (prospective employees) generated via `Character::new_random`.
- Characters are removed from the pool when hired or after a duration of time passes.
- Higher company reputation / employer attractiveness increases both the skill quality of generated candidates and how long they remain in the pool.

---

## 3. Teams

- Teams are stored centrally in a `SlotMap<Team>`. Each `Character` holds `Vec<TeamBoxSlotId>` for the teams they belong to, and each `Team` holds `Vec<CharacterBoxSlotId>`.
- Some `TeamKind`s (e.g., `Research`) are limited to **one** per character.
- A character can belong to multiple `Management` teams (e.g., Research Team Lead + R&D Division Management).
- *Future optimization:* Cache the `TeamKind`s a character belongs to locally on `Character` to simplify assignment validation.

---

## 4. Init Roll Distributions: IQ & Looks

### IQ (`TRAIT_INTELLECT_IQ_REGISTRY`)
Generated upon character initialization based on a normalized & capped Terman & Merrill (1960) distribution using coin-flip (`random_bool`) branching:

| # | Trait | IQ Range | Probability | Terman & Merrill Ref |
| :- | :--- | :--- | :--- | :--- |
| 1 | **Genius** | 140–149 | `1.5625%` | 1.1% (140–169: 1.33%) |
| 2 | **Astute** | 130–139 | `3.125%` | 3.1% |
| 3 | **Shrewd** | 120–129 | `6.25%` | 8.2% |
| 4 | **Smart** | 110–119 | `12.5%` | 18.1% |
| 5 | **Bright** | 100–109 | `26.5625%` (`25% + 1.5625%`) | 23.5% |
| 6 | **Dim** | 90–99 | `26.5625%` (`25% + 1.5625%`) | 23.0% |
| 7 | **Dull** | 80–89 | `12.5%` | 14.5% |
| 8 | **Dense** | 70–79 | `6.25%` | 5.6% |
| 9 | **Dimwit** | 60–69 | `3.125%` | 2.0% |
| 10 | **Simpleton** | 40–59 | `1.5625%` | 0.63% |

```text
// Coin-flip decision tree (Option 2):
50% -> 25% Bright (100-109) | 25% Dim (90-99)
50% ->
  25% -> 12.5% Smart (110-119) | 12.5% Dull (80-89)
  25% ->
    12.5% -> 6.25% Shrewd (120-129) | 6.25% Dense (70-79)
    12.5% ->
      6.25% -> 3.125% Astute (130-139) | 3.125% Dimwit (60-69)
      6.25% ->
        3.125% -> 1.5625% Bright (100-109) | 1.5625% Dim (90-99)
        3.125% -> 1.5625% Genius (140-149) | 1.5625% Simpleton (40-59)
```

### Looks (`TRAIT_LOOKS_REGISTRY`)
Generated at character initialization (8 tiers, providing modifiers to social interactions):
- **Gorgeous:** `3.125%`
- **Beautiful:** `6.25%`
- **Attractive:** `12.5%`
- **Pretty:** `25%`
- **Appealing:** `25%`
- **Unsightly:** `12.5%`
- **Hideous:** `6.25%`
- **Aberrant:** `3.125%`

---

## 5. Health, Pain, Mental Health, Retirement & Death

### Pain (`0` = No pain, `1000` = Maximum pain)
- Productivity scales down as pain rises; above a threshold (player-defined, nation-state capped), the character stops working entirely.
- Most pain modifiers are timed, though conditions like arthritis or permanent injuries can be permanent.
- Company policies (ergonomic chairs, physiotherapy, mechanical assistance) are the primary way to mitigate permanent pain.

### Health (`1000` = Healthy, `<= 0` = Immediate Death)
- Influenced by traits and injuries (timed or permanent, sustained on or off the job).

### Mental Health (`1000` = Healthy, `<= 0` = Suicide)
- Functions similarly to Pain/Health. Reaching `0` triggers suicide (unless prevented by specific traits).

### Retirement
- Characters retire when they hit the legally required or company-defined retirement age (whichever is younger).
- Retired characters remain in the simulation marked as retired so the player can potentially rehire badly needed talent at an enormous cost (up to a national forced retirement age limit).

### Death & Consequences

1. **Natural Causes:**
   - Primarily affects retirees.
   - If not retired: their team loses a small amount of morale/stats, with no major company-wide event.
2. **Unnatural — Workplace Accident (`health <= 0` from injury):**
   - Major event: large maluses to team members, official work-safety investigation, moderate PR hit.
   - Tracked as an annual counter; exceeding a threshold triggers a massive company-wide crisis.
3. **Unnatural — Suicide (`health <= 0` from pain/mental health or `mental_health <= 0`):**
   - Whether it happens **On-Site** or **Off-Site** depends on the cause (work-related vs. private) and character traits/flags (`SUICIDE_ALWAYS_ON_SITE`, `SUICIDE_NEVER_ON_SITE`, etc.):
   - **On-Site:** Massive event, company-wide stat maluses (worst for team members), police investigation, and major PR disaster. Player responses range from empathetic (*"Shut down production entirely until we know what happened and fix it"*) to dystopian (*"Just put security near the airlocks"* / *"Suppress the news"*).
   - **Off-Site:** Large maluses for team members, minor impact on the rest of the company unless the annual off-site suicide counter crosses a critical threshold.

---

## 6. Future Subsystems: Needs, Memories & Goals

### Needs (Happiness Subsystem)
Values like `comfort`, `fulfilment`, and `environment` work best not as standalone `StatBox` base values, but as a subsystem feeding into `HAPPINESS` and steering character goals (e.g., low fulfilment/environment in a current team causing a character to seek a lateral transfer rather than a promotion into their boss's job).

### Memories & Goals
- **State:** Life goals, current short-term goal (stepping stone toward life goal), good/bad memories, friends (liked characters), and enemies (disliked characters).
- **Simulation Loop:**
  `Pick random employee` -> `Wants to interact` -> `Choose interaction fitting traits & current goal` -> `Pick target character (e.g., same team)` -> `Interact (advancing used skills)` -> `If impactful, record good/bad memory`.
- **Goal Steering:** A character with the life goal *"Become CEO"* adopts *"Get Promoted"* as their current goal, steering them to practice both their current job skills and management/social skills.
