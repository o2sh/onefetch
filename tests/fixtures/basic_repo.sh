#!/bin/bash
set -eu -o pipefail

git init -q

# BOTH NAME AND EMAIL ARE NEEDED FOR RECOGNITION
git config --local --add "committer.name" "onefetch-committer-name"
git config --local --add "committer.email" "onefetch-committer-email@onefetch.com"

git config --local --add "user.name" "onefetch-user-name"
git config --local --add "user.email" "onefetch-user-email@onefetch.com"

git config --local --add "author.name" "onefetch-author-name"
git config --local --add "author.email" "onefetch-author-email@onefetch.com"

git checkout -b main
touch this
git add this
git commit -q -m c1
echo hello >> this
git commit -q -am c2
