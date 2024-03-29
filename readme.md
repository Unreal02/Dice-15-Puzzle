# 22Fall Dice 15 Puzzle project

- Modified 15-Puzzle with cubes instead of pannel
- Implemented in bevy with Entity-Component-System
- Check gitlab wiki for detailed information

#### Play Link (Web)

- https://dice15puzzle.haje.org
- Generated by CI/CD.

# Todo
- [x] Puzzle Share & Load
  - [x] create URL for Puzzle
  - [x] Store URL2Puzzle mapping in DB
  - [x] Create Puzzle with current puzzle state
    - [x] Share Button -> create URL for current state 
  - [x] Puzzle Load: URL to http request -> get puzzle from DB & load it in normal mode

- [ ] DataBase
  - [ ] Main server: Redis?
    - [x] URL to Puzzle
    - [x] DailyPuzzle Info
      - [ ] DailyPuzzle Ranking (Time, Move separately)
  - [ ] Backup: RDB (mysql)
    - [ ] Save rawdata & restore redis from rawdata

- [ ] UI
  - [x] Mode selection
  - [ ] ClearUI with play info (Time, Movement, undo 했는지 표시)
  - [x] Load button
  - [x] Store button -> Need to copy to clipboard
  - [x] Undo/Redo Button
  - [ ] More beautiful UI?

- [ ] Game Logic
  - [x] Problem fix: timer reset after mode selection
  - [x] Undo / Redo
  - [x] Measure time after first move
  - [x] Daily Puzzle
    - [x] Daily Puzzle creation & DB update
    - [ ] Improve UI

- [ ] Exernal site
  - [x] Credit
  - [x] Tutorial?

- [x] Statistics export: clipboard

# TBD
- [ ] Minimal Movement mode
- [ ] Game Replay
- [ ] Tutorial mode?

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

#### Rust Analyzer Settings

in `.vscode/settings.json`:

```
"rust-analyzer.cargo.target": "wasm32-unknown-unknown",
"rust-analyzer.check.targets": "wasm32-unknown-unknown",
```

#### Build web version

Already built in GitLab CI/CD.

```
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web_build/ --no-typescript --target web ./target/wasm32-unknown-unknown/debug/dice_15_puzzle.wasm
```

#### Run web version

```
cargo run --target wasm32-unknown-unknown
```

Expected output:
```
INFO wasm_server_runner: compressed wasm output is 5.27mb large
INFO wasm_server_runner::server: starting webserver at http://127.0.0.1:1334
```

#### How to build with inspector

- Give --features="debug" option for cargo
