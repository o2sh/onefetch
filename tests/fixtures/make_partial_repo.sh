#!/bin/bash
set -eu -o pipefail

mkdir base
(cd base
    git init -q
    git checkout -b main
    touch code.rs
    git add code.rs
    git commit -q -m c1
    echo hello >> code.rs
    git add code.rs
    git commit -q -m c2
    echo world >> code.rs
    git add code.rs
    git commit -q -m c3
    echo something >> code.rs
    git add code.rs
    git commit -q -m c4
    echo more >> code.rs
    git mv code.rs renamed.rs
    echo change >> renamed.rs
    git commit -q -am c5

    git config uploadpack.allowfilter true
)

git clone --filter=blob:none file://$PWD/base partial
(cd partial
    git config diff.renames true
)
