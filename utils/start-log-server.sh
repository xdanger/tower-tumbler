#!/bin/bash
# Start the browser console log server

echo "Starting browser console log server..."
echo "Make sure you have websockets installed: pip install websockets"
echo "Logs will be saved to ../logs/browser/ directory"
echo ""

python3 log-server.py