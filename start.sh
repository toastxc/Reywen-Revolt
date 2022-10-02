git clone https://github.com/toastxc/Reywen-Revolt.git
cd Reywen-Revolt
cargo build
vim auth.json
vim bridge.json
cp target/debug/reywen2 .
rm archive  Cargo.lock  Cargo.toml src target LICENSE  README.md target -rf
echo "done"
