#!/bin/bash
# start-bridge.sh — Start the ollama-bridge and wait for it
# Usage: ./start-bridge.sh [port]

PORT=${1:-11434}
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Kill any existing bridge
kill $(lsof -ti:$PORT) 2>/dev/null

# Start the bridge
node "$SCRIPT_DIR/ollama-bridge.mjs" --port $PORT &
BRIDGE_PID=$!
disown $BRIDGE_PID

# Wait for it to be ready
for i in $(seq 1 10); do
    if curl -s http://127.0.0.1:$PORT/ > /dev/null 2>&1; then
        echo "Bridge ready on port $PORT (PID: $BRIDGE_PID)"
        exit 0
    fi
    sleep 0.5
done

echo "Bridge failed to start"
exit 1