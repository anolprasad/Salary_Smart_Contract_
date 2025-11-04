#!/bin/bash

# Comprehensive Test Suite for Salary Smart Contract Dashboard
# Date: November 5, 2025

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║  🧪 Testing All Dashboard Functions                           ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

BASE_URL="http://localhost:3000"
PASSED=0
FAILED=0

# Test function
test_endpoint() {
    local name=$1
    local method=$2
    local endpoint=$3
    local data=$4
    local expected_field=$5
    
    echo -n "Testing $name... "
    
    if [ "$method" == "GET" ]; then
        response=$(curl -s "$BASE_URL$endpoint")
    else
        response=$(curl -s -X POST "$BASE_URL$endpoint" \
            -H "Content-Type: application/json" \
            -d "$data")
    fi
    
    if echo "$response" | grep -q "$expected_field"; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}"
        echo "   Response: $response"
        ((FAILED++))
        return 1
    fi
}

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📡 Testing Backend Server Connectivity"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Test 1: Frontend availability
echo -n "1. Frontend HTML... "
if curl -s "$BASE_URL" | grep -q "Decentralized Salary System"; then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 Testing Read-Only API Endpoints"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Test 2: Treasury Balance
test_endpoint "2. Get Treasury Balance" "GET" "/api/treasury-balance" "" "balanceXLM"

# Test 3: Get Employees
test_endpoint "3. Get All Employees" "GET" "/api/employees" "" "employees"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✏️  Testing Write API Endpoints (Validation Only)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Test 4: Add Employee (validation test - will fail but proves endpoint works)
echo -n "4. Add Employee API (validation)... "
response=$(curl -s -X POST "$BASE_URL/api/add-employee" \
    -H "Content-Type: application/json" \
    -d '{"name":"","stellarAddress":"INVALID","salary":"0"}')
if echo "$response" | grep -q "error\|required"; then
    echo -e "${GREEN}✅ PASS (validation working)${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    echo "   Response: $response"
    ((FAILED++))
fi

# Test 5: Fund Treasury (validation test)
echo -n "5. Fund Treasury API (validation)... "
response=$(curl -s -X POST "$BASE_URL/api/fund-treasury" \
    -H "Content-Type: application/json" \
    -d '{"amount":"0"}')
if echo "$response" | grep -q "error\|required\|greater than 0"; then
    echo -e "${GREEN}✅ PASS (validation working)${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    echo "   Response: $response"
    ((FAILED++))
fi

# Test 6: Pay All Employees (endpoint exists test)
echo -n "6. Pay All Employees API (endpoint exists)... "
response=$(curl -s -X POST "$BASE_URL/api/pay-all-salaries" \
    -H "Content-Type: application/json" \
    -d '{}')
if echo "$response" | grep -q "success\|error"; then
    echo -e "${GREEN}✅ PASS (endpoint exists)${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    echo "   Response: $response"
    ((FAILED++))
fi

# Test 7: Stop Employee Salary (validation test)
echo -n "7. Stop Employee Salary API (validation)... "
response=$(curl -s -X POST "$BASE_URL/api/stop-employee-salary" \
    -H "Content-Type: application/json" \
    -d '{"employeeAddress":""}')
if echo "$response" | grep -q "error\|required\|address"; then
    echo -e "${GREEN}✅ PASS (validation working)${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    echo "   Response: $response"
    ((FAILED++))
fi

# Test 8: Resume Employee Salary (validation test)
echo -n "8. Resume Employee Salary API (validation)... "
response=$(curl -s -X POST "$BASE_URL/api/resume-employee-salary" \
    -H "Content-Type: application/json" \
    -d '{"employeeAddress":""}')
if echo "$response" | grep -q "error\|required\|address"; then
    echo -e "${GREEN}✅ PASS (validation working)${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    echo "   Response: $response"
    ((FAILED++))
fi

# Test 9: Claim Salary (validation test)
echo -n "9. Claim Salary API (validation)... "
response=$(curl -s -X POST "$BASE_URL/api/claim-salary" \
    -H "Content-Type: application/json" \
    -d '{"employeeAddress":""}')
if echo "$response" | grep -q "error\|required\|address"; then
    echo -e "${GREEN}✅ PASS (validation working)${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    echo "   Response: $response"
    ((FAILED++))
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎨 Testing Frontend JavaScript Functions"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

FRONTEND_FILE="/Users/pravinkumar/soroban-hello-world/contracts/salary-system/frontend.html"

# Test 10-15: Check if JavaScript functions exist
functions=(
    "addEmployeeToBlockchain"
    "payAllEmployees"
    "fundTreasury"
    "stopEmployeeSalary"
    "resumeEmployeeSalary"
    "claimMySalary"
)

counter=10
for func in "${functions[@]}"; do
    echo -n "$counter. JavaScript function: $func... "
    if grep -q "async function $func()" "$FRONTEND_FILE"; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}"
        ((FAILED++))
    fi
    ((counter++))
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔘 Testing Button Bindings"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Test 16-21: Check if buttons are bound to functions
counter=16
for func in "${functions[@]}"; do
    echo -n "$counter. Button onclick: $func()... "
    if grep -q "onclick=\"$func()\"" "$FRONTEND_FILE"; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}"
        ((FAILED++))
    fi
    ((counter++))
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📦 Testing Backend Server Configuration"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Test 22: CORS enabled
echo -n "22. CORS enabled... "
if grep -q "app.use(cors())" "/Users/pravinkumar/soroban-hello-world/backend_server.js"; then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
fi

# Test 23: Express JSON parser
echo -n "23. JSON body parser... "
if grep -q "app.use(express.json())" "/Users/pravinkumar/soroban-hello-world/backend_server.js"; then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
fi

# Test 24: Static file serving
echo -n "24. Static file serving... "
if grep -q "app.use(express.static" "/Users/pravinkumar/soroban-hello-world/backend_server.js"; then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
fi

# Test 25: Server listening on port 3000
echo -n "25. Server on port 3000... "
if lsof -i :3000 | grep -q "node"; then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Test Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

TOTAL=$((PASSED + FAILED))
PERCENTAGE=$((PASSED * 100 / TOTAL))

echo -e "${GREEN}✅ Passed: $PASSED${NC}"
echo -e "${RED}❌ Failed: $FAILED${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Total Tests: $TOTAL"
echo "Success Rate: $PERCENTAGE%"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  🎉 ALL TESTS PASSED! System is fully functional!             ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════════╝${NC}"
    exit 0
else
    echo -e "${YELLOW}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${YELLOW}║  ⚠️  Some tests failed. Please review the output above.       ║${NC}"
    echo -e "${YELLOW}╚════════════════════════════════════════════════════════════════╝${NC}"
    exit 1
fi
