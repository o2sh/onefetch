set -eu -o pipefail

git init -q

# BOTH NAME AND EMAIL ARE NEEDED FOR RECOGNITION
git config --local --add "committer.name" "onefetch-committer-name"
git config --local --add "committer.email" "onefetch-committer-email@onefetch.com"

git remote add origin https://github.com/user/repo.git

git checkout -b main

# Markdown is prose, so the default `--type programming markup` matches nothing
# here and onefetch has to fall back to every type (cf. #1705)
cat <<'EOF' > README.md
# Title

Some prose, and no code at all.
EOF
git add README.md
git commit -q -m c1
