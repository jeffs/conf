-- TODO
--
-- * `#include` an unversioned file of overrides
--   - This comes up a lot for opacity tweaking
-- * Configure click handling
--   - See <https://github.com/wezterm/wezterm/discussions/529>
--   - Nushell `ls` somehow outputs clickable links, which I configure through
--     macOS to open using an Automator script (`on-file-click.app`), which
--     in turn runs a shell script (`on-file-click.nu`), which calls `wezterm
--     cli spawn` to open the file in my chosen editor.
-- * In the list shown by `wezterm.action.ShowTabNavigator`, focus the current
--   tab by default.
-- * Fix set-window-title; see <https://github.com/wezterm/wezterm/pull/6913>

local wezterm = require 'wezterm'
local config = wezterm.config_builder()

config.term = "wezterm"

-- Requires nightly build of Wezterm.
-- config.window_decorations = 'TITLE | RESIZE | MACOS_USE_BACKGROUND_COLOR_AS_TITLEBAR_COLOR';
config.window_decorations = 'RESIZE';

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
local zellij = if_readable(wezterm.home_dir .. '/.cargo/bin/zellij')
  or if_readable('/opt/homebrew/bin/zellij')
config.default_prog = { zellij }

config.set_environment_variables = {
  XDG_CONFIG_HOME = wezterm.home_dir .. '/.config'
}

config.unicode_version = 16
config.allow_square_glyphs_to_overflow_width = "Always"

config.color_scheme = 'Belge (terminal.sexy)'

-- Some applications, such as `md-tui`, require nerd fonts.
config.font = wezterm.font 'VictorMono Nerd Font'
-- config.font = wezterm.font 'Monoid'

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

-- <https://wezterm.org/config/lua/config/audible_bell.html>
--
-- I can't seem to get the visual bell to work, at least in Helix:
-- <https://wezterm.org/config/lua/config/visual_bell.html>
-- config.audible_bell = 'Disabled'

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

  -- Nushell accepts Alt+Enter to enter multiline commands, so I use Cmd+Enter
  -- instead (matching the iTerm2 default).
  {
    mods = 'ALT', key = 'Enter',
    action = wezterm.action.DisableDefaultAssignment,
  },
  {
    mods = 'CMD', key = 'Enter',
    action = wezterm.action.ToggleFullScreen,
  }
}

config.hyperlink_rules = require 'hyperlink_rules'

-- Load any local (unversioned) overrides. Sample override:
-- 
--  return {
--    window_background_opacity = 0.9,
--  }
-- 
-- One problem with using a colocated `local.lua` is that `jj rebase` deletes
-- it. This is obviously a bug in jj, but nobody seems to have filed it yet,
-- and I can't be bothered right now. Or maybe the jj maintainers don't think
-- it's a bug, which would be on-brand for Google. In the meantime, I keep
-- a copy at ~/.config/wezterm.local.lua, and cp it whenever jj blows it
-- away.
--
-- If you're dealing with this, and also cannot be bothered to get jj
-- properly fixed, consider vibe-coding something that automates the cp if
-- ~/.config/wezterm.local.lua (or whatever) exists but local.lua does not.
-- Note that a symlink won't work, because WezTerm won't automatically pick up
-- changes to the linked file.
local local_config = wezterm.config_dir .. '/local.lua'
if if_readable(local_config) then
  local ok, overrides = pcall(dofile, local_config)
  if ok and type(overrides) == 'table' then
    for k, v in pairs(overrides) do
      config[k] = v
    end
  end
end

return config
