#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status
set -e

OUTPUT_FILE="combined_source.md"
TARGET_DIR="src"

# Edge Case Check: Ensure the src directory actually exists
if [ ! -d "$TARGET_DIR" ]; then
    echo "Error: Directory '$TARGET_DIR' not found." >&2
    exit 1
fi

# Clear or initialize the output file
echo "# Project Source Code (Combined)" > "$OUTPUT_FILE"
echo "Generated on: $(date)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "Combining files from '$TARGET_DIR' into '$OUTPUT_FILE'..."

# Find all files in the src directory recursively
# Sorts them alphabetically to keep the output predictable
find "$TARGET_DIR" -type f | sort | while read -r filepath; do
    echo "Processing: $filepath"

    # Append the file path as a header
    echo "## File: \`$filepath\`" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"

    # Open Markdown code block with Rust syntax highlighting
    echo "\`\`\`rust" >> "$OUTPUT_FILE"

    # Append file contents safely
    cat "$filepath" >> "$OUTPUT_FILE"

    # Close code block
    echo "" >> "$OUTPUT_FILE"
    echo "\`\`\`" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
done

echo "Done! You can now upload or copy the contents of '$OUTPUT_FILE'."
