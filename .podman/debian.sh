apt update -y
apt upgrade -y
apt install git openssl
git clone https://github.com/toastxc/Reywen-Revolt.git
cd Reywen-Revolt
cargo build --release
cd /
cp Reywen-Revolt/config . -r
