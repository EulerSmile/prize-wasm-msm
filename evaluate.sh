# Compiles reference into a `.wasm` module
cd reference
wasm-pack build --release
cd ..

# Compiles submission into a `.wasm` module
# Feel free to change
cd submission
cp submission.wasm ../www
cd ..

cd euler
CC=emcc AR=llvm-ar wasm-pack build --release
cp pkg/euler_bg.wasm ../www
cp pkg/euler_bg.js ../www
cd ..

# Evaluation
cd ./www
npm install
npm run start
