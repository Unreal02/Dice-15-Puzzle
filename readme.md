# 22Fall Dice 15 Puzzle project

- Modified 15-Puzzle with cubes instead of pannel
- Implemented in bevy with Entity-Component-System
- Check gitlab wiki for detailed information

#### Play Link (Web)

- https://dice15puzzle.haje.org
- Generated by CI/CD.

# Todo

- [x] Add Texture on cubes
- [x] Add way to check cube state (Check upperside)
- [x] Code refactor to separate file
- [x] Add puzzle creation logic
  - [ ] Optimize puzzle creation logic
- [x] Add UI
- [x] Add game reset functionality
- [x] Add Game clear logic
  - [ ] Add Game clear UI
- [x] Add Game Play info
  - [ ] UI (Timer, Move Count)
- [x] Add Buffer Input
  - [x] Add Input inversion UI
- [x] Add animation toggle button
- [ ] Add game mode
  - [x] normal
  - [ ] speed solving
  - [ ] daily puzzle

## Maybe
- [ ] Add Ranking
- [ ] Add Credit (External site)

# Study Bevy

- https://bevy-cheatbook.github.io/introduction.html
- https://www.youtube.com/watch?v=QgZfxweAxvc&list=PL6uRoaCCw7GN_lJxpKS3j-KXuThRiSXc6

## WebAssembly

- https://bevy-cheatbook.github.io/platforms/wasm.html

#### Install setup

```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo install -f wasm-bindgen-cli
```

#### Build web version

Already built in GitLab CI/CD.

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --no-typescript --target web ./target/wasm32-unknown-unknown/release/dice_15_puzzle.wasm
```

#### How to build with inspector

- Give --features="debug" option for cargo
