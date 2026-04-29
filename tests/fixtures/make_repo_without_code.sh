set -eu -o pipefail

git init -q

# BOTH NAME AND EMAIL ARE NEEDED FOR RECOGNITION
git config --local --add "committer.name" "onefetch-committer-name"
git config --local --add "committer.email" "onefetch-committer-email@onefetch.com"

git remote add origin https://github.com/user/repo.git

git checkout -b main

# Markdown is exclude by default because it's a prose language (cf. --type option)
touch README.md
git add README.md
git commit -q -m c1
