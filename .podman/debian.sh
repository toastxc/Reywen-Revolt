#!/bin/bash
reywen=$( ls | grep Reywen-Revolt )
config=$( ls | grep config  )

# pull Reywen
if  [ "$reywen" = "" ]; then
        apt update -y
        apt upgrade -y
        apt install git openssl
        git clone https://github.com/toastxc/Reywen-Revolt.git
        chmod 777 -R /Reywen-Revolt
        cd Reywen-Revolt
        cargo update
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
        cd Reywen-Revolt
        echo "Directory: $PWD"
        cargo update
        cargo r -r
fi
