---
status: TODO
title: Try chezmoi
---

Consider replacing platform/machine-specific functionality with chezmoi. Claude's assessment:

> Most of the portability machinery here maps cleanly onto chezmoi:
> 
> - **`etc/platform/macos.toml` (per-OS) → chezmoi templates + `.chezmoidata`.** chezmoi exposes `.chezmoi.os`, `.chezmoi.arch`, `.chezmoi.osRelease`, hostname, etc., and lets you stash structured data in `.chezmoidata/*.toml`. A `linux.toml` + `macos.toml` pair under `.chezmoidata/` plus `{{ if eq .chezmoi.os "darwin" }}` gates is the idiomatic version of what `platform_toml_name()` does.
> - **`var/site.toml` (per-host overrides) → `~/.config/chezmoi/chezmoi.toml`.** That's literally chezmoi's per-machine config file, and it merges into the template data the same way your `deep_merge` does.
> - **`prj/login` generating `env.sh` + `env.json` for POSIX vs Nu/Xonsh → two `.tmpl` files over shared data.** This is the textbook chezmoi use case: one source of truth, multiple rendered shells. You'd lose the typed Rust schema but gain the diff/apply/dry-run lifecycle.
> - **Editor unification (helix/nvim/vscode/ideavim) → same templating story.** Probably the strongest reason to adopt it, since you don't have an analogue of `prj/login` for editors yet and you'd otherwise be building one.

