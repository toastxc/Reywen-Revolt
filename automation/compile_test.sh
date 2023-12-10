echo "compiling"

# standard (hyper)
mold --run cargo b --features standard

# wasm (reqwasm)
mold --run cargo b --features wasm


echo "testing"
mold --run cargo test --lib fetch --features standard

# not currently working
#mold --run cargo test --lib fetch --features wasm

