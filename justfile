build:
    wasm-pack --quiet build --target web --out-name test --release
    wasm-opt -Os -o test-opt.wasm pkg/test_bg.wasm
    @ruby -e "puts \"test-opt.wasm: #{File.size(\"test-opt.wasm\")} bytes\""