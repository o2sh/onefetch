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

#
# expected output generated using this command in repo directory
# onefetch --show-logo never --ascii-language <language>
#

case "${1}" in
  verilog)
cat >expected << '__expected_output__'
[39;1monefetch-committer-name[0m [39;1m~[0m [39;1mgit version 2.37.2[0m
[39m--------------------------------------------[39m
[39;1mProject[0m[39;1m:[0m [39mrepo[39m
[39;1mHEAD[0m[39;1m:[0m [39m671ed7b (main)[39m
[39;1mPending[0m[39;1m:[0m [39m1+[39m
[39;1mCreated[0m[39;1m:[0m [39m22 years ago[39m
[39;1mLanguage[0m[39;1m:[0m [39m[41m                          [49m
          [31m●[39m [39mVerilog (100.0 %)[39m [39m
[39;1mAuthor[0m[39;1m:[0m [39m[39m100% author 2[39m[39m
[39;1mLast change[0m[39;1m:[0m [39m22 years ago[39m
[39;1mRepo[0m[39;1m:[0m [39mhttps://github.com/user/repo.git[39m
[39;1mCommits[0m[39;1m:[0m [39m2[39m
[39;1mLines of code[0m[39;1m:[0m [39m1[39m
[39;1mSize[0m[39;1m:[0m [39m6 B (1 file)[39m

[40m   [49m[41m   [49m[42m   [49m[43m   [49m[44m   [49m[45m   [49m[46m   [49m[47m   [49m
__expected_output__
      ;;
esac
