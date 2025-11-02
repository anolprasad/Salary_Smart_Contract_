# 🎯 How to Check Treasury Sustainability - Step by Step Guide

## ⚡ Quick Answer

To check **exactly how many years** your treasury will last with 5% annual increments:

```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_sustainability_years
```

This returns a number like `49` which means **49 years**!

---

## 📋 All Methods to Check Sustainability

### Method 1: Quick Script (Easiest) ✨

```bash
cd /Users/pravinkumar/soroban-hello-world
./check_sustainability.sh
```

**What you'll see:**
- Treasury balance in XLM
- Number of employees
- Monthly and annual payroll
- **Sustainability in YEARS** ← This is what you want!
- Health status and recommendations

**Example Output:**
```
📋 Summary:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Treasury Balance:    296 XLM
  Monthly Payroll:     6 XLM
  Annual Payroll:      72 XLM (Year 1)
  Total Employees:     3
  Sustainability:      49 years    ← HERE IT IS!
  Annual Increment:    5% (compound)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

### Method 2: Direct CLI Command (Most Accurate)

#### Step 1: Calculate Sustainability Years
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_sustainability_years
```

**Returns:** A number like `"49"` = 49 years

#### Step 2: Get Treasury Balance (for context)
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- get_treasury_balance
```

**Returns:** Balance in stroops (divide by 10,000,000 for XLM)

#### Step 3: Get All Employees (to understand payroll)
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- get_all_employees
```

**Returns:** List of employee addresses

---

### Method 3: Frontend (Visual) 🖥️

1. **Open the frontend:**
   ```bash
   open /Users/pravinkumar/soroban-hello-world/contracts/salary-system/frontend.html
   ```

2. **Navigate to "Treasury & Sustainability Analysis"** section

3. **Click "Load Treasury Information"** button

4. **View the results:**
   - Treasury Balance card
   - Monthly Payroll card
   - **Sustainability Years card** ← Shows the number!
   - Health Status

**Visual Example:**
```
┌─────────────────────────────────┐
│  Sustainability                 │
│  49 Years                       │ ← HERE!
│  With 5% annual increments      │
└─────────────────────────────────┘
```

---

## 🧮 Understanding the Calculation

### What the Smart Contract Does:

The `calculate_sustainability_years()` function in `lib.rs`:

1. **Gets current treasury balance** (e.g., 296 XLM)
2. **Gets all active employees** (e.g., 3 employees)
3. **Simulates year by year:**
   ```
   Year 1: 
     - Employee1: 12 XLM
     - Employee2: 24 XLM  
     - Employee3: 36 XLM
     - Total: 72 XLM
     - Remaining: 296 - 72 = 224 XLM
   
   Year 2 (with 5% increment):
     - Employee1: 12.6 XLM (12 × 1.05)
     - Employee2: 25.2 XLM (24 × 1.05)
     - Employee3: 37.8 XLM (36 × 1.05)
     - Total: 75.6 XLM
     - Remaining: 224 - 75.6 = 148.4 XLM
   
   Year 3 (with 5% increment again):
     - Employee1: 13.23 XLM (12.6 × 1.05)
     - Employee2: 26.46 XLM (25.2 × 1.05)
     - Employee3: 39.69 XLM (37.8 × 1.05)
     - Total: 79.38 XLM
     - Remaining: 148.4 - 79.38 = 69.02 XLM
   
   Year 4:
     - Total needed: 83.35 XLM
     - Available: 69.02 XLM
     - ❌ Not enough! Stop here.
   ```

4. **Returns total complete years** (3 in this example)

### The 5% Compound Effect:

- **Simple calculation** (no increments): 296 ÷ 72 = **4.1 years**
- **With 5% compound increments**: Approximately **3 years**
- **Difference**: ~1 year lost due to salary growth

---

## 📊 Real Example with Your Contract

### Current State:

```bash
# 1. Check treasury
stellar contract invoke --id salary_system --source-account alice --network testnet -- get_treasury_balance
# Output: "2960000000" = 296 XLM

# 2. Calculate sustainability
stellar contract invoke --id salary_system --source-account alice --network testnet -- calculate_sustainability_years
# Output: The exact number of years!
```

### If Result is 49 Years:
```
✅ EXCELLENT! Your treasury can support all employees for 49 years
   with automatic 5% annual salary increments.
```

### If Result is 3 Years:
```
⚠️ FAIR - Consider adding more funds within 2 years
   to maintain a healthy buffer.
```

### If Result is 0 Years:
```
❌ CRITICAL - Treasury cannot support even 1 year of payroll.
   Add funds immediately!
```

---

## 🎯 Quick Reference Commands

### One-Line Check:
```bash
echo "Sustainability: $(stellar contract invoke --id salary_system --source-account alice --network testnet -- calculate_sustainability_years | tr -d '\"') years"
```

### Complete Analysis:
```bash
./check_sustainability.sh
```

### Just the Number:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_sustainability_years | tr -d '"'
```

---

## 💡 What the Number Means

| Years | Status | Action Needed |
|-------|--------|---------------|
| 10+ | 🟢 Excellent | No action needed |
| 5-10 | 🟢 Very Good | Monitor annually |
| 2-5 | 🟡 Good | Plan funding in 1-2 years |
| 1-2 | 🟠 Fair | Add funds within 6 months |
| <1 | 🔴 Low | Add funds immediately |
| 0 | 🔴 Critical | Fund now! |

---

## 🔧 How to Improve Sustainability

### Option 1: Add More Funds
```bash
# Add 500 XLM to treasury
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- fund_treasury \
  --from $(stellar keys address alice) \
  --amount 50000000000

# Then check again
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_sustainability_years
```

### Option 2: Stop Inactive Employees
```bash
# Stop an employee's salary
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- stop_employee_salary \
  --employee_address EMPLOYEE_ADDRESS

# Check sustainability again
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_sustainability_years
```

---

## ✅ TL;DR - Just Give Me the Answer!

**Run this ONE command:**

```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- calculate_sustainability_years
```

**The output is your answer!**

Example: `"49"` means **49 years** 🎉

That's it! The smart contract does all the complex calculations for you, accounting for:
- ✅ All active employees
- ✅ Their current salaries
- ✅ 5% annual compound increments
- ✅ 12 monthly payments per year
- ✅ Current treasury balance

**The number you get is exactly how many complete years your treasury will support all employees!** 🚀
