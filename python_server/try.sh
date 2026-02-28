#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Supply Chain Event Adder${NC}"
echo "======================"
echo

# Get item details from user
read -p "Item ID (e.g. ECID123): " ITEM_ID
read -p "Event Type (manufacture/ship/receive): " EVENT_TYPE
read -p "Location (e.g. Factory Delhi): " LOCATION
read -p "Owner (e.g. SupplierA): " OWNER
read -p "Document Hash (or press Enter for auto): " DOC_HASH

# Auto-generate timestamp and doc hash if empty
TIMESTAMP=$(date -Iseconds -u)
DOC_HASH=${DOC_HASH:-"auto_$(date +%s)_$RANDOM"}

# JSON payload for your Rust app
EVENT_JSON=$(cat <<EOF
{
  "item_id": "$ITEM_ID",
  "event_type": "$EVENT_TYPE", 
  "location": "$LOCATION",
  "timestamp": "$TIMESTAMP",
  "owner": "$OWNER",
  "document_hash": "$DOC_HASH"
}
EOF
)

echo -e "\n${GREEN}Adding event:${NC}"
echo "$EVENT_JSON"
echo

# Call your Rust binary (adjust path/name)
cargo run --bin polari -- add $ITEM_ID $EVENT_TYPE $LOCATION $OWNER $DOC_HASH

echo -e "\n${GREEN}✅ Event added to blockchain!${NC}"
echo -e "${YELLOW}Trace: cargo run -- trace $ITEM_ID${NC}"
