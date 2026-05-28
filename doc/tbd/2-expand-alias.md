---
status: TODO
title: Support debug output (expansion) from `~/conf/prj/alias`.
---

See `2-expand-alias/0001-WIP-adding-debug-expansion-output-to-alias.patch` for a start on this. (Committing a patch to a repo is ironic, but files are easier to manage, search, etc. than commits; nor do I want the changes to appear as commits in Git logs.) It was generated thus:

```sh
jj log -r onqvmnzl --no-graph -T 'commit_id ++ "\n"'
# 4b31626ad2b9936828c27bf49d381371b2ba1217

git format-patch -1 4b31626ad2b9936828c27bf49d381371b2ba1217
# 0001-WIP-adding-debug-expansion-output-to-alias.patch
```
