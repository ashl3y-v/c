#!/bin/sh
for d in */ ; do
    echo "$d"
    cd "$d"
    git status
    cd ../
done
