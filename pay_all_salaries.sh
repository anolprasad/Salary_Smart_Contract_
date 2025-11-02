#!/bin/bash

echo "💰 Paying Salaries to All Employees"
echo "===================================="
echo ""

# Get all employee addresses from the contract
echo "📋 Getting list of employees..."
EMPLOYEES=$(stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- get_all_employees)

# Extract addresses (remove brackets and quotes)
ADDRESSES=$(echo $EMPLOYEES | grep -o 'G[A-Z0-9]\{55\}')

if [ -z "$ADDRESSES" ]; then
    echo "❌ No employees found in the contract!"
    exit 1
fi

echo "Found employees:"
COUNT=0
for ADDR in $ADDRESSES; do
    COUNT=$((COUNT + 1))
    echo "  $COUNT. $ADDR"
done
echo ""

# Pay each employee
PAID=0
FAILED=0

for EMPLOYEE_ADDR in $ADDRESSES; do
    echo "💸 Paying salary to: $EMPLOYEE_ADDR"
    
    RESULT=$(stellar contract invoke \
      --id salary_system \
      --source-account alice \
      --network testnet \
      -- pay_salary \
      --employee_address $EMPLOYEE_ADDR 2>&1)
    
    if echo "$RESULT" | grep -q "Success"; then
        echo "   ✅ Payment successful!"
        PAID=$((PAID + 1))
    else
        echo "   ⚠️  Payment failed or already paid this month"
        FAILED=$((FAILED + 1))
    fi
    echo ""
done

echo "================================================"
echo "📊 Summary:"
echo "   Total employees: $COUNT"
echo "   Paid successfully: $PAID"
echo "   Failed/Already paid: $FAILED"
echo ""
echo "✅ Salary payments complete!"
