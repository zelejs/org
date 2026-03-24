# org-rust API Testing Plan

## Overview
Comprehensive API testing plan for the org-rust organization service, covering the complete organization lifecycle.

## Test Environment
- **Base URL**: `http://localhost:8082`
- **Database**: PostgreSQL `enrollment` @ `192.168.3.100:5432`
- **Test Headers**:
  ```bash
  x-org-id: 1
  x-tenant-org-id: 1
  x-appid: 1
  ```

---

## Test Suite 1: Health & Basic Connectivity

### Test 1.1: Health Check
```bash
curl -s http://localhost:8082/health | jq .
```
**Expected**: `{"status": "healthy"}`

---

## Test Suite 2: Organization Query Operations

### Test 2.1: Get Organization Tree
```bash
curl -s -H "x-org-id: 1" -H "x-tenant-org-id: 1" -H "x-appid: 1" \
  "http://localhost:8082/api/adm/org/tree?appid=1" | jq .
```
**Expected**: Tree structure with root org and children

### Test 2.2: Page Organizations (Default)
```bash
curl -s "http://localhost:8082/api/adm/org?pageNum=1&pageSize=10" | jq .
```
**Expected**: Paginated list of organizations

### Test 2.3: Get Organization by ID
```bash
curl -s "http://localhost:8082/api/adm/org/1" | jq .
```
**Expected**: Single organization details

---

## Test Suite 3: Organization Creation (Complete Lifecycle)

### Phase 1: Create Child Organizations

#### Test 3.1: Create Department under Root (Expected to Fail)
```bash
curl -s -X POST "http://localhost:8082/api/adm/org/1/children" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Department",
    "fullName": "Test Department Full",
    "orgCode": "TEST001"
  }' | jq .
```
**Expected**: Error - "平台跟租户类型的组织不可创建"

#### Test 3.2: Create Department under Engineering (Success)
```bash
curl -s -X POST "http://localhost:8082/api/adm/org/2/children" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "QA Team",
    "fullName": "Quality Assurance Team",
    "orgCode": "QA001"
  }' | jq .
```
**Expected**: `{"code": 200, "data": <new_org_id>}`
**Save the returned ID as `QA_ORG_ID`**

#### Test 3.3: Create Another Department under Sales
```bash
curl -s -X POST "http://localhost:8082/api/adm/org/3/children" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Direct Sales",
    "fullName": "Direct Sales Team",
    "orgCode": "DS001"
  }' | jq .
```
**Expected**: `{"code": 200, "data": <new_org_id>}`
**Save the returned ID as `DS_ORG_ID`**

#### Test 3.4: Create Nested Team under Backend Team
```bash
curl -s -X POST "http://localhost:8082/api/adm/org/4/children" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "API Team",
    "fullName": "API Development Team",
    "orgCode": "API001"
  }' | jq .
```
**Expected**: `{"code": 200, "data": <new_org_id>}`
**Save the returned ID as `API_ORG_ID`**

---

## Test Suite 4: Organization Update Operations

### Test 4.1: Update Organization Name and Note
```bash
curl -s -X PUT "http://localhost:8082/api/adm/org/${QA_ORG_ID}" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "QA Team Updated",
    "fullName": "Quality Assurance Team Updated",
    "note": "Updated note for QA team",
    "orgCode": "QA001-UPD"
  }' | jq .
```
**Expected**: `{"code": 200, "data": ${QA_ORG_ID}}`

### Test 4.2: Verify Update
```bash
curl -s "http://localhost:8082/api/adm/org/${QA_ORG_ID}" | jq .
```
**Expected**: Updated values reflected

### Test 4.3: Attempt to Modify Tree Structure Fields (Expected to Fail)
```bash
curl -s -X PUT "http://localhost:8082/api/adm/org/${QA_ORG_ID}" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test",
    "pid": 999
  }' | jq .
```
**Expected**: Error - "非法数据:不能修改组织层级相关字段"

---

## Test Suite 5: Organization Query After Operations

### Test 5.1: Verify Tree Structure with New Orgs
```bash
curl -s -H "x-org-id: 1" -H "x-tenant-org-id: 1" -H "x-appid: 1" \
  "http://localhost:8082/api/adm/org/tree?appid=1" | jq .
```
**Expected**: Tree includes all newly created organizations

### Test 5.2: Verify Pagination Count Increased
```bash
curl -s "http://localhost:8082/api/adm/org?pageNum=1&pageSize=20" | jq '.data.records | length'
```
**Expected**: Count should be > 5 (original 5 + new orgs)

---

## Test Suite 6: Organization Deletion (Complete Removal)

### Phase 1: Delete Leaf Organizations (No Children)

#### Test 6.1: Delete API Team (Leaf Node)
```bash
curl -s -X DELETE "http://localhost:8082/api/adm/org/${API_ORG_ID}" | jq .
```
**Expected**: `{"code": 200, "data": ${API_ORG_ID}}`

#### Test 6.2: Verify Deletion
```bash
curl -s "http://localhost:8082/api/adm/org/${API_ORG_ID}" | jq .
```
**Expected**: Error - "组织不存在" or `{"code": -1}`

### Phase 2: Delete Parent with Children (Expected to Fail)

#### Test 6.3: Attempt Delete Backend Team (Has Deleted Child)
```bash
curl -s -X DELETE "http://localhost:8082/api/adm/org/4" | jq .
```
**Expected**: Success (child was already deleted)

### Phase 3: Delete Direct Sales

#### Test 6.4: Delete Direct Sales Team
```bash
curl -s -X DELETE "http://localhost:8082/api/adm/org/${DS_ORG_ID}" | jq .
```
**Expected**: `{"code": 200, "data": ${DS_ORG_ID}}`

### Phase 4: Attempt Delete Root/OrgType 1 (Expected to Fail)

#### Test 6.5: Attempt Delete Root Organization
```bash
curl -s -X DELETE "http://localhost:8082/api/adm/org/1" | jq .
```
**Expected**: Error - "平台跟租户类型的组织不可删除"

#### Test 6.6: Attempt Delete Engineering (Type 2 with Children)
```bash
curl -s -X DELETE "http://localhost:8082/api/adm/org/2" | jq .
```
**Expected**: Error - "有子组织，不可删除"

---

## Test Suite 7: Party Organization Operations

### Test 7.1: List Party Organizations
```bash
curl -s "http://localhost:8082/api/adm/sys/partyOrg/list" | jq .
```
**Expected**: Empty list or existing party orgs

### Test 7.2: Sync Party Organizations
```bash
curl -s -X POST "http://localhost:8082/api/adm/sys/partyOrg/sync" | jq .
```
**Expected**: `{"code": 200, "data": 0}`

### Test 7.3: Add Party Organization (Requires Existing Parent)
```bash
# First, create a regular org to use as parent
NEW_ORG=$(curl -s -X POST "http://localhost:8082/api/adm/org/3/children" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Party Parent", "fullName": "Test Party Parent", "orgCode": "TPP001"}' | jq -r '.data')

# Then add party org relationship
curl -s -X POST "http://localhost:8082/api/adm/sys/partyOrg/add" \
  -H "Content-Type: application/json" \
  -d "{
    \"id\": 1001,
    \"parentId\": ${NEW_ORG},
    \"name\": \"Test Party Org\",
    \"shortName\": \"Test Party\",
    \"orgNum\": \"PARTY001\",
    \"type\": 1
  }" | jq .
```
**Expected**: `{"code": 200, "data": <new_org_id>}`

---

## Test Suite 8: Edge Cases & Error Handling

### Test 8.1: Get Non-existent Organization
```bash
curl -s "http://localhost:8082/api/adm/org/99999" | jq .
```
**Expected**: Error - "组织不存在"

### Test 8.2: Create Org under Non-existent Parent
```bash
curl -s -X POST "http://localhost:8082/api/adm/org/99999/children" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test", "fullName": "Test"}' | jq .
```
**Expected**: Error - "父组织不存在"

### Test 8.3: Update Non-existent Organization
```bash
curl -s -X PUT "http://localhost:8082/api/adm/org/99999" \
  -H "Content-Type: application/json" \
  -d '{"name": "Test"}' | jq .
```
**Expected**: Error - "组织不存在"

### Test 8.4: Delete Non-existent Organization
```bash
curl -s -X DELETE "http://localhost:8082/api/adm/org/99999" | jq .
```
**Expected**: Error - "组织不存在"

---

## Test Suite 9: Nested Set Model Verification

### Test 9.1: Verify left_num/right_num Consistency
```bash
# After all operations, verify tree integrity
curl -s "http://localhost:8082/api/adm/org/tree?appid=1" \
  -H "x-org-id: 1" -H "x-tenant-org-id: 1" -H "x-appid: 1" | \
  jq '.data.children[0].children | map({id, name, nodeLevel})'
```
**Expected**: Proper nesting with correct levels

### Test 9.2: Verify Parent-Child Relationships
```bash
curl -s "http://localhost:8082/api/adm/org?pageNum=1&pageSize=20" | \
  jq '.data.records | map({id, pid, name, nodeLevel})'
```
**Expected**: All records have valid pid (except root)

---

## Cleanup Script

```bash
# Clean up all test organizations created during testing
# Run this after completing all tests

# Delete leaf nodes first
# Then work up the tree

# Reset database to initial state (WARNING: This deletes all data)
sqlx migrate revert --database-url "postgresql://postgres:postgres@192.168.3.100:5432/enrollment" --source migrations
sqlx migrate run --database-url "postgresql://postgres:postgres@192.168.3.100:5432/enrollment" --source migrations
```

---

## Test Execution Checklist

- [ ] Suite 1: Health Check
- [ ] Suite 2: Query Operations
- [ ] Suite 3: Create Organizations
- [ ] Suite 4: Update Operations
- [ ] Suite 5: Query After Updates
- [ ] Suite 6: Delete Operations (Complete Removal)
- [ ] Suite 7: Party Organization Operations
- [ ] Suite 8: Edge Cases
- [ ] Suite 9: Nested Set Verification
- [ ] Cleanup

---

## Expected Test Results Summary

| Operation | Count | Status |
|-----------|-------|--------|
| Health Check | 1 | ✅ Pass |
| Query Operations | 3 | ✅ Pass |
| Create Operations | 4 | ✅ Pass (1 expected fail) |
| Update Operations | 3 | ✅ Pass (1 expected fail) |
| Delete Operations | 6 | ✅ Pass (2 expected fails) |
| Party Org Operations | 3 | ✅ Pass |
| Edge Cases | 4 | ✅ Pass (all expected fails) |
| Verification | 2 | ✅ Pass |

**Total**: 22 tests, 18 success, 4 expected failures
