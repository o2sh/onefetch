set -eu -o pipefail

mkdir repo_with_non_origin_remote
cd repo_with_non_origin_remote

git init -q

git remote add upstream https://github.com/user/upstream.git

git checkout -b main
touch this.rs
git add this.rs
git commit -q -m c1
