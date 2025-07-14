#!/bin/bash
# Helper script to test CUE4Parse compatibility with Oblivion Remastered

# Configuration
GAME_DIR="C:\Games\Steam\steamapps\common\Oblivion Remastered\OblivionRemastered"
CONTENT_DIR="$GAME_DIR\Content\Paks"

echo "🎮 CUE4Parse Oblivion Remastered Test"
echo "===================================="

# Check if game directory exists
if [ ! -d "$GAME_DIR" ]; then
    echo "❌ Game directory not found: $GAME_DIR"
    echo "Please update the GAME_DIR variable in this script"
    exit 1
fi

echo "📂 Game Directory: $GAME_DIR"
echo "📦 Content Directory: $CONTENT_DIR"

# List available pak files
echo ""
echo "📋 Available Package Files:"
echo "--------------------------"
if [ -d "$CONTENT_DIR" ]; then
    ls -la "$CONTENT_DIR"/*.pak 2>/dev/null || echo "No .pak files found"
    ls -la "$CONTENT_DIR"/*.utoc 2>/dev/null || echo "No .utoc files found"
    ls -la "$CONTENT_DIR"/*.ucas 2>/dev/null || echo "No .ucas files found"
else
    echo "❌ Content directory not found: $CONTENT_DIR"
fi

# Look for .usmap files
echo ""
echo "🗺️  Looking for .usmap mapping files:"
echo "------------------------------------"
find "$GAME_DIR" -name "*.usmap" 2>/dev/null || echo "No .usmap files found in game directory"

# Common .usmap locations to check
USMAP_LOCATIONS=(
    "$GAME_DIR/Mappings.usmap"
    "$GAME_DIR/Content/Mappings.usmap"
    "$GAME_DIR/Binaries/Mappings.usmap"
    "$GAME_DIR/_mappings.usmap"
    "$CONTENT_DIR/Mappings.usmap"
)

FOUND_USMAP=""
for location in "${USMAP_LOCATIONS[@]}"; do
    if [ -f "$location" ]; then
        echo "✅ Found .usmap file: $location"
        FOUND_USMAP="$location"
        break
    fi
done

if [ -z "$FOUND_USMAP" ]; then
    echo "⚠️  No .usmap files found - will test without mappings"
fi

echo ""
echo "🚀 Running CUE4Parse compatibility test..."
echo "========================================="

# Build the command
CMD="cargo run --example real_world_test --features unrealmodding-compat --"
CMD="$CMD --game-dir \"$GAME_DIR\""

if [ -n "$FOUND_USMAP" ]; then
    CMD="$CMD --usmap-path \"$FOUND_USMAP\""
fi

echo "Command: $CMD"
echo ""

# Run the test
eval $CMD

echo ""
echo "✅ Test completed!"
