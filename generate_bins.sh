#!/bin/bash
# generate_bins.sh
# Run this script from the root of your project (where Cargo.toml is)

CARGO_TOML="Cargo.toml"

# Make a backup first
cp "$CARGO_TOML" "$CARGO_TOML.bak"

# Remove existing [[bin]] entries (optional: keep if you want manual entries intact)
# This clears all old bin definitions before regenerating
awk '!/\[\[bin\]\]/ && !/name =/ && !/path =/' "$CARGO_TOML.bak" > "$CARGO_TOML"

# Re-add top-level package info
echo "" >> "$CARGO_TOML"

# Find all .rs files recursively (excluding Cargo.toml itself)
find . -type f -name "*.rs" | while read -r file; do
    # Strip leading ./ from path
    path="${file#./}"

    # Use filename (without extension) as bin name
    name="$(basename "$file" .rs)"

    echo "[[bin]]" >> "$CARGO_TOML"
    echo "name = \"$name\"" >> "$CARGO_TOML"
    echo "path = \"$path\"" >> "$CARGO_TOML"
    echo "" >> "$CARGO_TOML"
done

echo "✅ Cargo.toml updated with all .rs files as binaries."
