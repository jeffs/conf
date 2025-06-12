def fc-list [] {
  let file_family_style = ^fc-list | lines | split column ': ' file family-style

  let file = $file_family_style | select file
  let family_style = $file_family_style | get family-style | split column ':style=' family style
  let family = $family_style | default -e '' family | select family | update family {split row ',' | where {is-not-empty}}
  let style = $family_style | default -e '' style | select style | update style {split row ',' | where {is-not-empty}}

  $file | merge $family | merge $style
}
