# Multi-Game Suite in Rust

A collection of classic command-line text games and simulations built entirely in Rust.

## How to Run

To build and launch the main interactive menu game loop, execute:

```bash
cargo run
```

To run the automated verification unit test suites, execute:

```bash
cargo test
```

## Game Rules

1. **Mad Libs**: Players provide various nouns, verbs, and adjectives to fill out a hidden narrative template, producing a humorous custom story.
2. **Hunt the Wumpus**: Players navigate a 20-room cavern grid to track and shoot a monster while avoiding bottomless pits, super bats, and running out of ammunition.
3. **Bulls and Cows**: Players guess a secret numeric code, receiving hints in the form of "Bulls" for correct digits in the right place and "Cows" for correct digits in the wrong place.
4. **Tic-Tac-Toe**: Two players alternate placing their symbols on a 3x3 board attempting to complete an uninterrupted line of three markers horizontally, vertically, or diagonally.
5. **Battleship**: Players guess grid coordinates to locate and sink a hidden enemy fleet of naval vessels concealed on a two-dimensional grid map.
6. **Game of Life**: An automated grid-based cellular automaton simulation where cells survive, die, or reproduce generation by generation based on the population density of their immediate neighbors.
