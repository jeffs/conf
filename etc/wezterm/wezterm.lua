-- TODO
--
-- Configure click handling. Nushell `ls` outputs clickable links using
-- OSC-8 escape sequences, and I configure macOS to open those "links" using
-- an Automator script (`on-file-click.app`), which in turn runs a shell
-- script (`on-file-click.nu`), which calls `wezterm cli spawn` to open the
-- file in my chosen editor. But, macOS file associations are fickle: The OS
-- seems to forget them on upgrades, and IDEs like to steal them. See also
-- <https://github.com/wezterm/wezterm/discussions/529>.
-- 
-- In the list shown by `wezterm.action.ShowTabNavigator`, focus the current tab
-- by default.
-- 
-- Fix set-window-title; see <https://github.com/wezterm/wezterm/pull/6913>.
-- Workaround: Zellij preserves tab names; apps override pane names.

local wezterm = require 'wezterm'
local config = wezterm.config_builder()

config.term = "wezterm"

-- Nightly builds of Wezterm would support:
-- config.window_decorations = 'TITLE | RESIZE | MACOS_USE_BACKGROUND_COLOR_AS_TITLEBAR_COLOR'
config.window_decorations = 'RESIZE'

-- Returns the specified path if it identifies a readable file, and nil
-- otherwise.
local function if_readable(path)
  local file = io.open(path, 'r')
  if file == nil then
    return nil
  end
  io.close(file)
  return path
end

-- Launch Zellij, which will start Nushell (configured in Zellij's config).
-- Each WezTerm tab/window gets its own Zellij session.
--
-- Zellij looks for config in `~/.config/zellij`, which is symlinked to
-- `~/conf/etc/zellij` by `~/conf/src/install-dotfiles.zsh`.
config.default_prog = {
  if_readable(wezterm.home_dir .. '/.cargo/bin/zellij')
    or if_readable('/opt/homebrew/bin/zellij')
 }

config.unicode_version = 16
config.allow_square_glyphs_to_overflow_width = "Always"

config.color_scheme = 'Belge (terminal.sexy)'

-- Some applications, such as `md-tui`, require nerd fonts.
config.font = wezterm.font 'VictorMono Nerd Font'
-- config.font = wezterm.font 'Maple Mono NF CN'

config.font_size = 16
config.initial_cols = 160
config.initial_rows = 36

config.send_composed_key_when_left_alt_is_pressed = false
config.send_composed_key_when_right_alt_is_pressed = false

-- config.mouse_bindings = {
--   -- Slower scroll up/down (3 lines instead of Page Up/Down)
--   { event = { Down = { streak = 1, button = { WheelUp = 1 } } }, mods = 'NONE', action = wezterm.action.Nop, },
--   { event = { Down = { streak = 1, button = { WheelDown = 1 } } }, mods = 'NONE', action = wezterm.action.Nop, },
-- }

config.window_background_opacity = 0.6
config.text_background_opacity = 1.0
wezterm.on('toggle-opacity', function(window)
  local overrides = window:get_config_overrides() or {}
  if not overrides.window_background_opacity then
    overrides.window_background_opacity = 1.0
    overrides.text_background_opacity = 1.0
  else
    overrides.window_background_opacity = nil
    overrides.text_background_opacity = nil
  end
  window:set_config_overrides(overrides)
end)

-- I can't seem to get the visual bell to work, at least in Helix. See also:
-- 
-- - <https://wezterm.org/config/lua/config/audible_bell.html>
-- - <https://wezterm.org/config/lua/config/visual_bell.html>
config.audible_bell = 'Disabled'

-- Zellij handles tabs/panes; WezTerm's tab bar is just for multiple WezTerm
-- windows. Can toggle with CMD+i.
-- config.hide_tab_bar_if_only_one_tab = true
config.enable_tab_bar = false
wezterm.on('toggle-tab-bar', function(window)
  local overrides = window:get_config_overrides() or {}
  overrides.enable_tab_bar =  not overrides.enable_tab_bar
  window:set_config_overrides(overrides)
end)

-- Multiplexing is handled by Zellij; WezTerm just provides the terminal.
config.keys = {
  -- Alter appearance
  {
    mods = 'CMD', key = 'i',
    action = wezterm.action.EmitEvent 'toggle-tab-bar',
  },
  {
    mods = 'CMD', key = 'u',
    action = wezterm.action.EmitEvent 'toggle-opacity',
  },
  {
    mods = 'CTRL', key = '-',
    action = wezterm.action.DisableDefaultAssignment,
  },
  {
    mods = 'CTRL', key = '=',
    action = wezterm.action.DisableDefaultAssignment,
  },
  {
    mods = 'CTRL', key = 'T',
    action = wezterm.action.DisableDefaultAssignment,
  },
  {
    -- Send Alt+t for Zellij's NewTab binding
    mods = 'CMD', key = 't',
    action = wezterm.action.SendKey { mods = 'ALT', key = 't' },
  },

  -- Nushell accepts Alt+Enter to enter multiline commands, so I use Cmd+Enter
  -- instead (matching the iTerm2 default).
  {
    mods = 'ALT', key = 'Enter',
    action = wezterm.action.DisableDefaultAssignment,
  },
  {
    mods = 'CMD', key = 'Enter',
    action = wezterm.action.ToggleFullScreen,
  },

  -- Claude Code expects this sequence for Shift+Enter (newline insertion).
  {
    mods = 'SHIFT', key = 'Enter',
    action = wezterm.action.SendString("\x1b\r"),
  },
}

config.hyperlink_rules = require 'hyperlink_rules'

-- Load any local (unversioned) overrides. Sample override:
-- 
--  return {
--    window_background_opacity = 0.9,
--  }
-- 
-- One problem with using a colocated `local.lua` is that `jj rebase` deletes
-- it. This is obviously a bug in jj, but nobody seems to have filed it yet, and
-- I can't be bothered right now. Or maybe the jj maintainers don't think it's
-- a bug, which would be on-brand for Google. In the meantime, I keep a copy at
-- ~/var/bak, and cp it whenever jj blows it away.
--
-- TODO: Automate the cp if the backup exists but local.lua does not. A symlink
--  won't work, because WezTerm doesn't watch the linked file.
local local_config = wezterm.home_dir .. '/conf/etc/wezterm/local.lua'
if if_readable(local_config) then
  local ok, overrides = pcall(dofile, local_config)
  if ok and type(overrides) == 'table' then
    for k, v in pairs(overrides) do
      config[k] = v
    end
  end
end

return config
