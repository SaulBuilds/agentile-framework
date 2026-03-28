#!/bin/bash

# Script to test TLA+ specifications for Orchard Game
# Note: This requires the TLA+ Toolkit to be installed

echo "Testing Orchard Game TLA+ Specifications"
echo "======================================"

# Check if java is available
if ! command -v java &> /dev/null
then
    echo "Java could not be found. Please install Java 8 or later."
    exit 1
fi

# Check if tla2tools.jar is available (would need to be downloaded separately)
if [ ! -f "./tla2tools.jar" ]; then
    echo "TLA+ Toolkit not found. Please download it from:"
    echo "https://lamport.azurewebsites.net/tla/tla.html"
    echo "Extract it and place tla2tools.jar in this directory."
    echo ""
    echo "For now, we'll just verify the syntax by checking the files exist:"
fi

echo "Checking specification files:"

SPECS=(
    "SeedLifecycle.tla"
    "FederationEcon.tla" 
    "BelnapAggregation.tla"
    "SchoolSafety.tla"
)

all_good=true
for spec in "${SPECS[@]}"; do
    if [ -f "$spec" ]; then
        echo "✓ $spec exists"
        # Basic syntax check - look for key TLA+ elements
        if grep -q "EXTENDS Naturals, TLC" "$spec" && grep -q "===" "$spec"; then
            echo "  ✓ Appears to have correct TLA+ structure"
        else
            echo "  ⚠ May have structural issues"
            all_good=false
        fi
    else
        echo "✗ $spec missing"
        all_good=false
    fi
done

if $all_good; then
    echo ""
    echo "All specification files are present and appear structurally correct."
    echo "To run full validation:"
    echo "1. Download TLA+ Toolkit from https://lamport.azurewebsites.net/tla/tla.html"
    echo "2. Extract tla2tools.jar into this directory"
    echo "3. Run: java -jar tla2tools.jar -config SeedLifecycle.cfg SeedLifecycle.tla"
else
    echo ""
    echo "Some specifications are missing or may have issues."
fi

echo ""
echo "Specifications reference:"
echo "- SeedLifecycle.tla: Seed planting, growth, and harvesting"
echo "- FederationEcon.tla: Reward pools, staking, and economic safety"
echo "- BelnapAggregation.tla: Four-valued logic for knowledge validation"
echo "- SchoolSafety.tla: Content filtering, panic buttons, and privacy"