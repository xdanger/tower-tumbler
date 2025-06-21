#!/bin/bash

# Trunk Server Wrapper with Logging
# Starts trunk serve and logs output to logs/server/

# Ensure log directory exists
mkdir -p logs/server

# Get current date for log filename
LOG_DATE=$(date +%Y-%m-%d)
LOG_FILE="logs/server/trunk-${LOG_DATE}.log"

echo "Starting Trunk development server..."
echo "Server logs will be saved to: ${LOG_FILE}"
echo "Press Ctrl+C to stop the server"
echo ""

# Start trunk serve with logging
# Use tee to both display output and save to file
cd .. && trunk serve --release false --no-autoreload 2>&1 | tee "${LOG_FILE}"