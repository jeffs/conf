---
status: TODO
title: rebase clone should track fork bookmarks
---

After `rebase clone`, configured bookmarks exist only as untracked remote
bookmarks (e.g. `custom@origin`), so the first rebase fails with
`Revision 'custom' doesn't exist`. Have the clone op run
`jj bookmark track <bookmark>@origin` for each bookmark in the manifest,
so freshly cloned forks work without manual intervention.
