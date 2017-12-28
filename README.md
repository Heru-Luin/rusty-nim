# rusty-nim
A Nim game in Rust.

## What is this ?

This is a substraction variant of the Nim game in Rust ([Nim on Wikipedia](https://en.wikipedia.org/wiki/Nim#The_subtraction_game_S(1,_2,_._._.,_k))).

## Rules

This is a human vs IA board game.

- At the start of each round, there are 21 sticks on the board.
- Human plays first.

The player removes **1, 2 or 3 sticks** from the board.

Once removed, it's the other player's turn.
The game continues until there are no more sticks on the board.

**The player who takes the last stick wins the round.**

## Build
Use [cargo](http://doc.crates.io/guide.html) to build this project :
```
cargo build --release
```

## Run
```
./target/release/rusty-nim
```

Enjoy
