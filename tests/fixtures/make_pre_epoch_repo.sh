#!/bin/bash
set -eu -o pipefail

git init -q
git checkout -b main

echo "hello\nworld" >> code.rs
git add code.rs
GIT_AUTHOR_DATE="@0 +0000" GIT_COMMITTER_DATE="@0 +0000" git commit -q -m c1
git cat-file -p @ > to-be-patched.txt

patch -p1 <<EOF
diff --git a/to-be-patched.txt b/to-be-patched.txt
index 95ad1b1..3ea89af 100644
--- a/to-be-patched.txt
+++ b/to-be-patched.txt
@@ -1,5 +1,5 @@
 tree 00d3a67028ba1004a04bd720eee966811102f0c3
-author author <author@example.com> 0 +0000
-committer committer <committer@example.com> 0 +0000
+author author <author@example.com> -5263747740 +0009
+committer committer <committer@example.com> -5263747740 +0009

 c1
EOF

new_commit=$(git hash-object -w -t commit to-be-patched.txt || git hash-object --literally -w -t commit to-be-patched.txt)
git update-ref refs/heads/main $new_commit

