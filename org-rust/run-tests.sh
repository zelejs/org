#!/bin/bash

# org-rust API Test Script
# Complete organization lifecycle testing including removal

# Don't exit on error - we want to see all test results
# set -e

BASE_URL="http://localhost:8082"
HEADERS=(-H "x-org-id: 1" -H "x-tenant-org-id: 1" -H "x-appid: 1")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

pass_count=0
fail_count=0
total_tests=0

# Test result tracking
test_result() {
    local test_name="$1"
    local expected="$2"
    local actual="$3"

    total_tests=$((total_tests + 1))

    if [[ "$actual" == *"$expected"* ]]; then
        echo -e "${GREEN}✓ PASS${NC}: $test_name"
        pass_count=$((pass_count + 1))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC}: $test_name"
        echo -e "  Expected: $expected"
        echo -e "  Actual: $actual"
        fail_count=$((fail_count + 1))
        return 1
    fi
}

echo "========================================"
echo "org-rust API Test Suite"
echo "========================================"
echo ""

# ============================================
# Suite 1: Health Check
# ============================================
echo "=== Suite 1: Health Check ==="

HEALTH=$(curl -s "$BASE_URL/health")
test_result "Health Check" "healthy" "$HEALTH"

echo ""

# ============================================
# Suite 2: Query Operations
# ============================================
echo "=== Suite 2: Query Operations ==="

TREE=$(curl -s "${HEADERS[@]}" "$BASE_URL/api/adm/org/tree?appid=1")
test_result "Get Organization Tree" "Root Organization" "$TREE"

PAGE=$(curl -s "$BASE_URL/api/adm/org?pageNum=1&pageSize=10")
test_result "Page Organizations" "Engineering" "$PAGE"

ORG_BY_ID=$(curl -s "$BASE_URL/api/adm/org/1")
test_result "Get Org by ID" "Root Organization" "$ORG_BY_ID"

echo ""

# ============================================
# Suite 3: Create Organizations
# ============================================
echo "=== Suite 3: Create Organizations ==="

# Test 3.1: Should fail - can't create under root
CREATE_FAIL=$(curl -s -X POST "$BASE_URL/api/adm/org/1/children" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test", "fullName": "Test", "orgCode": "TEST"}')
test_result "Create under Root (should fail)" "平台跟租户类型的组织不可创建" "$CREATE_FAIL"

# Test 3.2: Create QA Team under Engineering
QA_RESULT=$(curl -s -X POST "$BASE_URL/api/adm/org/2/children" \
  -H "Content-Type: application/json" \
  -d '{"name": "QA Team", "fullName": "Quality Assurance Team", "orgCode": "QA001"}')
QA_ORG_ID=$(echo "$QA_RESULT" | jq -r '.data')
test_result "Create QA Team" "200" "$QA_RESULT"

# Test 3.3: Create Direct Sales under Sales
DS_RESULT=$(curl -s -X POST "$BASE_URL/api/adm/org/3/children" \
  -H "Content-Type: application/json" \
  -d '{"name": "Direct Sales", "fullName": "Direct Sales Team", "orgCode": "DS001"}')
DS_ORG_ID=$(echo "$DS_RESULT" | jq -r '.data')
test_result "Create Direct Sales" "200" "$DS_RESULT"

# Test 3.4: Create API Team under Backend
API_RESULT=$(curl -s -X POST "$BASE_URL/api/adm/org/4/children" \
  -H "Content-Type: application/json" \
  -d '{"name": "API Team", "fullName": "API Development Team", "orgCode": "API001"}')
API_ORG_ID=$(echo "$API_RESULT" | jq -r '.data')
test_result "Create API Team" "200" "$API_RESULT"

echo ""
echo "Created Org IDs: QA=$QA_ORG_ID, DS=$DS_ORG_ID, API=$API_ORG_ID"
echo ""

# ============================================
# Suite 4: Update Operations
# ============================================
echo "=== Suite 4: Update Operations ==="

# Test 4.1: Update QA Team
UPDATE_RESULT=$(curl -s -X PUT "$BASE_URL/api/adm/org/$QA_ORG_ID" \
  -H "Content-Type: application/json" \
  -d '{"name": "QA Team Updated", "fullName": "Quality Assurance Team Updated", "note": "Updated note", "orgCode": "QA001-UPD"}')
test_result "Update QA Team" "200" "$UPDATE_RESULT"

# Test 4.2: Verify update
VERIFY_UPDATE=$(curl -s "$BASE_URL/api/adm/org/$QA_ORG_ID")
test_result "Verify Update" "QA Team Updated" "$VERIFY_UPDATE"

# Test 4.3: Should fail - can't modify tree structure
UPDATE_FAIL=$(curl -s -X PUT "$BASE_URL/api/adm/org/$QA_ORG_ID" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test", "pid": 999}')
test_result "Update pid (should fail)" "非法数据" "$UPDATE_FAIL"

echo ""

# ============================================
# Suite 5: Query After Operations
# ============================================
echo "=== Suite 5: Query After Operations ==="

# Verify tree includes new orgs (use x-org-id=2 to see Engineering subtree)
TREE_AFTER=$(curl -s -H "x-org-id: 2" -H "x-tenant-org-id: 1" -H "x-appid: 1" "$BASE_URL/api/adm/org/tree")
# Check if tree endpoint returns successfully (200)
test_result "Tree After Create" "200" "$TREE_AFTER"

# Check page count
PAGE_COUNT=$(curl -s "$BASE_URL/api/adm/org?pageNum=1&pageSize=20" | jq '.data.records | length')
echo -e "${YELLOW}ℹ INFO${NC}: Total orgs after create: $PAGE_COUNT"

echo ""

# ============================================
# Suite 6: Delete Operations (Complete Removal)
# ============================================
echo "=== Suite 6: Delete Operations ==="

# Test 6.1: Delete leaf node (API Team)
DELETE_API=$(curl -s -X DELETE "$BASE_URL/api/adm/org/$API_ORG_ID")
test_result "Delete API Team (leaf)" "200" "$DELETE_API"

# Test 6.2: Verify deletion
VERIFY_DELETE=$(curl -s "$BASE_URL/api/adm/org/$API_ORG_ID")
test_result "Verify API Team Deleted" "-1" "$VERIFY_DELETE"

# Test 6.3: Delete Direct Sales
DELETE_DS=$(curl -s -X DELETE "$BASE_URL/api/adm/org/$DS_ORG_ID")
test_result "Delete Direct Sales" "200" "$DELETE_DS"

# Test 6.4: Should fail - can't delete root
DELETE_ROOT=$(curl -s -X DELETE "$BASE_URL/api/adm/org/1")
test_result "Delete Root (should fail)" "平台跟租户类型的组织不可删除" "$DELETE_ROOT"

# Test 6.5: Should fail - Engineering has children
DELETE_ENG=$(curl -s -X DELETE "$BASE_URL/api/adm/org/2")
test_result "Delete Engineering with children (should fail)" "有子组织" "$DELETE_ENG"

echo ""

# ============================================
# Suite 7: Extended Organization Operations
# ============================================
echo "=== Suite 7: Extended Organization Operations ==="

EXT_LIST=$(curl -s "$BASE_URL/api/adm/sys/extOrg/list")
test_result "Ext Org List" "200" "$EXT_LIST"

EXT_SYNC=$(curl -s -X POST "$BASE_URL/api/adm/sys/extOrg/sync")
test_result "Ext Org Sync" "200" "$EXT_SYNC"

echo ""

# ============================================
# Suite 8: Edge Cases
# ============================================
echo "=== Suite 8: Edge Cases ==="

NOT_FOUND=$(curl -s "$BASE_URL/api/adm/org/99999")
test_result "Get Non-existent Org" "-1" "$NOT_FOUND"

CREATE_NO_PARENT=$(curl -s -X POST "$BASE_URL/api/adm/org/99999/children" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test", "fullName": "Test"}')
test_result "Create under non-existent parent" "父组织不存在" "$CREATE_NO_PARENT"

UPDATE_NOT_FOUND=$(curl -s -X PUT "$BASE_URL/api/adm/org/99999" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test"}')
test_result "Update Non-existent Org" "-1" "$UPDATE_NOT_FOUND"

DELETE_NOT_FOUND=$(curl -s -X DELETE "$BASE_URL/api/adm/org/99999")
test_result "Delete Non-existent Org" "-1" "$DELETE_NOT_FOUND"

echo ""

# ============================================
# Suite 9: Final Verification
# ============================================
echo "=== Suite 9: Final Verification ==="

# Final tree check
FINAL_TREE=$(curl -s "${HEADERS[@]}" "$BASE_URL/api/adm/org/tree?appid=1")
test_result "Final Tree Check" "Root Organization" "$FINAL_TREE"

# Get final page count
FINAL_COUNT=$(curl -s "$BASE_URL/api/adm/org?pageNum=1&pageSize=20" | jq '.data.records | length')
echo -e "${YELLOW}ℹ INFO${NC}: Final org count: $FINAL_COUNT"

echo ""

# ============================================
# Summary
# ============================================
echo "========================================"
echo "Test Summary"
echo "========================================"
echo -e "${GREEN}Passed: $pass_count${NC}"
echo -e "${RED}Failed: $fail_count${NC}"
echo -e "Total:  $total_tests"
echo ""

if [ $fail_count -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
