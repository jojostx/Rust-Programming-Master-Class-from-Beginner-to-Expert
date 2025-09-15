#!/bin/bash
# normalize_names.sh
# Safely normalize file & folder names:
# - spaces & hyphens -> "_"
# - collapse multiple "_"
# - remove leading/trailing "_"
# - lowercase
# - renames dirs deepest-first, then files

set -e

normalize_name() {
    echo "$1" \
        | sed -E 's/[ -]+/_/g' \
        | sed -E 's/_+/_/g' \
        | sed -E 's/^_+|_+$//g' \
        | tr 'A-Z' 'a-z'
}

echo "📂 Normalizing directory names..."
# Sort directories deepest-first by path depth
find . -type d | awk -F/ '{ print NF, $0 }' | sort -nr | cut -d" " -f2- | while read -r dir; do
    # Skip the root "."
    [ "$dir" = "." ] && continue

    new="$(normalize_name "$dir")"
    if [ "$dir" != "$new" ]; then
        echo "Renaming directory: $dir -> $new"
        # Ensure parent exists in case it was renamed earlier
        mkdir -p "$(dirname "$new")"
        mv "$dir" "$new"
    fi
done

echo "📂 Normalizing file names..."
find . -type f | while read -r file; do
    new="$(normalize_name "$file")"
    if [ "$file" != "$new" ]; then
        echo "Renaming file: $file -> $new"
        mv "$file" "$new"
    fi
done

echo "✅ All names normalized (underscores, lowercase, no trailing _)."
