# 22Fall Dice 15 Puzzle project
- Modified 15-Puzzle with cubes instead of pannel
- Implemented in bevy with Entity-Component-System
- Check gitlab wiki for detailed information

# Todo
- [x] Add Texture on cubes
- [x] Add way to check cube state (Check upperside)
- [x] Code refactor to separate file
- [ ] Add puzzle creation logic
- [ ] Add game clear check logic
- [x] Add UI
- [ ] Add game reset functionality

# Study Bevy
- https://bevy-cheatbook.github.io/introduction.html
- https://www.youtube.com/watch?v=QgZfxweAxvc&list=PL6uRoaCCw7GN_lJxpKS3j-KXuThRiSXc6

## WebAssembly
- https://bevy-cheatbook.github.io/platforms/wasm.html
##### Install setup
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo install -f wasm-bindgen-cli
```
##### Build web version
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --no-typescript --target web ./target/wasm32-unknown-unknown/release/dice_15_puzzle.wasm
```