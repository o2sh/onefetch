set -eu -o pipefail

commit() {
  GIT_COMMITTER_DATE="$1" GIT_AUTHOR_DATE="$1" git commit -q --allow-empty -m "$2"
}

# No tags.
mkdir no_tags
(cd no_tags
  git init -q
  git checkout -q -b main
  commit "2001-01-01T00:00:00Z" c1
)

# Only a tag that points at a tree, not a commit.
mkdir tree_tag
(cd tree_tag
  git init -q
  git checkout -q -b main
  commit "2001-01-01T00:00:00Z" c1
  git tag tree-tag "HEAD^{tree}"
)

# v2.0.0 (annotated) is on the newest commit. v3.0.0 sorts last by name
# and is created last, but on a side-branch commit that is older.
mkdir tags
(cd tags
  git init -q
  git checkout -q -b main
  commit "2001-01-01T00:00:00Z" c1
  git tag v1.0.0
  git checkout -q -b side
  commit "2002-01-01T00:00:00Z" c3
  git checkout -q main
  commit "2003-01-01T00:00:00Z" c2
  git tag -a v2.0.0 -m "v2.0.0"
  git tag v3.0.0 side
)
