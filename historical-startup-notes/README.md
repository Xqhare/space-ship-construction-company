
> Game engine is the wrong term, I want it to handle some? game logic and state handling, not graphics.
> No Physics, collision, etc. to keep the scope small

To continue work on space-ship-construction-company, I would like to make a game engine for the game.

I have no idea of what I am doing. The perfect place to start!

> Rescoped from v0 after months of thinking.

# Goals

1. Game engine for space-ship-construction-company
2. No graphics, only game logic and state handling
3. Playable without an UI or graphics
4. Usable for more than just space-ship-construction-company

## 3. Playable without an UI or graphics

ThePrimagen has written a Tower defense game with a `json mode` as he calls it.
This returns a structured json file with the game state.
It then accepts a json file as input and calculates the next game state from it.

Using this approach as the backend for a game engine is a good idea.
I can easily hook into that API for a CLI, TUI, etc.
It also makes it possible for AI agents to play the game (could be very useful for testing, debugging and general development using AI).

This adds one constraint:
The game state needs some kind of turn based system.
Why? In `json mode` the game state is static and a player needs time to parse that and then decide what to do.
Real time is hard, but it could be done paradox style (each turn is a day / hour / second; Turns are autoexecuted with a certain frequency and a pause button to halt autoexecution).

# Constraints

1. No graphics, only game logic and state handling
2. Playable without an UI or graphics
3. Turn based system
4. Generalised for more than just space-ship-construction-company

## Generalisation

Presents the largest hurdle for me and the most difficult part of this project.
I have never written a game engine before. So its difficult to decide what and how to generalise.

Main Idea at the moment:

Trait based.
The game engine only provides traits for game logic and state handling.

For example Researching a technology:

Needs a trait for a specific technology and a way of interacting with that technology.

```rust
trait Technology {
    fn researched(&self) -> bool;
    fn name(&self) -> &str;
    fn cost(&self) -> Vec<(Rc<Resource>, i32)>;
    fn time(&self) -> u32; // Probably counting down in # turns
    fn add_resources(&mut self, &Vec<(Rc<Resource>, i32)>) -> (bool, i32); // Returns true if technology was researched and how much is left over of the resource
    fn add_resource(&mut self, &Rc<Resource>, i32) -> (bool, i32);
    fn can_unlock(&self, available_technologies: Rc<AvailableTechnologies>) -> bool {
        if self.cost.iter().all(|(_, amount)| amount <= 0) && self.time == 0 {
            let mut prerequisites_met = true;
            for prerequisite in self.prequisites() {
                if !available_technologies.contains(&prerequisite) {
                    prerequisites_met = false;
                    break;
                }
            }
            prerequisites_met
        } else {
            false
        }
    };
    fn unlock(&mut self); // Set researched to true
    fn prequisites(&self) -> Vec<Rc<Technology>>;
    fn effects(&self) -> Vec<Rc<Effect>>;
}

trait Resource {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
    fn effects(&self) -> Vec<Rc<Effect>>;
}

trait Effect {
    fn name(&self) -> &str;
    fn value(&self) -> i32;
    fn effects(&self) -> Vec<Rc<Effect>>;
    fn description(&self) -> String;
    fn duration(&self) -> i32;
    fn constraints(&self) -> Vec<Rc<Constraint>>;
}

struct ResearchModule {
    all_tech: AllTechnologies,
    researched_tech: ResearchedTechnologies,
    available_tech: AvailableTechnologies
}

trait ResearchedTechnologies {
    fn researched_technologies(&self) -> Vec<Rc<Technology>>;
    fn add_technology(&mut self, technology: Rc<Technology>) -> bool;
    fn add_technologies(&mut self, technologies: Vec<Rc<Technology>>) -> bool;
    fn set_technologies(&mut self, technologies: Vec<Rc<Technology>>);
    fn remove_technology(&mut self, technology: Rc<Technology>) -> bool;
}
trait AvailableTechnologies {
    fn available_technologies(&self) -> Vec<Rc<Technology>>;
    fn add_technology(&mut self, technology: Rc<Technology>) -> bool;
    fn add_technologies(&mut self, technologies: Vec<Rc<Technology>>) -> bool;
    fn set_technologies(&mut self, technologies: Vec<Rc<Technology>>);
    fn remove_technology(&mut self, technology: Rc<Technology>) -> bool;
}

trait AllTechnologies {
    fn all_technologies(&self) -> Vec<Rc<Technology>>;
    fn unlock_all(&mut self);
    fn lock_all(&mut self);
    fn available_technologies(&self) -> Vec<Rc<Technology>>;
    fn remove_available_technologies(&mut self);
    fn set_all_technologies(&mut self, technologies: Vec<Rc<Technology>>);
}
```
