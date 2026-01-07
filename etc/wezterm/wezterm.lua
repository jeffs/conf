-- TODO
--
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

-- Use Nu from Cargo if available, else Nu from Homebrew, else default shell.
config.default_prog = {
  if_readable(wezterm.home_dir .. '/.cargo/bin/nu') or
    if_readable('/opt/homebrew/bin/nu'),
  '--login'
}

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

config.window_background_opacity = 0.4
-- config.text_background_opacity = 1.0
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

-- TODO: I keep going back and forth on whether to show the tab bar when
--  there's only one tab. I pretty much want this when I'm working with multiple
--  terminal windows, or when I keep opening and closing a second tab. These
--  have become more common situations since I've been relying more heavily
--  on terminal-based AI agents. Maybe I should update my existing binding to
--  toggle the tab bar (CMD+i) to instead cycle through three states: Hidden,
--  Hidden if only one tab, Shown. See `toggle-opacity` above for an example of
--  a custom event that triggers a callback function (and its binding below).
--
-- config.hide_tab_bar_if_only_one_tab = true

config.enable_tab_bar = true
wezterm.on('toggle-tab-bar', function(window)
  local overrides = window:get_config_overrides() or {}
  overrides.enable_tab_bar =  not overrides.enable_tab_bar
  window:set_config_overrides(overrides)
end)

config.leader = { mods = 'CTRL', key = 'h', timeout_milliseconds = 1000 }
config.keys = {
  -- Split
  {
    mods = 'LEADER', key = 'S',
    action = wezterm.action.SplitHorizontal { domain = 'CurrentPaneDomain' }
  },
  {
    mods = 'LEADER', key = 's',
    action = wezterm.action.SplitVertical { domain = 'CurrentPaneDomain' }
  },

  -- Zoom
  {
    mods = 'LEADER', key = 'z',
    action = wezterm.action.TogglePaneZoomState
  },

  -- Swap
  {
    mods = 'LEADER|CTRL', key = 'h',
    action = wezterm.action.RotatePanes 'Clockwise'
  },
  {
    mods = 'LEADER', key = 'W',
    action = wezterm.action.PaneSelect { mode = 'SwapWithActive' }
  },

  -- Navigate
  {
    mods = 'LEADER', key = 'Escape',
    action = wezterm.action.ActivateCopyMode
  },
  {
    mods = 'LEADER', key = 'h',
    action = wezterm.action.ActivatePaneDirection('Left')
  },
  {
    mods = 'LEADER', key = 'j',
    action = wezterm.action.ActivatePaneDirection('Down')
  },
  {
    mods = 'LEADER', key = 'k',
    action = wezterm.action.ActivatePaneDirection('Up')
  },
  {
    mods = 'LEADER', key = 'l',
    action = wezterm.action.ActivatePaneDirection('Right')
  },
  {
    mods = 'LEADER', key = 'n',
    action = wezterm.action.ActivatePaneDirection("Next")
  },
  {
    mods = 'LEADER', key = 'N',
    action = wezterm.action.ActivatePaneDirection("Prev")
  },
  {
    mods = 'LEADER', key = 't',
    action = wezterm.action.ShowTabNavigator
  },
  {
    mods = 'LEADER', key = 'w',
    action = wezterm.action.PaneSelect
  },

  -- Alter appearance. These don't use the Leader key, which I reserve for the
  -- experimental mux.
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

return config
