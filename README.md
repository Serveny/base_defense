# base_defense

A litte tower defense game with Rust and Bevy engine. It's my first litte game project, only to learn Rust/Bevy.

Base defense concept:

- There is one Base board and one defense board
- there are resources you can earn in the base and spend in defense
- Energy (produced by power plants and gets stored in main base building)
- Materials (produced by factories)
  (- Buildings and towers have levels)
- Base has own defense system and if it got hit by an enemy, it will consume resources to defend. If there are no resources anymore, the game is over.
- Base consumes energy constantly. If there is a specific time no energy, the game is over
- Base has level, towers get unlocked with base level
- Controls:
  - Shift: Show all tower ranges
