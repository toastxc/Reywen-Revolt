#!/bin/bash
reywen=$( ls | grep Reywen-Revolt )
config=$( ls | grep config  )

# pull Reywen
if  [ "$reywen" = "" ]; then
        apt update -y
        apt upgrade -y
        apt install git openssl
        cargo update
        git clone https://github.com/toastxc/Reywen-Revolt.git
        chmod 777 -R /Reywen-Revolt
        cd Reywen-Revolt
        cargo build --release
        cd /

        # if not exists, copy config
        if [ "$config" = "" ]; then

                cp Reywen-Revolt/config . -r
                echo "root config created"
        else
                echo "root config detected, skipping"
        fi
else
        # start reywen
        echo "Starting reywen..."
        echo "Directory: $PWD"
        ./Reywen-Revolt/target/release/reywen2
fi
