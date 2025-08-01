#!/bin/bash

# Store current directory
initialDir=$(pwd)

# Change directory to home
cd go-book
# Run command
git ls-tree -r --name-only HEAD --full-tree src/es | xargs -I {} git log -1 --format="%aI,go-book/{}" {} | grep "\.md$" | awk -F "." '{print $1}'

# Change back to initial directory
cd "$initialDir"
