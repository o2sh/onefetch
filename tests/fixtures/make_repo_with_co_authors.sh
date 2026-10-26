set -eu -o pipefail

mkdir repo_with_co_authors
cd repo_with_co_authors

git init -q

git checkout -b main
touch code.rs
git add code.rs
git commit -q -m c1 -m "Co-authored-by: Author Two <author2@example.org>" \
  --author="Author One <author1@example.org>"
echo hello >> code.rs
git commit -q -am c2 -m "co-authored-by: Author Three <author3@example.org>" \
  --author="Author Two <author2@example.org>"
echo world >> code.rs
git commit -q -am c3 -m "Co-authored-by: Author One <author1@example.org>" \
  --author="renovate[bot] <bot@example.org>"
