# 👥 Employee Management Guide

## ✅ You Now Have 3 Employees!

| Employee | Annual Salary | Monthly Salary | Address |
|----------|--------------|----------------|---------|
| employee1 | 12 XLM | 1 XLM | GCBGNIA2KZ5C5Q3X737DEPBOUB56F5VRVSRBW5YX6CR6UH5KTYIYOYVA |
| employee2 | 24 XLM | 2 XLM | GCBDACPLLDVEC67IO6V7DCOVZFUH4DP5UTSO5CPHIB46F4VRN7PWGAT3 |
| employee3 | 36 XLM | 3 XLM | GBXCEWXR6J4DRLYCWCUBHBSKBFOQ3T4AJM5MXKCE6V3TAUJNBG6LDTNP |

**Total Monthly Payroll:** 6 XLM/month = 72 XLM/year

---

## 🆕 Add More Employees

### Using the Script (Easiest):
```bash
./add_employee.sh <employee_name> <monthly_salary_xlm>
```

**Examples:**
```bash
# Add employee4 with 4 XLM/month (48 XLM/year)
./add_employee.sh employee4 4

# Add employee5 with 5 XLM/month (60 XLM/year)
./add_employee.sh employee5 5

# Add John with 10 XLM/month (120 XLM/year)
./add_employee.sh john 10
```

**Important:** The script expects **monthly salary** in XLM. It will automatically:
1. Create a testnet account for the employee
2. Fund it with testnet XLM
3. Add them to the contract with the monthly salary converted to stroops
4. Show both monthly and annual amounts

### Manual Method:
```bash
# Step 1: Create account
stellar keys generate EMPLOYEE_NAME --network testnet --fund

# Step 2: Add to contract
# Note: base_salary is in stroops (1 XLM = 10,000,000 stroops)
# Example below: 10 XLM/month = 100,000,000 stroops
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- add_employee \
  --employee_address $(stellar keys address EMPLOYEE_NAME) \
  --name "EMPLOYEE_NAME" \
  --base_salary 100000000
```

**Stroops Conversion:**
- 1 XLM = 10,000,000 stroops
- 5 XLM = 50,000,000 stroops
- 10 XLM = 100,000,000 stroops

---

## 💰 Pay Salaries

### Pay All Employees (Easiest):
```bash
./pay_all_salaries.sh
```

This will automatically:
- ✅ Get list of all employees
- ✅ Pay each employee their monthly salary
- ✅ Show summary of payments

### Pay Single Employee:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- pay_salary \
  --employee_address EMPLOYEE_ADDRESS
```

**Pay specific employees:**
```bash
# Pay employee1
stellar contract invoke --id salary_system --source-account alice --network testnet -- pay_salary --employee_address GCBGNIA2KZ5C5Q3X737DEPBOUB56F5VRVSRBW5YX6CR6UH5KTYIYOYVA

# Pay employee2
stellar contract invoke --id salary_system --source-account alice --network testnet -- pay_salary --employee_address GCBDACPLLDVEC67IO6V7DCOVZFUH4DP5UTSO5CPHIB46F4VRN7PWGAT3

# Pay employee3
stellar contract invoke --id salary_system --source-account alice --network testnet -- pay_salary --employee_address GBXCEWXR6J4DRLYCWCUBHBSKBFOQ3T4AJM5MXKCE6V3TAUJNBG6LDTNP
```

---

## 🔍 View Employee Information

### Get All Employees:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- get_all_employees
```

### Get Specific Employee Details:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- get_employee \
  --employee_address EMPLOYEE_ADDRESS
```

### Calculate Current Salary (with increments):
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_current_salary \
  --employee_address EMPLOYEE_ADDRESS
```

---

## ⏸️ Stop/Resume Employee Salaries

### Stop Employee Salary:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- stop_employee_salary \
  --employee_address EMPLOYEE_ADDRESS
```

### Resume Employee Salary:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- resume_employee_salary \
  --employee_address EMPLOYEE_ADDRESS
```

---

## 🦊 Import Employees into Freighter

To view employee balances in Freighter, get their seed phrases:

```bash
# Employee 1
cat ~/.config/stellar/identity/employee1.toml

# Employee 2
cat ~/.config/stellar/identity/employee2.toml

# Employee 3
cat ~/.config/stellar/identity/employee3.toml
```

Then import the seed phrase into Freighter (Settings → TESTNET)

---

## 💵 Fund Treasury for Payroll

Calculate how much you need:
- **Monthly payroll:** 6 XLM (1 + 2 + 3)
- **For 12 months:** 72 XLM
- **With 5% annual increment:** ~80-85 XLM/year

```bash
# Fund with 500 XLM (covers ~6-7 years)
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- fund_treasury \
  --from $(stellar keys address alice) \
  --amount 50000000000
```

---

## 📊 Check System Status

### Treasury Balance:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- get_treasury_balance
```

### Calculate Sustainability:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_sustainability_years
```

This tells you how many years the treasury can sustain all employees with 5% annual increments.

---

## 🎯 Quick Commands Summary

```bash
# Add employee
./add_employee.sh employee4 48

# Pay all employees
./pay_all_salaries.sh

# Fund treasury (500 XLM)
stellar contract invoke --id salary_system --source-account alice --network testnet -- fund_treasury --from $(stellar keys address alice) --amount 50000000000

# Check treasury
stellar contract invoke --id salary_system --source-account alice --network testnet -- get_treasury_balance

# List all employees
stellar contract invoke --id salary_system --source-account alice --network testnet -- get_all_employees

# Check sustainability
stellar contract invoke --id salary_system --source-account alice --network testnet -- calculate_sustainability_years
```

---

## 🔗 View Everything on Stellar Expert

- **Contract:** https://stellar.expert/explorer/testnet/contract/CDXYPMRADZYRBUI2GNYICZBS7H6GJLX7BI3DI4HCUG2YISRR3NTLL4R4
- **Employee 1:** https://stellar.expert/explorer/testnet/account/GCBGNIA2KZ5C5Q3X737DEPBOUB56F5VRVSRBW5YX6CR6UH5KTYIYOYVA
- **Employee 2:** https://stellar.expert/explorer/testnet/account/GCBDACPLLDVEC67IO6V7DCOVZFUH4DP5UTSO5CPHIB46F4VRN7PWGAT3
- **Employee 3:** https://stellar.expert/explorer/testnet/account/GBXCEWXR6J4DRLYCWCUBHBSKBFOQ3T4AJM5MXKCE6V3TAUJNBG6LDTNP

---

## ✨ Your Salary System is Ready!

You can now:
- ✅ Add unlimited employees
- ✅ Pay monthly salaries with one command
- ✅ Track everything on the blockchain
- ✅ View in Freighter wallet
- ✅ Stop/resume employees
- ✅ Calculate sustainability

**Have fun managing your decentralized payroll!** 🚀
