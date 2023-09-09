#!/bin/sh
set -eu -o pipefail

git init -q

git checkout -b main
touch this.rs
git add this.rs
git commit -q -m c1
echo hello >> this.rs
git commit -q -am c2
