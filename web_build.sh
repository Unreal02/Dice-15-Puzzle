cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web_build/ --no-typescript --target web ./target/wasm32-unknown-unknown/release/dice_15_puzzle.wasm
cp -r assets/ web_build/assets/
git add web_build/*
git commit -m "web_build"
git push origin master
git subtree push --prefix web_build origin build