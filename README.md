# base_defense

A litte tower defense game with Rust and Bevy engine. It's my first litte game project, only to learn Rust/Bevy.

Base defense concept:

- There is one Base board and one defense board
- there are resources you can earn in power plants/factories and spend in defense
- The game is over if you have no more resources and if you can't get positive count of resources again after a countdown
- Energy (produced by power plants and gets stored in main base building)
- Materials (produced by factories, factories need energy)
  (- Buildings and towers have levels)
- Base has own defense system and if it got hit by an enemy, it will consume resources to defend. If there are no resources anymore, the game is over.
- Base consumes energy constantly. If there is a specific time no energy, the game is over
- Base has level, towers get unlocked with base level
- Destroing an enemy results in a little reward of materials
- Controls:
  - Shift: Show all tower ranges
- Enemies sometimes drop special items on death
- Advanced road traffic system
- Special event waves
- tower self-destroy mode: kills evey enemy on board
- Random special items:

  - Bomb dropping
  - More temp. tower damage
  - Slower enemies

  TODO:

  - Wave ticker ✓
  - Z-Layer Shots ✓
  - Enemy resource drops at death ✓
  - Building/tower costs ✓
  - fast forward function ✓
  - tank enemy type ✓
  - game over countdown ✓
  - spawn anti collision system
  - anti enemy collision system (enemys getting slower if one slower enemy is before them) ✓
  - improve enemy collision system
  - Tower Target System: target setting
  - Soundsystem: Background Music, Ingame Sounds
  - Upgrade system
