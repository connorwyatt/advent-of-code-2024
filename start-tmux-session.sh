#!/bin/bash

SESSION_NAME="aoc-2024"

if tmux has-session -t $SESSION_NAME 2>/dev/null; then
    echo "Session $SESSION_NAME already exists. Attaching to it."
    tmux attach-session -t $SESSION_NAME
else
    tmux new-session -d -s $SESSION_NAME

    tmux send-keys -t 0 'nvim .' C-m

    tmux attach-session -t $SESSION_NAME
fi

