#!/bin/bash
set -eu -o pipefail

case "${1}" in
  verilog)
      LANG_DIR="verilog"
      LANG_EXT="vg"
      ;;
  *)
      echo "OOPS, ARGUMENT EXPECTED TO BE ONE OF THESE VALUES:"
      echo "  verilog  for language type verilog"
      exit
      ;;
esac

mkdir ${LANG_DIR}
cd ${LANG_DIR}

git init -q

# BOTH NAME AND EMAIL ARE NEEDED FOR RECOGNITION
git config --local --add "committer.name" "onefetch-committer-name"
git config --local --add "committer.email" "onefetch-committer-email@onefetch.com"

git config --local --add "user.name" "onefetch-user-name"
git config --local --add "user.email" "onefetch-user-email@onefetch.com"

git config --local --add "author.name" "onefetch-author-name"
git config --local --add "author.email" "onefetch-author-email@onefetch.com"

git remote add origin https://github.com/user/repo.git

git checkout -b main
touch code.${LANG_EXT}
git add code.${LANG_EXT}
git commit -q -m c1
echo hello >> code.${LANG_EXT}
git commit -q -am c2

