#!/usr/bin/env bash

today=$(printf %02d $1)
cargo new day_$today

yest=$(printf %02d $(($1-1)))

sed -i s/day_$yest/day_$today/ .vscode/launch.json
mkdir day_$today/input
touch day_$today/input/demo.txt
touch day_$today/input/input.txt