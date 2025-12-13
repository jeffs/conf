#!zsh

# Load nvm lazily, since merely sourcing its definition takes ~350ms.
nvm() {
    # Install NVM if necessary.
    if [ ! -d ~/.nvm ]; then
        ~/conf/src/install-nvm.zsh || return 1
    fi
    unset -f nvm
    export NVM_DIR=~/.nvm
    source $NVM_DIR/nvm.sh
    source $NVM_DIR/bash_completion
    nvm "$@"
}

for command in node npm npx yarn; do
    $command() {
        unset -f node npm npx yarn

        # If NVM hasn't already set our Node version, let it do so.
        if [[ -z "$NVM_DIR" ]]; then
            local dir=$PWD nvmrc=
            while [[ "$dir" != "/" ]]; do
                if [[ -f "$dir/.nvmrc" ]]; then
                    nvmrc="$dir/.nvmrc"
                    break
                fi
                dir=${dir:h}
            done
            if [[ -n "$nvmrc" ]]; then
                nvm use 2>/dev/null || nvm install
            else
                nvm use --lts 2>/dev/null || nvm install --lts
            fi
        fi

        "$0" "$@"
    }
done
unset command
