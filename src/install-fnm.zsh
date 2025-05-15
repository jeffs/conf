#!/usr/bin/env nu

# Download and install fnm:
curl -o- https://fnm.vercel.app/install | bash

# Download and install Node.js:
fnm install 24

# Verify the Node.js version:
node -v # Should print "v24.0.2".

# Verify npm version:
npm -v # Should print "11.3.0".
