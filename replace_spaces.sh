#!/usr/bin/env bash
# safe_normalize.sh
# Safe normalization of filenames and directories:
#  - spaces & hyphens -> underscore
#  - collapse multiple underscores
#  - strip leading/trailing underscores
#  - lowercase
#  - Excludes .git and everything inside it
#  - Excludes files named Cargo.lock (anywhere)
#  - Dry-run by default; use --apply to actually rename
#  - Optionally use --git (with --apply) to run git mv (keeps history)

set -euo pipefail

DRY_RUN=1
USE_GIT_MV=0

# parse flags
while [[ $# -gt 0 ]]; do
    case "$1" in
        --apply) DRY_RUN=0; shift ;;
        --git) USE_GIT_MV=1; shift ;;
        --help) printf "Usage: %s [--apply] [--git]\n\n--apply  : actually perform renames (default: dry-run)\n--git    : when used with --apply and inside a git repo, use git mv\n" "$0"; exit 0 ;;
        *) printf "Unknown option: %s\n" "$1"; exit 2 ;;
    esac
done

# Basenames to exclude (exact match). Add more names if needed.
EXCLUDE_NAMES=( "Cargo.lock" )

normalize_component() {
    local s="$1"
    # replace whitespace and hyphen with underscore, collapse underscores, strip edges, lowercase
    s="$(printf '%s' "$s" | sed -E 's/[[:space:]-]+/_/g')"
    s="$(printf '%s' "$s" | sed -E 's/_+/_/g')"
    s="$(printf '%s' "$s" | sed -E 's/^_+|_+$//g')"
    printf '%s' "$s" | tr 'A-Z' 'a-z'
}

is_excluded_basename() {
    local b="$1"
    for ex in "${EXCLUDE_NAMES[@]}"; do
        if [[ "$b" == "$ex" ]]; then
            return 0
        fi
    done
    return 1
}

echo "🔎 Mode: $( [ $DRY_RUN -eq 1 ] && printf 'DRY-RUN (no changes)' || printf 'APPLY (will rename)')"
if [ "$USE_GIT_MV" -eq 1 ]; then
    echo "🔁 Will attempt to use git mv when applying (requires --apply and a git repo)."
fi
echo "Excluded basenames: ${EXCLUDE_NAMES[*]}"
echo "Excluded path: ./.git (and everything under it)"
echo

# Build a find that prunes .git and Cargo.lock file(s) early.
# Note: -prune on ./.git avoids descending into it. -name 'Cargo.lock' -prune avoids that file.
# The -print0 emits all remaining paths (files and dirs) in depth-first order (because of -depth).
# We still double-check inside the loop as extra-safety.
find . \( -path './.git' -o -path './.git/*' -o -name 'Cargo.lock' \) -prune -o -depth -print0 | \
while IFS= read -r -d '' path; do
    # skip root
    [ "$path" = "." ] && continue

    # Extra-safety: skip anything starting with ./.git (if find pruning failed for some reason)
    if [[ "$path" == ./.git* ]]; then
        # (shouldn't happen because of -prune, but double-check)
        echo "  [SKIP .git] $path"
        continue
    fi

    dir=$(dirname "$path")
    base=$(basename "$path")

    # skip excluded basenames (e.g. Cargo.lock)
    if is_excluded_basename "$base"; then
        echo "  [SKIP EXCLUDED] $path"
        continue
    fi

    new_base="$(normalize_component "$base")"

    # If nothing changes, continue
    if [ "$base" = "$new_base" ]; then
        continue
    fi

    new_path="$dir/$new_base"

    # ensure parent for destination exists (should be true, but defensive)
    mkdir -p -- "$(dirname "$new_path")"

    # If destination exists, create a unique name (_1, _2, ...)
    if [ -e "$new_path" ]; then
        name_noext="$new_base"
        ext=""
        if [[ "$new_base" == *.* && -f "$path" ]]; then
            name_noext="${new_base%.*}"
            ext=".${new_base##*.}"
        fi
        i=1
        while [ -e "$dir/${name_noext}_$i${ext}" ]; do
            i=$((i + 1))
        done
        new_path="$dir/${name_noext}_$i${ext}"
    fi

    if [ $DRY_RUN -eq 1 ]; then
        printf "[DRY] %s -> %s\n" "$path" "$new_path"
    else
        # apply rename: prefer git mv if requested and available and repo exists
        if [ $USE_GIT_MV -eq 1 ] && command -v git >/dev/null 2>&1 && [ -d .git ]; then
            printf "git mv: %s -> %s\n" "$path" "$new_path"
            # try git mv; if it fails, fall back to plain mv
            if ! git mv -k -- "$path" "$new_path" 2>/dev/null; then
                echo "  ⚠ git mv failed for $path, falling back to mv"
                mv -- "$path" "$new_path"
            fi
        else
            printf "mv: %s -> %s\n" "$path" "$new_path"
            mv -- "$path" "$new_path"
        fi
    fi
done

echo
echo "✅ Completed. (Dry-run: $DRY_RUN)"
if [ $DRY_RUN -eq 1 ]; then
    echo "Run with --apply to perform the renames (use --git to prefer git mv where possible)."
fi
