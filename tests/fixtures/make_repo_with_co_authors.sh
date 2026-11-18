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
echo again >> code.rs
git commit -q -am c4 -m "Co-authored-by: Author One <author1@example.org>
Co-authored-by: Author One <author1@example.org>" \
  --author="Author Three <author3@example.org>"
echo more >> code.rs
git commit -q -am c5 -m "Co-authored-by: Author Two <author2@example.org>
Co-authored-by: Author Three <author3@example.org>" \
  --author="Author One <author1@example.org>"
git commit -q --allow-empty -m c6 -m "Signed-off-by: Author Two <author2@example.org>
Reviewed-by: Author Three <author3@example.org>" \
  --author="Author One <author1@example.org>"
git commit -q --allow-empty -m c7 -m "CO-AUTHORED-BY: Author One <author1@example.org>
Co-Authored-By: Author Three <author3@example.org>" \
  --author="Author Two <author2@example.org>"
git commit -q --allow-empty -m c8 -m "Co-authored-by: Author Three <author3@example.org>" \
  --author="Author Three <author3@example.org>"
git commit -q --allow-empty -m c9 -m "Co-authored-by: just a name
Co-authored-by: Author Two <author2@example.org>" \
  --author="Author One <author1@example.org>"
git commit -q --allow-empty -m c10 -m "Co-authored-by: dependabot[bot] <bot@example.org>" \
  --author="Author Two <author2@example.org>"
cat > .mailmap <<'EOF'
Author One <author1@example.org> <old1@example.org>
Author Three <author3@example.org> <author3-alias@example.org>
EOF
git add .mailmap
git commit -q -m c11 -m "Co-authored-by: Old Name <old1@example.org>
Co-authored-by: Alias <author3-alias@example.org>" \
  --author="Author Three <author3@example.org>"

# A signed commit whose signature header holds a blank line, so the
# message must be told apart from the headers correctly.
date=$(git log -1 --format=%cd --date=raw)
commit=$(printf '%s\n' \
  "tree $(git rev-parse 'HEAD^{tree}')" \
  "parent $(git rev-parse HEAD)" \
  "author Author One <author1@example.org> $date" \
  "committer Author One <author1@example.org> $date" \
  "gpgsig -----BEGIN PGP SIGNATURE-----" \
  " " \
  " signature" \
  " -----END PGP SIGNATURE-----" \
  "" \
  "c12" \
  "" \
  "Co-authored-by: Author Two <author2@example.org>" |
  git hash-object -t commit -w --stdin)
git update-ref HEAD "$commit"
