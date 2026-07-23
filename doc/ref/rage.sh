# Install
cargo install --locked rage

# Encrypt
env PINENTRY_PROGRAM= rage --passphrase -o file.age file

# Decrypt
env PINENTRY_PROGRAM= rage -d file.age | tar zx
