#!/bin/bash

# Release Build Script with Logging
# Builds the project for production and logs output

# Ensure log directory exists
mkdir -p logs/server

# Get current date and time for log filename
LOG_TIMESTAMP=$(date +%Y-%m-%d_%H-%M-%S)
LOG_FILE="logs/server/build-${LOG_TIMESTAMP}.log"

echo "Starting release build..."
echo "Build logs will be saved to: ${LOG_FILE}"
echo ""

# Run trunk build with logging (using cargo release profile but skipping wasm-opt)
cd .. && trunk build --cargo-profile release 2>&1 | tee "${LOG_FILE}"

# Check build result
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo ""
    echo "✅ Build completed successfully!"
    echo "Output files are in: dist/"
    echo "Build log saved to: ${LOG_FILE}"
else
    echo ""
    echo "❌ Build failed!"
    echo "Check the build log for details: ${LOG_FILE}"
    exit 1
fi