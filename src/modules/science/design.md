# Science & Research System Design

## 1. Core Structure Overview

Static field definitions and their tier/parent DAG live in [`src/consts/base_values/science.rs`](file:///home/xqhare/Adytum/rust/space-ship-construction-company/src/consts/base_values/science.rs) via `ConstScienceField` and `SCI_REGISTRY_OF_REGISTRIES`.

The runtime research module tracks dynamic progress per field across different `Academia` levels:

```rust
pub struct ResearchFields {
    /// Keyed by `ConstScienceField.name` (or `&'static ConstScienceField`)
    fields: HashMap<&'static str, FieldState>,
}

pub struct FieldState {
    pub field: &'static ConstScienceField,
    pub affinity: HpFloat,
    pub points: HpFloat,
    /// A nation or company can restrict the tech level of a field.
    /// Tech levels that exceed `current_year + restricted` will not leak points to higher academia levels.
    /// Small number == strict restriction; Large number == loose restriction.
    pub restricted: Option<u8>,
}

pub struct Academia {
    pub world: ResearchFields,
    pub player: ResearchFields,
    pub nations: HashMap<StateName, ResearchFields>,
    pub ai: HashMap<AiCompName, (StateName, ResearchFields)>,
}
```

---

## 2. Points, Affinity & Tech Level Formula

- **Points (`points >= 0`):** The base bonus (in years) above `current_year`.
- **Affinity:** Increases whenever points are added to a field, and decays once per year (slower once it reaches `0`, though negative affinity is possible).
  - *Note on tick frequency:* Yearly decay keeps simulation overhead low compared to monthly/quarterly ticks.

### Final Tech Level Calculation
For display and integer checks (e.g., unlocking special projects or component base values):
```text
final = current_year + round_down(points + (points * affinity))
final_restricted = lim(current_year + restricted, final)
```
For internal simulation calculations (`final_sim`), `round_down` is omitted:
```text
final_sim = current_year + points + (points * affinity)
```

---

## 3. Field Interdependence (Tiers & Parent Soft Caps)

Fields are organized into 6 tiers in [`src/consts/base_values/science.rs`](file:///home/xqhare/Adytum/rust/space-ship-construction-company/src/consts/base_values/science.rs):
- **Tier 0 (`T0Logic`):** Logic (Root)
- **Tier 1 (`T1Formal`):** Mathematics, Philosophy, Empiricism
- **Tier 2 (`T2Natural` & `T2Social`):** Physics, Biology, Chemistry, Economics, Psychology, etc.
- **Tier 3 (`T3Applied`):** Meteorology, Optics, Materials, Quantum Physics, etc.
- **Tier 4 (`T4Discipline`):** Engineering, Corporate Law, etc.
- **Tier 5 (`T5SubDiscipline`):** Computer Science, Artificial Intelligence, Aerospace Engineering, etc.

### Parent Soft Caps & Catch-up Bonuses
Each field (above Tier 0) lists its parent fields in `field.parents()`. Parents act as an anchor/soft cap for their children:
- If a child field is **behind** its parent field(s), it receives a **growth bonus**.
- If a child field is **ahead** of its parent field(s), it receives a **growth malus**.
- The bonus/malus scales with how far behind or ahead the child field is relative to its parents.
- **Tiers 0, 1 & 2** act as global anchors: they stay close to `current_year`, do not vary wildly between nations, and cannot be restricted. Players primarily interact with **Tier 3+** (and mostly **Tier 4+**).

---

## 4. Academia Hierarchy & Restrictions

Technology diffuses between three layers of Academia: **World** <-> **Nations** <-> **Player / AI Companies**.

1. **World Academia:**
   - The global baseline. No field in World Academia can be restricted.
2. **Nation Academia:**
   - Sits below World Academia. On each simulation tick, each nation compares its fields against World Academia:
     ```rust
     for field in nation_academia {
         let world_field = world_academia.get(field.field.name);
         if field.final_restricted < world_field.final_sim {
             field.learn_from(world_field);
         } else if field.final_restricted > world_field.final_sim {
             world_field.learn_from_restricted(field);
         }
     }
     ```
   - **Nation-level Restriction:** Any component or ship using a restricted tech level (or higher) can only be sold to that nation state or on its domestic market.
3. **Player & AI Company Academia:**
   - Sits below a parent Nation Academia and exchanges knowledge with it using the same `learn_from` / `learn_from_restricted` mechanism.
   - **Company-level Restriction:** Companies can restrict their own fields (subject to espionage/events). Components using company-restricted tech can only be sold to the parent nation state; selling them on the open national market opens the tech to that market.

---

## 5. Player Interaction & R&D Division

The player's R&D division is split into **Research** and **Development**, operated by character [`Team`](file:///home/xqhare/Adytum/rust/space-ship-construction-company/src/modules/team/mod.rs)s.

### Research Teams & Work Modes
Each research team sums its members' science field outputs from their `StatBox`es, applies team modifiers, and operates in one of three modes:

| Mode | Member Impact | Output Scaling |
| :--- | :--- | :--- |
| **1. General** | Small bonuses to satisfaction / happiness (members pursue personal interests). | Sums passed along unmodified (+ possible flat ~10% bonus to general academia). |
| **2. Specialised** | Neutral (no bonuses or maluses). | `+10–20%` to prioritized/project fields; `-20–30%` to irrelevant fields. |
| **3. Focused** | Stacking timed stat maluses on team members (crunch/grind). | `+30%` to prioritized fields; irrelevant field output converted into prioritized fields at `10–15%` efficiency. |

- **General Research Assignment:** Points go directly to the player's Academia levels.
- **Special Projects Assignment:** Unlocks new capabilities or modifiers (for ships, ship construction, research, etc.). Relevant field points progress the project; ~10% of irrelevant field points spill over into the player's Academia.
- **Indirect Research:**
  - Integrating purchased third-party components into a ship grants research points toward the tech levels used in those components.
  - Employees with traits like `Curious` (`ALWAYS_PROVIDES_RESEARCH`) passively generate research points regardless of their job.

### Development Pipeline (Designing Components)
Special projects only unlock a capability "on paper." To use it on a ship, the Development division must take it through:
```text
[Special Project] -> [ComponentDesign] -> [ComponentPrototype] -> [Iterations] -> [Integration] -> [Component]
```
The tech levels (years) of the relevant research fields at design time determine both the available special projects and the base values of the resulting component.
