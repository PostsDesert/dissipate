#!/bin/bash

# SQLite binary download and setup script
# Downloads the latest SQLite CLI binary for the current platform

# Update these variables to get the latest SQLite version
SQLITE_YEAR="2025"
SQLITE_VERSION="3510100"

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
DATABASE_DIR="$(cd "$PROJECT_DIR/../database" && pwd)"

echo -e "${GREEN}SQLite Binary Setup Script${NC}"
echo "=============================="
echo ""

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "Detected OS: $OS"
echo "Detected Architecture: $ARCH"
echo ""

# Determine download URL based on OS and architecture
DOWNLOAD_URL=""

if [[ "$OS" == "Darwin" ]]; then
    # macOS
    if [[ "$ARCH" == "arm64" ]]; then
        # Apple Silicon
        DOWNLOAD_URL="https://sqlite.org/$SQLITE_YEAR/sqlite-tools-osx-arm64-$SQLITE_VERSION.zip"
    else
        # Intel Mac
        DOWNLOAD_URL="https://sqlite.org/$SQLITE_YEAR/sqlite-tools-osx-x86-$SQLITE_VERSION.zip"
    fi
elif [[ "$OS" == "Linux" ]]; then
    # Linux
    if [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
        DOWNLOAD_URL="https://sqlite.org/$SQLITE_YEAR/sqlite-tools-linux-aarch64-$SQLITE_VERSION.zip"
    else
        DOWNLOAD_URL="https://sqlite.org/$SQLITE_YEAR/sqlite-tools-linux-x86-$SQLITE_VERSION.zip"
    fi
else
    echo -e "${RED}Error: Unsupported operating system: $OS${NC}"
    echo "This script supports macOS and Linux only."
    exit 1
fi

echo -e "${YELLOW}SQLite Version:${NC} $SQLITE_VERSION"
echo -e "${YELLOW}Download URL:${NC} $DOWNLOAD_URL"
echo ""

# Create temporary directory for download
TEMP_DIR=$(mktemp -d)
echo "Creating temporary directory: $TEMP_DIR"

# Download SQLite
echo -e "${GREEN}Downloading SQLite binary...${NC}"
if command -v curl &> /dev/null; then
    curl -L -o "$TEMP_DIR/sqlite.zip" "$DOWNLOAD_URL"
elif command -v wget &> /dev/null; then
    wget -O "$TEMP_DIR/sqlite.zip" "$DOWNLOAD_URL"
else
    echo -e "${RED}Error: Neither curl nor wget is installed.${NC}"
    echo "Please install curl or wget and try again."
    exit 1
fi

# Extract the archive
echo -e "${GREEN}Extracting archive...${NC}"
unzip -q "$TEMP_DIR/sqlite.zip" -d "$TEMP_DIR"

# Find sqlite3 binary - check if extracted directly or in subdirectory
if [[ -f "$TEMP_DIR/sqlite3" ]]; then
    EXTRACTED_DIR="$TEMP_DIR"
else
    EXTRACTED_DIR=$(find "$TEMP_DIR" -type d -name "sqlite-tools-*" | head -1)
fi

if [[ -z "$EXTRACTED_DIR" ]] || [[ ! -f "$EXTRACTED_DIR/sqlite3" ]]; then
    echo -e "${RED}Error: Could not find sqlite3 binary in extracted files.${NC}"
    echo "Contents of temp directory:"
    ls -la "$TEMP_DIR"
    exit 1
fi

# Copy sqlite3 binary to database directory
echo -e "${GREEN}Installing sqlite3 to $DATABASE_DIR${NC}"
cp "$EXTRACTED_DIR/sqlite3" "$DATABASE_DIR/"

# Make it executable
chmod +x "$DATABASE_DIR/sqlite3"

# Create symlink in project root for convenience
ln -sf "$DATABASE_DIR/sqlite3" "$PROJECT_DIR/sqlite3"

# Clean up
echo -e "${GREEN}Cleaning up temporary files...${NC}"
rm -rf "$TEMP_DIR"

# Verify installation
echo ""
echo -e "${GREEN}=================================${NC}"
echo -e "${GREEN}SQLite installation complete!${NC}"
echo -e "${GREEN}=================================${NC}"
echo ""
echo "SQLite binary installed at: $DATABASE_DIR/sqlite3"
echo "Symlink created at: $PROJECT_DIR/sqlite3"
echo ""

# Display version
if "$DATABASE_DIR/sqlite3" --version; then
    echo ""
    echo -e "${GREEN}✓ SQLite is ready to use!${NC}"
    echo ""
    echo "Usage:"
    echo "  cd backend"
    echo "  ../database/sqlite3 dissipate.db"
    echo "  # or use the symlink:"
    echo "  sqlite3 dissipate.db"
else
    echo -e "${RED}Warning: SQLite binary installation failed verification.${NC}"
    exit 1
fi
