#!/bin/bash
# ci.sh
# Hubstry CaaS Generic CI/CD Integration Script

set -e

THRESHOLD="${1:-90.0}"
TARGET_DIR="${2:-src}"

echo "🚀 Starting Hubstry CaaS Compliance Check..."
echo "📊 Minimum required score: $THRESHOLD%"
echo "📁 Target directory: $TARGET_DIR"

if ! command -v hubstry_iso_code &> /dev/null; then
    echo "⚙️  Installing hubstry_iso_code from source..."
    cargo install --path . --force
fi

echo "🔎 Analyzing Rust files..."
FILES=$(find "$TARGET_DIR" -type f -name "*.rs")

FAILURES=0

for file in $FILES; do
    echo "------------------------------------------------"
    echo "Analyzing: $file"
    # Ensure tool exists or returns error correctly
    if ! hubstry_iso_code --file "$file" --threshold "$THRESHOLD"; then
        echo "❌ Compliance check failed for $file"
        FAILURES=$((FAILURES + 1))
    else
        echo "✅ Compliance check passed for $file"
    fi
done

echo "------------------------------------------------"

if [ $FAILURES -gt 0 ]; then
    echo "🛑 CI/CD Pipeline Failed! Found compliance violations in $FAILURES file(s)."
    echo "📄 Check the generated compliance_report.html or compliance_report.json for details."
    exit 1
else
    echo "🎉 Success! All analyzed files meet the required compliance threshold."
    exit 0
fi
