---
status: TODO
title: dynamic claude settings
---

# Context

Claude Code defaults to storing various settings, such as the model to use for new chats, in a user-level `settings.json` file. (Mine is symlinked from `~/.claude/settings.json` to `~/conf/etc/claude/settings.json`). CC also allows folder-level overrides in intrusive `.claude` directories, within which some files are versioned, whereas others are excluded.

Moreover, user- and folder-scope `.claude` directories include not only settings, but also a wide array of data such as rules and skills. This is the reason only individual files (`settings.json`, `CLAUDE.md`), rather than the entire `claude` directory, are currently symlinks into `~/conf`.

In short: Claude Code's configuration support is built around a single config per machine, with overrides per physical directory. This paradigm is at odds with mixing and matching config across machines or projects, which is how the `jeffs/conf` system of sharing and overrides is meant to work.

# Problems

1. The atomicity of `settings.json` (and `settings.local.json`) makes it essentially impossible to share a subset of settings (such as default model or effort level) across machines, or across folders.

2. Folder-level `.claude/settings.local.json` files are not preserved across Jujutsu workspaces, nor fresh clones of repositories.

3. Claude Code frequently reorders the entries in `settings.json`, causing meaningless diffs in version control.

# Goal

The result of this ticket should be the ability to define the contents of `settings.json` in multiple, separately versioned (or unversioned) files, and to assemble them on demand into whatever files Claude Code actually consults. This functionality is functionally similar to what `../../prj/mkenv` does for shell config.

Additionally, the result should enable assemblage of entire `.claude` directories for use at user scope, or bespoke for particular projects. (Claude Code already has a notion of "project" data, such as memories and chat transcripts, stored extrinsically from folders, keyed by filesystem path; though the existing mechanisms are not entirely suitable for `.claude` maintenance.)

For example, a stable assemblage of settings should be definable per filesystem path. Given machines M and N, and O, it should be possible to define settings shared by M and N, as well as settings shared by N and O, without manually maintained copies of any of the settings. Similar flexibility should apply to multiple repos R, S, and T. 

Assemblages should be deterministic, avoiding problem 3 above.
