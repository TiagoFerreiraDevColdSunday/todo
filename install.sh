#!/bin/bash

# Add the alias to ~/.bashrc if it doesn't already exist
if ! grep -q "alias todo=" ~/.bashrc; then
    echo 'alias todo="cargo"' >> ~/.bashrc
    echo "Alias 'todo' added to ~/.bashrc"
else
    echo "Alias 'todo' already exists in ~/.bashrc"
fi

# Reload the shell configuration to apply the alias
source ~/.bashrc