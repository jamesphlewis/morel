#!/bin/bash

# Test script for morel live update feature

echo "Creating initial test file..."
cat > test_live.txt << 'EOF'
Line 1: Initial content
Line 2: This file will be updated
Line 3: Watch for new lines to appear!
Line 4:
Line 5:
Line 6:
Line 7:
Line 8:
Line 9:
Line 10: (More content will appear below)
EOF

echo "Test file created: test_live.txt"
echo ""
echo "Instructions:"
echo "1. In this terminal, run: cargo run -- test_live.txt"
echo "2. In another terminal, run this script again with 'append' argument:"
echo "   ./test_live_update.sh append"
echo ""
echo "Or run both automatically:"
echo "   ./test_live_update.sh auto"
echo ""

if [ "$1" == "append" ]; then
    echo "Waiting 2 seconds before appending..."
    sleep 2
    echo "Appending line 11..."
    echo "Line 11: NEW CONTENT ADDED!" >> test_live.txt
    sleep 2
    echo "Appending line 12..."
    echo "Line 12: Another update!" >> test_live.txt
    sleep 2
    echo "Appending line 13..."
    echo "Line 13: And another!" >> test_live.txt
    sleep 2
    echo "Appending line 14..."
    echo "Line 14: Live updates working!" >> test_live.txt
    echo "Done! The content should update automatically in morel."
elif [ "$1" == "auto" ]; then
    echo "Starting morel and auto-updating..."
    # Run appender in background
    (
        sleep 3
        echo "Line 11: NEW CONTENT ADDED!" >> test_live.txt
        sleep 2
        echo "Line 12: Another update!" >> test_live.txt
        sleep 2
        echo "Line 13: And another!" >> test_live.txt
        sleep 2
        echo "Line 14: Live updates working!" >> test_live.txt
    ) &

    # Run morel
    cargo run -- test_live.txt
fi
