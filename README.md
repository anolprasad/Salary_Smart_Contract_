# Decentralized Salary System - Soroban Smart Contract

A complete decentralized salary management system built on Stellar's Soroban platform with monthly payments, annual increments, treasury management, and sustainability calculations.
## Instructions are there in the html file
## 🌟 Features

- **Employee Management**: Add and manage employees with individual monthly salaries
- **Monthly Payments**: Process salary payments every month to active employees
- **Annual Increments**: Automatic 5% compound salary increases every year
- **Treasury System**: Fund and manage the organization's treasury
- **Sustainability Calculator**: Calculate how many years the treasury can sustain all salaries
- **Web Interface**: Beautiful HTML frontend for viewing contract status and employee data

## 📁 Project Structure

```text
soroban-hello-world/
├── contracts/
│   └── salary-system/
│       ├── src/
│       │   ├── lib.rs              # Main smart contract
│       │   └── test.rs             # Contract tests
│       ├── frontend.html           # Web interface
│       ├── Cargo.toml
│       └── README.md
├── add_employee.sh                 # Script to add employees
├── pay_all_salaries.sh            # Script to pay all salaries
├── check_sustainability.sh        # Script to check treasury health
├── EMPLOYEE_MANAGEMENT.md         # Employee guide
├── HOW_TO_CHECK_SUSTAINABILITY.md # Treasury guide
├── TREASURY_SUSTAINABILITY_GUIDE.md
├── Cargo.toml
└── README.md
```

## 🚀 Quick Start

### 1. Build the Contract
```bash
cd contracts/salary-system
stellar contract build
```

### 2. Deploy to Testnet
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/salary_system.wasm \
  --source alice \
  --network testnet
```

### 3. Initialize Contract
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- initialize \
  --token <TOKEN_ADDRESS> \
  --admin <ADMIN_ADDRESS>
```

### 4. Use Helper Scripts
```bash
# Add an employee (creates account + adds to contract)
./add_employee.sh employee_name monthly_salary_xlm

# Pay all employees
./pay_all_salaries.sh

# Check treasury sustainability
./check_sustainability.sh
```

## 📊 Current Status

- **Contract ID**: `CDXYPMRADZYRBUI2GNYICZBS7H6GJLX7BI3DI4HCUG2YISRR3NTLL4R4`
- **Network**: Stellar Testnet
- **Admin**: alice
- **Employees**: 3 active
- **Treasury**: 296 XLM
- **Sustainability**: 3 years (with 5% annual compound increments)

## 📖 Documentation

- [`contracts/salary-system/README.md`](contracts/salary-system/README.md) - Contract details and API
- [`EMPLOYEE_MANAGEMENT.md`](EMPLOYEE_MANAGEMENT.md) - How to manage employees
- [`HOW_TO_CHECK_SUSTAINABILITY.md`](HOW_TO_CHECK_SUSTAINABILITY.md) - Treasury analysis guide
- [`TREASURY_SUSTAINABILITY_GUIDE.md`](TREASURY_SUSTAINABILITY_GUIDE.md) - Detailed treasury info

## 🌐 Web Interface

Open `contracts/salary-system/frontend.html` in a browser to:
- View contract information
- Check treasury balance and sustainability
- See employee list
- View network status

## 🧪 Run Tests

```bash
cd contracts/salary-system
cargo test
```

All tests include real token transfers on testnet!

## 🔧 Technologies

- **Blockchain**: Stellar Soroban
- **Language**: Rust (smart contract)
- **Frontend**: HTML/CSS/JavaScript
- **Network**: Stellar Testnet
- **Token**: Native XLM (wrapped)

## Deploy

```bash
soroban contract deploy \
  --wasm ../../target/wasm32-unknown-unknown/release/salary_system.wasm \
  --source alice \
  --network testnet
```

For complete deployment and usage instructions, see the [salary-system README](contracts/salary-system/README.md).

---

Built with ❤️ on Stellar Soroban
