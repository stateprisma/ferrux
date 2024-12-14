#!/bin/sh

HOOK_DIR=".git/hooks"
COMMIT_MSG_HOOK="$HOOK_DIR/commit-msg"
COMMIT_MSG_HOOK_SCRIPT=$(cat <<'EOF'
COMMITLINT_OUTPUT=$(cat "$1" | commitlint 2>&1)
COMMITLINT_EXIT_CODE=$?

if [ $COMMITLINT_EXIT_CODE -ne 0 ]; then
    echo "Commitlint Error ($COMMITLINT_EXIT_CODE): [$COMMITLINT_OUTPUT] $(cat $1)"
    exit $COMMITLINT_EXIT_CODE
fi
EOF
)

# Check if .git directory exists
if [ ! -d ".git" ]; then
    echo "Error: This is not a Git repository."
    exit 1
fi

# Ensure the hooks directory exists
if [ ! -d "$HOOK_DIR" ]; then
    echo "Error: Git hooks directory does not exist."
    exit 2
fi

# Download commitlint-rs (Rust version)
cargo install commitlint

# Set location of git hooks
git config --local core.hooksPath .git/hooks

# Set up commit-msg git hook
if [ ! -f "$COMMIT_MSG_HOOK" ]; then
    echo "#!/bin/sh" > "$COMMIT_MSG_HOOK"
    echo "$COMMIT_MSG_HOOK_SCRIPT" >> "$COMMIT_MSG_HOOK"
    chmod +x "$COMMIT_MSG_HOOK"
    echo "commit-msg hook created."
else
    if grep -q "$COMMIT_MSG_HOOK_SCRIPT" "$COMMIT_MSG_HOOK"; then
        echo "Script already injected into commit-msg hook."
    else
        echo -e "$COMMIT_MSG_HOOK_SCRIPT" >> "$COMMIT_MSG_HOOK"
        echo "Script injected into commit-msg hook."
    fi
fi