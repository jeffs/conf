local wezterm = require 'wezterm'
local config = wezterm.config_builder()

-- config.color_scheme = 'Bamboo Multiplex'
-- config.color_scheme = 'One Dark Pro'
-- config.color_scheme = 'Abernathy'
-- config.color_scheme = 'Aco (Gogh)'
-- config.color_scheme = 'zenwritten_dark'

config.font = wezterm.font 'VictorMono Nerd Font'
-- config.font = wezterm.font 'JetBrains Mono'
-- config.font = wezterm.font 'Hasklig'
-- config.font = wezterm.font 'Fira Code'
-- config.font = wezterm.font 'Hack Nerd Font Mono'
-- config.font = wezterm.font 'FiraCode Nerd Font Mono'
-- config.font = wezterm.font 'Hasklug Nerd Font'
-- config.font = wezterm.font 'Lekton Nerd Font Mono'
-- config.font = wezterm.font 'IntoneMono Nerd Font Mono'

config.font_size = 14
config.initial_cols = 104
config.initial_rows = 40
config.hide_tab_bar_if_only_one_tab = true

-- TODO: https://wezterm.org/config/lua/window/set_config_overrides.html
config.window_background_opacity = 0.8
-- config.text_background_opacity = 0.7

config.send_composed_key_when_left_alt_is_pressed = true
config.send_composed_key_when_right_alt_is_pressed = false

config.leader = { mods = 'CTRL', key = 'g', timeout_milliseconds = 1000 }
config.keys = {
  -- Split
  {
    mods = 'LEADER',
    key = '%',
    action = wezterm.action.SplitHorizontal { domain = 'CurrentPaneDomain' }
  },
  {
    mods = 'LEADER',
    key = '"',
    action = wezterm.action.SplitVertical { domain = 'CurrentPaneDomain' }
  },

  -- Zoom
  {
    mods = 'LEADER',
    key = 'z',
    action = wezterm.action.TogglePaneZoomState
  },

  -- Navigate
  {
    mods = 'LEADER',
    key = 'h',
    action = wezterm.action.ActivatePaneDirection('Left')
  },
  {
    mods = 'LEADER',
    key = 'j',
    action = wezterm.action.ActivatePaneDirection('Down')
  },
  {
    mods = 'LEADER',
    key = 'k',
    action = wezterm.action.ActivatePaneDirection('Up')
  },
  {
    mods = 'LEADER',
    key = 'l',
    action = wezterm.action.ActivatePaneDirection('Right')
  },
  {
    mods = 'LEADER|CTRL',
    key = 'g',
    action = wezterm.action.ActivatePaneDirection("Next")
  },
  {
    mods = 'LEADER|CTRL|SHIFT',
    key = 'g',
    action = wezterm.action.ActivatePaneDirection("Prev")
  }
}

return config
