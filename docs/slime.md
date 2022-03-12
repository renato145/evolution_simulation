The main evolving creature.
- It consume 1 energy on every X seconds.
- Each movement consumes Y energy (proportional to its size).
- When energy is less than a threshold S, its free to move.
- By consuming A energy, it can jump to eat.
  - Slime requires to have at least H energy to be able to jump.
  - Jump has a time cooldown.
- Every P energy it will evolve.
- When meeting another slime, if energy is at least M and the difference
  is not less than the smaller slime's energy, a new slime will spawn.
- If energy reaches 0, the slime will die.
- If no food has been consumed on the last W seconds, the slime will die.

### Slime evolution (skills)
- There are 3 evolving paths, the fist time it evolves the slime will randomly
  choose a path and will follow it on next evolutions.
- A maximum of H skills levels can be hold at the same time.
- When a new slime is spawned, if the parents already have some skills it will
  inherit them, choosing randomly for each parent and reducing its level by
  half (rounding up).
- Skill paths:
  1) Vision: increase the range of vision to detect food.
  2) Efficiency: reduces the energy needed to move around.
  3) Jumper: reduces jump cooldown.