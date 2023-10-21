#!/bin/sh
for d in */ ; do
    echo "$d"
    cd "$d"
    git pull
    cd ../
done
