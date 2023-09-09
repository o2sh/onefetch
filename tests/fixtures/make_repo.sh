#!/bin/sh
set -eu -o pipefail

git init -q

# BOTH NAME AND EMAIL ARE NEEDED FOR RECOGNITION
git config --local --add "committer.name" "onefetch-committer-name"
git config --local --add "committer.email" "onefetch-committer-email@onefetch.com"

git remote add origin https://github.com/user/repo.git

git checkout -b main
touch code.rs
git add code.rs
git commit -q -m c1 --author="Author One <author1@example.org>"
git tag tag1
echo hello >> code.rs
git add code.rs
git commit -q -m c2 --author="Author Two <author2@example.org>"
echo world >> code.rs
git add code.rs
git commit -q -m c3 --author="Author Three <author3@example.org>"
echo something >> code.rs
git add code.rs
git commit -q -m c4 --author="Author Four <author4@example.org>"
echo more >> code.rs

cat > Cargo.toml << EOF
[package]
name = "repo"
version = "0.1.0"
description = "Amazing tool"

[dependencies]
anyhow = "1.0.65"
EOF

cat > LICENSE << '__LICENSE__'
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
__LICENSE__


