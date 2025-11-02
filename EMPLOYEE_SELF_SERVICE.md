# 💰 Employee Self-Service Salary Claims

## 🎉 New Feature Added!

Employees can now **withdraw their own salaries** without needing admin approval! This makes the system truly decentralized.

---

## 📋 How It Works

### Two Ways to Get Paid

**1. Admin Pays (Original Method)**
```bash
# Admin (alice) pays an employee
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- pay_salary \
  --employee_address EMPLOYEE_ADDRESS
```

**2. Employee Claims (New Self-Service Method) ✨**
```bash
# Employee claims their own salary
stellar contract invoke \
  --id salary_system \
  --source-account employee1 \
  --network testnet \
  -- claim_salary \
  --employee_address $(stellar keys address employee1)
```

---

## 🔐 Security

### Authentication
- **`pay_salary`**: Requires **admin** authentication
- **`claim_salary`**: Requires **employee** authentication (the employee themselves)

### Safety Features
Both methods have the same protections:
- ✅ Can only claim once per month
- ✅ Must be an active employee  
- ✅ Treasury must have sufficient balance
- ✅ Automatic 5% annual increment applied

---

## 💡 Usage Examples

### Employee1 Claims Their Salary
```bash
stellar contract invoke \
  --id salary_system \
  --source-account employee1 \
  --network testnet \
  -- claim_salary \
  --employee_address $(stellar keys address employee1)
```

### Employee2 Claims Their Salary
```bash
stellar contract invoke \
  --id salary_system \
  --source-account employee2 \
  --network testnet \
  -- claim_salary \
  --employee_address $(stellar keys address employee2)
```

### Employee3 Claims Their Salary
```bash
stellar contract invoke \
  --id salary_system \
  --source-account employee3 \
  --network testnet \
  -- claim_salary \
  --employee_address $(stellar keys address employee3)
```

---

## 📝 Create a Claim Script

Save this as `claim_my_salary.sh`:

```bash
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

echo ""
echo "✅ Done! Check your balance:"
echo "   stellar contract invoke --id salary_system --source-account alice --network testnet -- get_employee --employee_address $EMPLOYEE_ADDRESS"
```

Make it executable:
```bash
chmod +x claim_my_salary.sh
```

Use it:
```bash
./claim_my_salary.sh employee1
./claim_my_salary.sh employee2
./claim_my_salary.sh employee3
```

---

## 🆚 Comparison

| Feature | `pay_salary` (Admin) | `claim_salary` (Employee) |
|---------|---------------------|---------------------------|
| **Who initiates** | Admin (alice) | Employee themselves |
| **Authentication** | Admin account | Employee account |
| **Use case** | Bulk payments | Individual self-service |
| **Control** | Centralized | Decentralized ✨ |

---

## 🎯 Benefits of Self-Service

1. **Decentralization**: Employees control when they get paid
2. **Flexibility**: No need to wait for admin
3. **Autonomy**: Employees manage their own finances
4. **Gas Fees**: Each employee pays their own transaction fees
5. **Scalability**: Admin doesn't need to pay everyone manually

---

## ⚠️ Important Notes

### Monthly Limit
Both methods enforce: **One payment per month per employee**

```bash
# First claim in November - ✅ Success
./claim_my_salary.sh employee1

# Second claim in November - ❌ Fails
./claim_my_salary.sh employee1
# Error: "Salary already paid for this month"

# Claim in December - ✅ Success (new month)
./claim_my_salary.sh employee1
```

### Treasury Balance
If treasury runs out, **neither** method will work:
```
Error: "Insufficient treasury balance"
```

Solution: Admin needs to fund the treasury:
```bash
stellar contract invoke \
  --id salary_system \
  --source-account alice \
  --network testnet \
  -- fund_treasury \
  --from $(stellar keys address alice) \
  --amount 100000000
```

---

## 🔄 Workflow Options

### Option A: Admin-Driven (Original)
1. Admin runs `pay_all_salaries.sh` script
2. All employees get paid automatically
3. Admin pays all transaction fees

### Option B: Self-Service (New)
1. Each employee runs `claim_my_salary.sh <name>`
2. Employees get paid when they want
3. Each employee pays their own transaction fee

### Option C: Hybrid
- Admin pays some employees with `pay_salary`
- Other employees claim with `claim_salary`
- Mix and match as needed!

---

## 🧪 Testing

The contract includes tests for:
- ✅ Employee can claim their salary
- ✅ Employee cannot claim twice in same month
- ✅ Treasury balance updates correctly
- ✅ Employee record updates correctly

Run tests:
```bash
cd contracts/salary-system
cargo test test_employee_claim
```

---

## 📊 Contract Functions Summary

Total functions: **15**

**Employee Operations:**
- `claim_salary` - ✨ NEW: Employee claims their own salary
- `pay_salary` - Admin pays employee salary

**Admin Operations:**
- `initialize` - Set up contract
- `fund_treasury` - Add funds
- `add_employee` - Add new employee
- `stop_employee_salary` - Pause employee
- `resume_employee_salary` - Resume employee

**Read-Only:**
- `get_employee` - Get employee details
- `get_all_employees` - List all employees
- `get_treasury_balance` - Check funds
- `get_annual_increment` - Get increment %
- `calculate_current_salary` - Calculate salary
- `calculate_sustainability_years` - Years remaining
- `get_year` - Current year
- `get_month` - Current month

---

## 🚀 Next Steps

1. **Redeploy Contract** (if using testnet):
   ```bash
   cd contracts/salary-system
   stellar contract build
   stellar contract deploy --wasm target/wasm32v1-none/release/salary_system.wasm --source alice --network testnet
   ```

2. **Create Claim Script**:
   ```bash
   # Copy the script above to claim_my_salary.sh
   chmod +x claim_my_salary.sh
   ```

3. **Test It**:
   ```bash
   ./claim_my_salary.sh employee1
   ```

4. **Update Frontend** (Optional):
   Add a "Claim My Salary" button for employees in the HTML interface

---

## 💬 Summary

Your salary system is now **truly decentralized**! Employees have full control over claiming their salaries while maintaining all security features and payment limits. 🎉
