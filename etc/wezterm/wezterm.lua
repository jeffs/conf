-- TODO
--
-- * Configure click handling
--   - See <https://github.com/wezterm/wezterm/discussions/529>
--   - Nushell `ls` somehow outputs clickable links, which I configure through
--     macOS to open using an Automator script (`on-file-click.app`), which
--     in turn runs a shell script (`on-file-click.nu`), which calls `wezterm
--     cli spawn` to open the file in my chosen editor.

local wezterm = require 'wezterm'
local config = wezterm.config_builder()

config.term = "wezterm"

config.window_decorations = 'RESIZE';

config.default_prog = { wezterm.home_dir .. '/.cargo/bin/nu', '--login' }
config.set_environment_variables = { XDG_CONFIG_HOME = wezterm.home_dir .. '/.config' }

config.unicode_version = 16
config.allow_square_glyphs_to_overflow_width = "Always"

-- config.color_scheme = 'Belge (terminal.sexy)'
config.color_scheme = 'Wombat'
config.colors = {
  background = '#101717',
  -- background = '#0f1419'
}

config.font = wezterm.font 'VictorMono Nerd Font'

config.font_size = 14
config.initial_cols = 160
config.initial_rows = 36

config.send_composed_key_when_left_alt_is_pressed = false
config.send_composed_key_when_right_alt_is_pressed = false

config.window_background_opacity = 0.6
config.text_background_opacity = 0.6
wezterm.on('toggle-opacity', function(window, pane)
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


config.enable_tab_bar = false
-- config.hide_tab_bar_if_only_one_tab = true
wezterm.on('toggle-tab-bar', function(window, pane)
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

return config
