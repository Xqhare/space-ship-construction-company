
As with Trait, I am thinking of splitting a skill into two parts.

1. Static reference containing the name and affected base value pair name.
2. The actual `CharacterSkill` struct living in the `Character.skills` field. 

# Static

Again, as with trait, I want a single source of truth, preferably by defining all needed info in one call chain.

There is one caveat:

Most `const::char::base_values` need a 'Master' skill.

When a character needs to do an action it has no stats for, I want to add the relevant base value to the stat box, along with the Master skill at level `Low(0)` (and a calculated affinity).

I will also want a `all_affecting_skills(base_value_name)` function alongside.
I can then filter said skills to whatever I need at the callsite itself.

Some base values (Health, Mental Health etc) dont really have a 'skill' one could master (Well, you can be more mindful of your health and try to lead a healty livestyle to help with both, and you could get better at it in real life.).
Others (like Socialbility) could be influenced by a large amount of skills (à la Dwarf fortress: Intimidation, Flattery, Lying etc) (I also just find it fun having a 'Master Liar' as a dwarf; That might be just as fun here.)

# Struct

Created from the static with a `new(skill_name)` (gets the static const from the reg itself) and `from_skill(&ConstSkill)`

That way I can chain `all_affecting` into a for loop and add all with `from_skill` OR only add the master by `new(master_skill(base_value_name))`
