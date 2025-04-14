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

config.window_background_opacity = 0.8

config.send_composed_key_when_left_alt_is_pressed = true
config.send_composed_key_when_right_alt_is_pressed = false

wezterm.on('toggle-opacity', function(window, pane)
  local overrides = window:get_config_overrides() or {}
  if not overrides.window_background_opacity then
    overrides.window_background_opacity = 1.0
  else
    overrides.window_background_opacity = nil
  end
  window:set_config_overrides(overrides)
end)

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
    mods = 'LEADER|CTRL',
    key = 'G',
    action = wezterm.action.ActivatePaneDirection("Prev")
  },

  -- Alter appearance
  {
    mods = 'CMD',
    key = 'u',
    action = wezterm.action.EmitEvent 'toggle-opacity',
  },
}

return config
