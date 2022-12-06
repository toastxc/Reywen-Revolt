# directory on filesystem for all reywen volumes
dir=$( echo "/home/$user/Downloads/reywen")

podman run  -d -it  \

        # name in podman
        --name reywen-server \
        
        #config files storage
        -v $dir/config:/config \

        # needed for communication with mongodb
        --pod reywen \

        -restart=always \

        # Container name not yet known
        # This script is not functional                              
