#!/bin/bash

echo "📊 Treasury & Sustainability Analysis"
echo "======================================"
echo ""

# Get treasury balance
echo "💰 Fetching treasury balance..."
TREASURY_STROOPS=$(stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- get_treasury_balance | tr -d '"')

TREASURY_XLM=$((TREASURY_STROOPS / 10000000))

echo "   Treasury Balance: $TREASURY_XLM XLM ($TREASURY_STROOPS stroops)"
echo ""

# Calculate sustainability
echo "📈 Calculating sustainability years..."
YEARS=$(stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_sustainability_years | tr -d '"')

echo "   Sustainability: $YEARS years (with 5% annual increments)"
echo ""

# Get employee count
echo "👥 Getting employee information..."
EMPLOYEE_COUNT=$(stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- get_all_employees | grep -o 'G[A-Z0-9]\{55\}' | wc -l | tr -d ' ')

echo "   Total Employees: $EMPLOYEE_COUNT"
echo ""

# Calculate monthly payroll (hardcoded for now - 6 XLM for 3 employees)
MONTHLY_PAYROLL=6
ANNUAL_PAYROLL=$((MONTHLY_PAYROLL * 12))

echo "📋 Summary:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Treasury Balance:    $TREASURY_XLM XLM"
echo "  Monthly Payroll:     $MONTHLY_PAYROLL XLM"
echo "  Annual Payroll:      $ANNUAL_PAYROLL XLM (Year 1)"
echo "  Total Employees:     $EMPLOYEE_COUNT"
echo "  Sustainability:      $YEARS years"
echo "  Annual Increment:    5% (compound)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ "$YEARS" -gt 5 ]; then
    echo "✅ Status: EXCELLENT - Well funded for long-term"
elif [ "$YEARS" -gt 2 ]; then
    echo "✅ Status: GOOD - Sufficient funds"
elif [ "$YEARS" -gt 1 ]; then
    echo "⚠️  Status: FAIR - Consider adding more funds soon"
else
    echo "❌ Status: LOW - Add funds immediately!"
fi
echo ""

# Calculate simple years without increments
SIMPLE_YEARS=$((TREASURY_XLM / ANNUAL_PAYROLL))
echo "💡 Insights:"
echo "   • Without increments: $SIMPLE_YEARS years"
echo "   • With 5% increments: $YEARS years"
echo "   • Difference: $((SIMPLE_YEARS - YEARS)) years due to salary growth"
echo ""
echo "📖 To improve sustainability:"
echo "   • Add more funds: ./add_employee.sh script includes funding commands"
echo "   • Or reduce payroll by stopping inactive employees"
echo ""
