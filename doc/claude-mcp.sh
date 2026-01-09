claude mcp add repomix -- npx -y repomix --mcp
claude mcp add --transport http linear-server https://mcp.linear.app/mcp
claude mcp add playwright npx '@playwright/mcp@latest'

# Notion

# Add to current project only (local scope, default)
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Add for all your projects (user scope)
claude mcp add --transport http -s user notion https://mcp.notion.com/mcp

# Add to project config checked into repo (project scope)
claude mcp add --transport http -s project notion https://mcp.notion.com/mcp
