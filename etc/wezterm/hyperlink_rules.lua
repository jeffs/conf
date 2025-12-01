-- <https://github.com/wezterm/wezterm/issues/3803#issuecomment-2379791340>
return {
  -- Matches: a URL in parens: (URL)
  -- Markdown: [text](URL title)
  {
    regex = '\\((\\w+://\\S+?)(?:\\s+.+)?\\)',
    format = '$1',
    highlight = 1,
  },
  -- Matches: a URL in brackets: [URL]
  {
    regex = '\\[(\\w+://\\S+?)\\]',
    format = '$1',
    highlight = 1,
  },
  -- Matches: a URL in curly braces: {URL}
  {
    regex = '\\{(\\w+://\\S+?)\\}',
    format = '$1',
    highlight = 1,
  },
  -- Matches: a URL in angle brackets: <URL>
  {
    regex = '<(\\w+://\\S+?)>',
    format = '$1',
    highlight = 1,
  },
  -- Then handle URLs not wrapped in brackets
  -- regex = '\\b\\w+://\\S+[)/a-zA-Z0-9-]+',
  {
    regex = '(?<![\\(\\{\\[<])\\b\\w+://\\S+',
    format = '$0',
  },
  -- implicit mailto link
  {
    regex = '\\b\\w+@[\\w-]+(\\.[\\w-]+)+\\b',
    format = 'mailto:$0',
  },
}
