#!/bin/env bash

# Get the path to the saved file from the command line
saved_file=$1

# Check if the file exists
if [ ! -e "$saved_file" ]; then
    exit 1
fi

# Loop through the ancestors directories of the saved file until we find a
# Cargo.toml file with a [package] section
dir=$(dirname $saved_file)
while [ "$dir" != "/" ]; do
    if [ -e "$dir/Cargo.toml" ] && grep -q "^\[package\]$" "$dir/Cargo.toml"; then
        break
    fi
    dir=$(dirname $dir)
done

# If we reached the root directory, exit
# This means that we didn't find a Cargo.toml file with a [package] section
if [ "$dir" == "/" ]; then
    exit 1
fi

# Run cargo check on the package, cargo component check if the package is a
# component, meaning it has a [package.metadata.component] section in its
# Cargo.toml file
if grep -q "^\[package.metadata.component\]$" "$dir/Cargo.toml"; then
    cargo component check --package $(basename "$dir") --message-format=json;
else
    cargo check --package $(basename "$dir") --message-format=json;
fi
