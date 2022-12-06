reywen=$( echo ls | grep Reywen-Revolt )

if [[ $reywen = "" ]]; then

        apt update -y
        apt upgrade -y
        apt install git openssl
        git clone https://github.com/toastxc/Reywen-Revolt.git
        cd Reywen-Revolt
        cargo build --release
        cd /
        cp Reywen-Revolt/config . -r
else
        ./Reywen-Revolt/target/release/reywen2


fi
