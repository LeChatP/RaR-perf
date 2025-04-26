#!/usr/bin/sh
echo "$1" "$2" "$3" "$4"
bin/confgen etc/sudoers etc/rootasrole.json "$1" "$2" "$3" "$4" "1000"
sleep 1