#!/bin/bash

echo "👤 Adding New Employee to Salary System"
echo "========================================"
echo ""

# Check if employee name and salary are provided
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: ./add_employee.sh <employee_name> <monthly_salary_in_xlm>"
    echo "Example: ./add_employee.sh employee2 10"
    echo ""
    echo "This will:"
    echo "  1. Create a new testnet account for the employee"
    echo "  2. Fund it with testnet XLM"
    echo "  3. Add them to the salary contract with monthly salary"
    exit 1
fi

EMPLOYEE_NAME=$1
MONTHLY_SALARY_XLM=$2
MONTHLY_SALARY_STROOPS=$((MONTHLY_SALARY_XLM * 10000000))
ANNUAL_SALARY_XLM=$((MONTHLY_SALARY_XLM * 12))

echo "Employee: $EMPLOYEE_NAME"
echo "Monthly Salary: $MONTHLY_SALARY_XLM XLM"
echo "Annual Salary: $ANNUAL_SALARY_XLM XLM (with 5% yearly increment)"
echo ""

# Step 1: Create employee account
echo "1️⃣ Creating testnet account for $EMPLOYEE_NAME..."
stellar keys generate $EMPLOYEE_NAME --network testnet --fund 2>/dev/null

if [ $? -ne 0 ]; then
    echo "⚠️  Account '$EMPLOYEE_NAME' already exists. Using existing account."
fi

EMPLOYEE_ADDRESS=$(stellar keys address $EMPLOYEE_NAME)
echo "✅ Employee address: $EMPLOYEE_ADDRESS"
echo ""

# Step 2: Add to contract
echo "2️⃣ Adding $EMPLOYEE_NAME to contract..."
echo "   Monthly Salary: $MONTHLY_SALARY_XLM XLM"
echo "   Annual Salary: $ANNUAL_SALARY_XLM XLM"
echo ""

stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- add_employee \
  --employee_address $EMPLOYEE_ADDRESS \
  --name "$EMPLOYEE_NAME" \
  --base_salary $MONTHLY_SALARY_STROOPS

echo ""
echo "✅ SUCCESS! $EMPLOYEE_NAME added to salary system!"
echo ""
echo "📋 Employee Details:"
echo "   Name: $EMPLOYEE_NAME"
echo "   Address: $EMPLOYEE_ADDRESS"
echo "   Monthly Salary: $MONTHLY_SALARY_XLM XLM"
echo "   Annual Salary: $ANNUAL_SALARY_XLM XLM"
echo ""
echo "💡 To pay this employee's salary, run:"
echo "   stellar contract invoke --id salary_system --source-account alice --network testnet -- pay_salary --employee_address $EMPLOYEE_ADDRESS"
echo ""
echo "🔑 To import into Freighter:"
echo "   cat ~/.config/stellar/identity/$EMPLOYEE_NAME.toml"
