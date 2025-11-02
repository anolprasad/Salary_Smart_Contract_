# Decentralized Salary System - Soroban Smart Contract

A complete decentralized salary management system built on Stellar's Soroban platform. This system features annual salary increments, treasury management, and sustainability calculations.

## 🌟 Features

- **Employee Management**: Add and manage employees with individual salaries
- **Annual Increments**: Automatic salary increases based on configurable percentage
- **Treasury System**: Fund and manage the organization's treasury
- **Sustainability Calculator**: Calculate how many years the treasury can sustain all salaries
- **Salary Payments**: Process annual salary payments to employees
- **Web Interface**: Beautiful HTML frontend for easy interaction

## 📋 Smart Contract Functions

### Admin Functions
- `initialize(admin, annual_increment_percent, token_address)` - Initialize the contract
- `add_employee(employee_address, name, base_salary)` - Add a new employee
- `pay_salary(employee_address)` - Pay annual salary to an employee
- `fund_treasury(from, amount)` - Add funds to the treasury

### Query Functions
- `calculate_current_salary(employee_address)` - Calculate employee's current salary with increments
- `calculate_sustainability_years()` - Calculate years the treasury can sustain operations
- `get_employee(employee_address)` - Get employee details
- `get_all_employees()` - Get list of all employees
- `get_treasury_balance()` - Get current treasury balance
- `get_annual_increment()` - Get annual increment percentage

## 🚀 Quick Start

### Prerequisites

1. **Install Rust and Soroban CLI**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install Soroban CLI
   cargo install --locked soroban-cli
   
   # Add wasm32 target
   rustup target add wasm32-unknown-unknown
   ```

2. **Configure Stellar Network**
   ```bash
   # Configure for Testnet
   soroban network add \
     --global testnet \
     --rpc-url https://soroban-testnet.stellar.org:443 \
     --network-passphrase "Test SDF Network ; September 2015"
   
   # Create an identity
   soroban keys generate --global alice --network testnet
   ```

### Build the Contract

```bash
cd contracts/salary-system

# Build optimized WASM
soroban contract build

# The compiled WASM will be at:
# ../../target/wasm32-unknown-unknown/release/salary_system.wasm
```

### Deploy to Testnet

```bash
# Deploy the contract
soroban contract deploy \
  --wasm ../../target/wasm32-unknown-unknown/release/salary_system.wasm \
  --source alice \
  --network testnet

# Save the returned contract ID
export CONTRACT_ID=<your-contract-id>
```

### Deploy a Test Token (for testing)

```bash
# Deploy Stellar Asset Contract (SAC)
soroban contract asset deploy \
  --asset native \
  --source alice \
  --network testnet

# Save the token contract ID
export TOKEN_ID=<your-token-id>
```

### Initialize the Contract

```bash
# Get your address
ADMIN_ADDRESS=$(soroban keys address alice)

# Initialize with 5% annual increment
soroban contract invoke \
  --id $CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  initialize \
  --admin $ADMIN_ADDRESS \
  --annual_increment_percent 5 \
  --token_address $TOKEN_ID
```

## 📝 Usage Examples

### Add an Employee

```bash
# Generate employee address
soroban keys generate --global bob --network testnet
EMPLOYEE_ADDRESS=$(soroban keys address bob)

# Add employee with 50 XLM annual salary (5,000,000,000 stroops)
soroban contract invoke \
  --id $CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  add_employee \
  --employee_address $EMPLOYEE_ADDRESS \
  --name "Bob Smith" \
  --base_salary 5000000000
```

### Fund the Treasury

```bash
# Wrap native XLM to use with the contract
soroban contract invoke \
  --id $TOKEN_ID \
  --source alice \
  --network testnet \
  -- \
  mint \
  --to $ADMIN_ADDRESS \
  --amount 10000000000

# Fund treasury with 100 XLM (10,000,000,000 stroops)
soroban contract invoke \
  --id $CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  fund_treasury \
  --from $ADMIN_ADDRESS \
  --amount 10000000000
```

### Check Treasury Balance

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  get_treasury_balance
```

### Calculate Sustainability

```bash
# Get how many years treasury can sustain operations
soroban contract invoke \
  --id $CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  calculate_sustainability_years
```

### Pay Employee Salary

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  pay_salary \
  --employee_address $EMPLOYEE_ADDRESS
```

### Get Employee Details

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  get_employee \
  --employee_address $EMPLOYEE_ADDRESS
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
cd contracts/salary-system
cargo test
```

Tests include:
- Contract initialization
- Treasury funding
- Employee management
- Salary calculations with increments
- Salary payments
- Sustainability calculations
- Multiple employee scenarios
- Error handling

## 🌐 Frontend Usage

1. **Open the Frontend**
   ```bash
   # Open frontend.html in your browser
   open frontend.html
   ```

2. **Install Freighter Wallet**
   - Install the [Freighter browser extension](https://www.freighter.app/)
   - Create or import a wallet
   - Switch to Testnet

3. **Connect & Configure**
   - Click "Connect Freighter Wallet"
   - Enter your deployed contract ID
   - Enter your token address
   - Set the annual increment percentage

4. **Use the Dashboard**
   - View treasury balance and sustainability
   - Add employees with their salaries
   - Fund the treasury
   - Process salary payments
   - Monitor all employees

## 💡 How It Works

### Annual Salary Increments

The contract uses compound annual increments:

```
Year 0: Base Salary = 100,000
Year 1: Salary = 100,000 + (100,000 × 5%) = 105,000
Year 2: Salary = 105,000 + (105,000 × 5%) = 110,250
Year 3: Salary = 110,250 + (110,250 × 5%) = 115,762.50
```

### Sustainability Calculation

The system simulates year-by-year spending to calculate how long the treasury can sustain all salaries:

1. Start with current treasury balance
2. Calculate total annual cost for all employees
3. Apply annual increment for next year
4. Repeat until treasury is exhausted
5. Return total years

### Time-Based System

The contract uses the Stellar ledger timestamp to determine the current year:
- Years are calculated from Unix epoch (1970)
- Each employee has a start year
- Salary calculations are based on years of service
- Payments can only be made once per year

## 📊 Data Structures

### Employee
```rust
pub struct Employee {
    pub address: Address,
    pub name: String,
    pub base_salary: i128,
    pub start_year: u64,
    pub last_payment_year: u64,
}
```

### Storage Keys
- `Admin` - Contract administrator address
- `TreasuryBalance` - Current treasury balance
- `AnnualIncrement` - Percentage increment (e.g., 5)
- `Employee(Address)` - Individual employee data
- `EmployeeList` - List of all employee addresses
- `TokenAddress` - Token contract address

## 🔒 Security Features

- **Authorization Required**: Admin-only functions require authentication
- **Double Payment Prevention**: Can't pay same employee twice in one year
- **Balance Checks**: Ensures sufficient treasury before payments
- **Initialization Protection**: Contract can only be initialized once

## 🎨 Frontend Features

- **Responsive Design**: Works on desktop and mobile
- **Real-time Updates**: Dashboard refreshes automatically
- **Visual Indicators**: Color-coded sustainability warnings
- **Employee Cards**: Clear display of all employee information
- **Transaction Status**: Success/error alerts for all operations
- **Local Storage**: Saves configuration between sessions

## 🔧 Customization

### Change Annual Increment
Modify during initialization or update the contract to allow changes:
```rust
pub fn update_increment(env: Env, new_percent: u32) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();
    env.storage().instance().set(&DataKey::AnnualIncrement, &new_percent);
}
```

### Add Bonus Payments
Extend the contract with one-time bonus functionality:
```rust
pub fn pay_bonus(env: Env, employee_address: Address, amount: i128) {
    // Implementation
}
```

### Remove Employees
Add employee removal functionality:
```rust
pub fn remove_employee(env: Env, employee_address: Address) {
    // Implementation
}
```

## 📚 Resources

- [Soroban Documentation](https://developers.stellar.org/docs/build/smart-contracts)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Stellar SDK](https://developers.stellar.org/docs/tools/sdks)
- [Freighter Wallet](https://www.freighter.app/)

## 🐛 Troubleshooting

### Contract Build Errors
```bash
# Clean and rebuild
cargo clean
soroban contract build
```

### Network Issues
```bash
# Check network configuration
soroban network ls

# Test RPC connection
curl https://soroban-testnet.stellar.org:443
```

### Authorization Errors
- Ensure you're using the correct identity
- Verify the admin address matches the contract admin
- Check that identity is funded on testnet

## 📄 License

MIT License - Feel free to use and modify for your projects!

## 🤝 Contributing

Contributions welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

---

Built with ❤️ on Stellar Soroban
