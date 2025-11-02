#!/bin/bash

echo "💰 Claiming Your Salary"
echo "======================="
echo ""

# Check if employee name is provided
if [ -z "$1" ]; then
    echo "Usage: ./claim_my_salary.sh <your_employee_name>"
    echo "Example: ./claim_my_salary.sh employee1"
    echo ""
    echo "Available employees: employee1, employee2, employee3"
    exit 1
fi

EMPLOYEE_NAME=$1
EMPLOYEE_ADDRESS=$(stellar keys address $EMPLOYEE_NAME)

echo "Employee: $EMPLOYEE_NAME"
echo "Address: $EMPLOYEE_ADDRESS"
echo ""

# Claim salary
echo "💸 Claiming your monthly salary..."
stellar contract invoke \
  --id salary_system \
  --source-account $EMPLOYEE_NAME \
  --network testnet \
  -- claim_salary \
  --employee_address $EMPLOYEE_ADDRESS

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Your salary has been transferred!"
    echo ""
    echo "📋 Check your employee details:"
    echo "   stellar contract invoke --id salary_system --source-account alice --network testnet -- get_employee --employee_address $EMPLOYEE_ADDRESS"
else
    echo ""
    echo "❌ Failed! Possible reasons:"
    echo "   - Already claimed this month"
    echo "   - Not an active employee"
    echo "   - Insufficient treasury balance"
fi
