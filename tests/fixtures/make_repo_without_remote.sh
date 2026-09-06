set -eu -o pipefail

mkdir repo_without_remote
cd repo_without_remote

git init -q

git checkout -b main
touch this.rs
git add this.rs
git commit -q -m c1
echo hello >> this.rs
git commit -q -am c2
