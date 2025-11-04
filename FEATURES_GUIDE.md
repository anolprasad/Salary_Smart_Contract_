# 🎯 Complete Features Guide - Salary Smart Contract Dashboard

**Last Updated**: November 4, 2025  
**Version**: 1.0  
**Dashboard URL**: http://localhost:3000

---

## 📋 Table of Contents

1. [Quick Start](#quick-start)
2. [Admin Features](#admin-features)
   - [Add New Employee](#1-add-new-employee)
   - [Pay All Employees](#2-pay-all-employees)
   - [Fund Treasury](#3-fund-treasury)
   - [Stop Employee Salary](#4-stop-employee-salary)
   - [Resume Employee Salary](#5-resume-employee-salary)
3. [Employee Features](#employee-features)
   - [Claim My Salary](#6-claim-my-salary)
4. [Backend API Reference](#backend-api-reference)
5. [Troubleshooting](#troubleshooting)

---

## 🚀 Quick Start

### Prerequisites
- Node.js v25.1.0 or higher
- Stellar CLI installed
- Contract deployed on Stellar testnet

### Starting the Server

```bash
cd /Users/pravinkumar/soroban-hello-world/contracts/salary-system
node backend_server.js
```

### Access Dashboard
Open your browser and go to: **http://localhost:3000**

---

## 👨‍💼 Admin Features

All admin features require you to use the admin account (alice).

### 1. 📝 Add New Employee

**Purpose**: Register a new employee in the salary system.

#### How to Use:
1. Navigate to the **"Employee Management"** section
2. Fill in the form:
   - **Employee Name**: The employee's name (e.g., "John Doe")
   - **Stellar Address**: Their Stellar public key (starts with 'G', 56 characters)
   - **Monthly Salary**: Salary in XLM (e.g., 5)
3. Click **"Add Employee"** button
4. Confirm the action in the popup dialog
5. Wait for confirmation

#### Validation Rules:
- ✅ Stellar address must start with 'G'
- ✅ Stellar address must be exactly 56 characters
- ✅ Salary must be a positive number
- ✅ Name is required

#### Success Message:
```
✅ Employee added successfully!
Name: John Doe
Address: GCBGNIA2KZ5C5Q3X...
Salary: 5 XLM/month
```

#### Common Errors:
- **"Employee already exists"**: This address is already registered
- **"Invalid address format"**: Check your address format
- **"Backend server not running"**: Start the server first

---

### 2. 💰 Pay All Employees

**Purpose**: Pay all registered employees their salary at once.

#### How to Use:
1. Navigate to the **"Salary Operations"** section
2. Click the **"💰 Pay All Employees"** button
3. Review the confirmation dialog showing:
   - Total number of employees
   - Estimated total cost
4. Click **OK** to proceed
5. Wait for the batch payment to complete

#### What Happens:
- System iterates through all registered employees
- Each eligible employee receives their monthly salary
- Employees who already claimed this month are skipped
- Treasury balance is updated automatically

#### Success Message:
```
✅ Successfully paid all employees!
Total Employees: 6
Payments Sent: 6
Total Amount: 30 XLM
```

#### Important Notes:
- ⚠️ Employees who already claimed this month won't receive duplicate payments
- ⚠️ Requires sufficient treasury balance
- ⚠️ Transaction may take a few seconds to complete
- ⚠️ This is an irreversible action

---

### 3. 💵 Fund Treasury

**Purpose**: Add funds to the contract treasury to pay employee salaries.

#### How to Use:
1. Navigate to the **"Salary Operations"** section
2. Find the **"Fund Treasury"** form
3. Enter the amount in XLM (e.g., 100)
4. Click **"Fund Treasury"** button
5. Confirm the transfer in the popup dialog
6. Wait for confirmation

#### Validation Rules:
- ✅ Amount must be greater than 0
- ✅ Must have sufficient balance in admin account
- ✅ Amount is in XLM (not stroops)

#### Success Message:
```
✅ Treasury funded successfully!
Amount Added: 100 XLM
New Balance: 496 XLM
```

#### Why Fund Treasury:
- Ensures there's always money to pay employees
- Required before paying salaries
- Shows in dashboard treasury balance
- Transparent tracking of funds

---

### 4. ⏸️ Stop Employee Salary

**Purpose**: Temporarily suspend salary payments to a specific employee.

#### How to Use:
1. Navigate to the **"Employee Management"** section
2. Find the **"Manage Employee Salary"** form
3. Enter the employee's Stellar address
4. Click **"⏸️ Stop Salary"** button
5. Confirm the action

#### What Happens:
- Employee's salary status is set to "stopped"
- They cannot claim salary until resumed
- Existing salary amount is preserved
- Can be reversed anytime

#### Use Cases:
- Temporary leave of absence
- Performance review period
- Suspension for investigation
- Contract negotiation period

#### Success Message:
```
✅ Employee salary stopped successfully!
Address: GCBGNIA2KZ5C5Q3X...
Status: Payments suspended
```

---

### 5. ▶️ Resume Employee Salary

**Purpose**: Restore salary payments for a previously stopped employee.

#### How to Use:
1. Navigate to the **"Employee Management"** section
2. Find the **"Manage Employee Salary"** form
3. Enter the employee's Stellar address
4. Click **"▶️ Resume Salary"** button
5. Confirm the action

#### What Happens:
- Employee's salary status is set to "active"
- They can claim salary again
- Previous salary amount is retained
- Takes effect immediately

#### Success Message:
```
✅ Employee salary resumed successfully!
Address: GCBGNIA2KZ5C5Q3X...
Status: Payments active
```

---

## 👤 Employee Features

### 6. 💰 Claim My Salary

**Purpose**: Employee self-service feature to claim monthly salary.

#### How to Use:
1. Navigate to the **"Employee Self-Service"** section (green theme)
2. Fill in the form:
   - **Employee Name** (Optional): Your name for reference
   - **Your Stellar Address** (Required): Your registered address
3. Click **"💰 Claim My Salary Now"** button
4. Review confirmation dialog
5. Click **OK** to claim
6. Wait for the transfer

#### Eligibility Requirements:
- ✅ Must be registered as an employee
- ✅ Haven't claimed in the last 30 days
- ✅ Salary status is "active" (not stopped)
- ✅ Treasury has sufficient balance

#### Claim Rules:
- **One Claim Per Month**: Can only claim once every 30 days
- **Automatic Tracking**: System tracks your last claim date
- **Direct Transfer**: Funds sent directly to your account
- **Full Amount**: You receive your complete registered salary

#### Success Message:
```
✅ Salary claimed successfully!
Employee: John Doe
Address: GCBGNIA2KZ5C5Q3X...
💰 Your salary has been transferred to your account!
```

#### Common Errors:

**Not Registered:**
```
❌ Employee Not Registered
You are not registered in the salary system.
Contact admin to be added as an employee.
```

**Already Claimed:**
```
❌ Already Claimed This Month
You already claimed your salary this month.
You can claim again after 30 days.
```

**Salary Stopped:**
```
❌ Salary Payments Suspended
Your salary is temporarily stopped.
Contact admin for more information.
```

---

## 🔌 Backend API Reference

### Base URL
```
http://localhost:3000
```

### Endpoints

#### 1. POST /api/add-employee
Add a new employee to the system.

**Request Body:**
```json
{
  "name": "John Doe",
  "stellarAddress": "GCBGNIA2KZ5C5Q3X737DEPBOUB56F5VRVSRBW5YX6CR6UH5KTYIYOYVA",
  "salary": "5"
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Employee added successfully!",
  "data": {
    "name": "John Doe",
    "stellarAddress": "GCBGNIA2KZ5C5Q3X...",
    "salary": "5 XLM"
  }
}
```

---

#### 2. POST /api/pay-all-salaries
Pay all registered employees at once.

**Request Body:**
```json
{
  "adminAddress": "GBPAKDYZKKXENIPOXGYCB2ZEVRQKK2JEBLPJ3CHK7MRMBZLHNPTGJAZE"
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Successfully paid all employees",
  "totalEmployees": 6,
  "paymentsSent": 6,
  "totalAmount": "30 XLM"
}
```

---

#### 3. POST /api/fund-treasury
Add funds to the contract treasury.

**Request Body:**
```json
{
  "amount": "100"
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Treasury funded successfully",
  "amountAdded": "100 XLM",
  "newBalance": "496 XLM"
}
```

---

#### 4. POST /api/stop-employee-salary
Stop salary payments for an employee.

**Request Body:**
```json
{
  "employeeAddress": "GCBGNIA2KZ5C5Q3X737DEPBOUB56F5VRVSRBW5YX6CR6UH5KTYIYOYVA"
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Employee salary stopped successfully",
  "data": {
    "employeeAddress": "GCBGNIA2KZ5C5Q3X...",
    "status": "stopped"
  }
}
```

---

#### 5. POST /api/resume-employee-salary
Resume salary payments for an employee.

**Request Body:**
```json
{
  "employeeAddress": "GCBGNIA2KZ5C5Q3X737DEPBOUB56F5VRVSRBW5YX6CR6UH5KTYIYOYVA"
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Employee salary resumed successfully",
  "data": {
    "employeeAddress": "GCBGNIA2KZ5C5Q3X...",
    "status": "active"
  }
}
```

---

#### 6. POST /api/claim-salary
Employee claims their own salary.

**Request Body:**
```json
{
  "employeeAddress": "GCBGNIA2KZ5C5Q3X737DEPBOUB56F5VRVSRBW5YX6CR6UH5KTYIYOYVA",
  "employeeName": "John Doe"
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Salary claimed successfully!",
  "data": {
    "employeeAddress": "GCBGNIA2KZ5C5Q3X...",
    "employeeName": "John Doe"
  },
  "note": "Your salary has been transferred to your account!"
}
```

---

#### 7. GET /api/treasury-balance
Get current treasury balance.

**Response:**
```json
{
  "success": true,
  "balance": "396 XLM"
}
```

---

#### 8. GET /api/employees
Get list of all registered employees.

**Response:**
```json
{
  "success": true,
  "employees": [
    {
      "name": "Alice",
      "address": "GBPAK...",
      "salary": "5 XLM",
      "status": "active"
    }
  ],
  "totalEmployees": 6
}
```

---

#### 9. GET /api/employee/:address
Get details of a specific employee.

**Response:**
```json
{
  "success": true,
  "employee": {
    "name": "John Doe",
    "address": "GCBGNIA2KZ5C5Q3X...",
    "salary": "5 XLM",
    "lastClaim": "2025-10-05",
    "status": "active"
  }
}
```

---

## 🔧 Troubleshooting

### Backend Server Not Running

**Symptoms:**
- Cannot access http://localhost:3000
- Buttons show "Backend server not running" error
- Connection refused errors

**Solution:**
```bash
cd /Users/pravinkumar/soroban-hello-world/contracts/salary-system
node backend_server.js
```

Keep the terminal window open while using the dashboard.

---

### Invalid Stellar Address

**Symptoms:**
- "Invalid address format" error
- Red validation message

**Solution:**
- Stellar addresses must start with 'G'
- Must be exactly 56 characters long
- Copy address directly from your Stellar wallet
- Example: `GCBGNIA2KZ5C5Q3X737DEPBOUB56F5VRVSRBW5YX6CR6UH5KTYIYOYVA`

---

### Already Claimed This Month

**Symptoms:**
- Employee cannot claim salary
- "Already claimed this month" error

**Solution:**
- Wait 30 days from last claim
- Check last claim date in employee info
- System enforces one claim per 30-day period
- This is by design to prevent duplicate payments

---

### Insufficient Treasury Funds

**Symptoms:**
- Cannot pay employees
- "Insufficient funds" error

**Solution:**
1. Check treasury balance on dashboard
2. Use "Fund Treasury" feature
3. Add sufficient XLM to cover all salaries
4. Recommended: Keep buffer for multiple months

---

### Employee Not Found

**Symptoms:**
- "Employee not registered" error
- Cannot claim salary

**Solution:**
1. Verify you're using the correct Stellar address
2. Check if admin added you to the system
3. Contact admin to register you as an employee
4. Admin uses "Add New Employee" feature

---

### Port Already in Use

**Symptoms:**
- "Port 3000 already in use" error
- Cannot start server

**Solution:**
```bash
# Find and kill the process using port 3000
lsof -i :3000
kill -9 [PID]

# Or kill all node processes
pkill -f "node backend_server.js"

# Then restart
node backend_server.js
```

---

## 📊 Contract Information

- **Contract ID**: `CDXYPMRADZYRBUI2GNYICZBS7H6GJLX7BI3DI4HCUG2YISRR3NTLL4R4`
- **Network**: Stellar Testnet
- **Token**: Native XLM (`CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`)
- **Admin Account**: alice (`GBPAKDYZKKXENIPOXGYCB2ZEVRQKK2JEBLPJ3CHK7MRMBZLHNPTGJAZE`)

**View on Stellar Expert**:  
https://stellar.expert/explorer/testnet/contract/CDXYPMRADZYRBUI2GNYICZBS7H6GJLX7BI3DI4HCUG2YISRR3NTLL4R4

---

## 🎯 Best Practices

### For Admins:
1. ✅ Keep treasury funded with at least 2-3 months buffer
2. ✅ Regularly check employee list and status
3. ✅ Document reasons for stopping/resuming salaries
4. ✅ Use "Pay All" feature at regular intervals
5. ✅ Monitor treasury balance before payments

### For Employees:
1. ✅ Verify your Stellar address before claiming
2. ✅ Claim salary at regular intervals (e.g., monthly)
3. ✅ Wait full 30 days between claims
4. ✅ Check your Stellar account to verify receipt
5. ✅ Contact admin if you encounter issues

### For Developers:
1. ✅ Keep backend server running
2. ✅ Monitor logs in `/tmp/backend.log`
3. ✅ Test features on testnet first
4. ✅ Backup important data regularly
5. ✅ Keep dependencies updated

---

## 🚀 Quick Command Reference

```bash
# Start Backend Server
cd /Users/pravinkumar/soroban-hello-world/contracts/salary-system
node backend_server.js

# Check if server is running
lsof -i :3000

# View backend logs
tail -f /tmp/backend.log

# Stop server
pkill -f "node backend_server.js"

# Install dependencies (if needed)
npm install
```

---

## 📚 Additional Resources

- **Main README**: [README.md](README.md)
- **Deployment Status**: [DEPLOYMENT_STATUS.md](DEPLOYMENT_STATUS.md)
- **Stellar Documentation**: https://developers.stellar.org
- **Stellar Expert**: https://stellar.expert
- **Freighter Wallet**: https://www.freighter.app

---

**Need Help?**  
Contact your system administrator or refer to the troubleshooting section above.

**Feature Requests?**  
This dashboard is fully functional with all core features. Suggestions welcome!

---

*This guide covers all 6 interactive features of the Salary Smart Contract Dashboard. No CLI commands required - everything works through the web interface!* 🎉
