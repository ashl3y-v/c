#!/bin/sh
for d in */ ; do
    echo "$d"
    cd "$d"
    git pull
    git add -A
    git status
    git commit -m "update"
    git push
    cd ../
done

./c/diff.sh
