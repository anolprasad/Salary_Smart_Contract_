#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, String, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct Employee {
    pub address: Address,
    pub name: String,
    pub base_salary: i128,        // Annual base salary
    pub start_year: u64,          // Year when employee started
    pub start_month: u32,         // Month when employee started (1-12)
    pub last_payment_year: u64,   // Last year payment was made
    pub last_payment_month: u32,  // Last month payment was made (1-12)
    pub is_active: bool,          // Whether employee can receive salary
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    TreasuryBalance,
    AnnualIncrement,              // Percentage increment (e.g., 5 = 5%)
    Employee(Address),
    EmployeeList,
    TokenAddress,
}

#[contract]
pub struct SalarySystemContract;

#[contractimpl]
impl SalarySystemContract {
    /// Initialize the contract with admin, annual increment rate, and token
    pub fn initialize(
        env: Env,
        admin: Address,
        annual_increment_percent: u32,
        token_address: Address,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::AnnualIncrement, &annual_increment_percent);
        env.storage()
            .instance()
            .set(&DataKey::TokenAddress, &token_address);
        env.storage().instance().set(&DataKey::TreasuryBalance, &0i128);
        env.storage()
            .instance()
            .set(&DataKey::EmployeeList, &Vec::<Address>::new(&env));
    }

    /// Fund the treasury
    pub fn fund_treasury(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .unwrap();
        let token_client = token::Client::new(&env, &token_address);

        // Transfer tokens to contract
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        // Update treasury balance
        let current_balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TreasuryBalance)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TreasuryBalance, &(current_balance + amount));
    }

    /// Add a new employee
    pub fn add_employee(
        env: Env,
        employee_address: Address,
        name: String,
        base_salary: i128,
    ) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let (current_year, current_month) = Self::get_current_year_month(&env);

        let employee = Employee {
            address: employee_address.clone(),
            name,
            base_salary,
            start_year: current_year,
            start_month: current_month,
            last_payment_year: current_year,
            last_payment_month: 0,
            is_active: true,
        };

        env.storage()
            .instance()
            .set(&DataKey::Employee(employee_address.clone()), &employee);

        // Add to employee list
        let mut employees: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::EmployeeList)
            .unwrap_or(Vec::new(&env));
        employees.push_back(employee_address);
        env.storage()
            .instance()
            .set(&DataKey::EmployeeList, &employees);
    }

    /// Calculate current monthly salary for an employee based on years of service
    pub fn calculate_current_salary(env: Env, employee_address: Address) -> i128 {
        let employee: Employee = env
            .storage()
            .instance()
            .get(&DataKey::Employee(employee_address))
            .unwrap();

        let current_year = Self::get_current_year(&env);
        let years_of_service = current_year - employee.start_year;

        let increment_percent: u32 = env
            .storage()
            .instance()
            .get(&DataKey::AnnualIncrement)
            .unwrap();

        // Calculate annual salary with compound annual increment
        let mut annual_salary = employee.base_salary;
        for _ in 0..years_of_service {
            annual_salary = annual_salary + (annual_salary * increment_percent as i128 / 100);
        }

        // Convert to monthly salary
        let monthly_salary = annual_salary / 12;
        monthly_salary
    }

    /// Pay monthly salary to an employee
    pub fn pay_salary(env: Env, employee_address: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut employee: Employee = env
            .storage()
            .instance()
            .get(&DataKey::Employee(employee_address.clone()))
            .unwrap();

        // Check if employee is active
        if !employee.is_active {
            panic!("Employee is not active");
        }

        let (current_year, current_month) = Self::get_current_year_month(&env);

        // Check if already paid this month
        if employee.last_payment_year == current_year 
            && employee.last_payment_month == current_month {
            panic!("Salary already paid for this month");
        }

        let monthly_salary = Self::calculate_current_salary(env.clone(), employee_address.clone());

        let treasury_balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TreasuryBalance)
            .unwrap();

        if treasury_balance < monthly_salary {
            panic!("Insufficient treasury balance");
        }

        // Transfer salary
        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .unwrap();
        let token_client = token::Client::new(&env, &token_address);

        token_client.transfer(&env.current_contract_address(), &employee.address, &monthly_salary);

        // Update treasury and employee record
        env.storage()
            .instance()
            .set(&DataKey::TreasuryBalance, &(treasury_balance - monthly_salary));

        employee.last_payment_year = current_year;
        employee.last_payment_month = current_month;
        env.storage()
            .instance()
            .set(&DataKey::Employee(employee_address), &employee);
    }

    /// Allow employees to claim their own salary (self-service)
    pub fn claim_salary(env: Env, employee_address: Address) {
        // Require authentication from the employee themselves
        employee_address.require_auth();

        let mut employee: Employee = env
            .storage()
            .instance()
            .get(&DataKey::Employee(employee_address.clone()))
            .unwrap();

        // Check if employee is active
        if !employee.is_active {
            panic!("Employee is not active");
        }

        let (current_year, current_month) = Self::get_current_year_month(&env);

        // Check if already paid this month
        if employee.last_payment_year == current_year 
            && employee.last_payment_month == current_month {
            panic!("Salary already paid for this month");
        }

        let monthly_salary = Self::calculate_current_salary(env.clone(), employee_address.clone());

        let treasury_balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TreasuryBalance)
            .unwrap();

        if treasury_balance < monthly_salary {
            panic!("Insufficient treasury balance");
        }

        // Transfer salary
        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .unwrap();
        let token_client = token::Client::new(&env, &token_address);

        token_client.transfer(&env.current_contract_address(), &employee.address, &monthly_salary);

        // Update treasury and employee record
        env.storage()
            .instance()
            .set(&DataKey::TreasuryBalance, &(treasury_balance - monthly_salary));

        employee.last_payment_year = current_year;
        employee.last_payment_month = current_month;
        env.storage()
            .instance()
            .set(&DataKey::Employee(employee_address), &employee);
    }

    /// Stop salary payments for an employee
    pub fn stop_employee_salary(env: Env, employee_address: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut employee: Employee = env
            .storage()
            .instance()
            .get(&DataKey::Employee(employee_address.clone()))
            .unwrap();

        employee.is_active = false;
        env.storage()
            .instance()
            .set(&DataKey::Employee(employee_address), &employee);
    }

    /// Resume salary payments for an employee
    pub fn resume_employee_salary(env: Env, employee_address: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut employee: Employee = env
            .storage()
            .instance()
            .get(&DataKey::Employee(employee_address.clone()))
            .unwrap();

        employee.is_active = true;
        env.storage()
            .instance()
            .set(&DataKey::Employee(employee_address), &employee);
    }

    /// Calculate how many years the treasury can sustain all employees
    pub fn calculate_sustainability_years(env: Env) -> u32 {
        let treasury_balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TreasuryBalance)
            .unwrap_or(0);

        let employees: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::EmployeeList)
            .unwrap_or(Vec::new(&env));

        if employees.is_empty() {
            return 0;
        }

        let increment_percent: u32 = env
            .storage()
            .instance()
            .get(&DataKey::AnnualIncrement)
            .unwrap();

        let current_year = Self::get_current_year(&env);
        let mut remaining_balance = treasury_balance;
        let mut years = 0u32;

        // Simulate year by year (calculating annual costs)
        loop {
            let mut annual_cost = 0i128;

            // Calculate total annual cost for all employees
            for i in 0..employees.len() {
                let employee_addr = employees.get(i).unwrap();
                let employee: Employee = env
                    .storage()
                    .instance()
                    .get(&DataKey::Employee(employee_addr))
                    .unwrap();

                let years_of_service = (current_year + years as u64) - employee.start_year;
                
                // Calculate annual salary with increments
                let mut annual_salary = employee.base_salary;
                for _ in 0..years_of_service {
                    annual_salary = annual_salary + (annual_salary * increment_percent as i128 / 100);
                }

                annual_cost += annual_salary;
            }

            if remaining_balance < annual_cost {
                break;
            }

            remaining_balance -= annual_cost;
            years += 1;

            // Safety limit
            if years > 1000 {
                break;
            }
        }

        years
    }

    /// Get employee details
    pub fn get_employee(env: Env, employee_address: Address) -> Employee {
        env.storage()
            .instance()
            .get(&DataKey::Employee(employee_address))
            .unwrap()
    }

    /// Get all employees
    pub fn get_all_employees(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::EmployeeList)
            .unwrap_or(Vec::new(&env))
    }

    /// Get treasury balance
    pub fn get_treasury_balance(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TreasuryBalance)
            .unwrap_or(0)
    }

    /// Get annual increment percentage
    pub fn get_annual_increment(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::AnnualIncrement)
            .unwrap()
    }

    /// Helper function to get current year and month (using ledger timestamp)
    fn get_current_year_month(env: &Env) -> (u64, u32) {
        let timestamp = env.ledger().timestamp();
        let total_days = timestamp / (24 * 60 * 60);
        let year = 1970 + (total_days / 365);
        let days_in_year = total_days % 365;
        let month = 1 + (days_in_year / 30).min(11);
        (year, month as u32)
    }

    /// Helper function to get current year (using ledger timestamp)
    fn get_current_year(env: &Env) -> u64 {
        let (year, _) = Self::get_current_year_month(env);
        year
    }

    /// Helper function to get current month (using ledger timestamp)
    fn get_current_month(env: &Env) -> u32 {
        let (_, month) = Self::get_current_year_month(env);
        month
    }

    /// Get current year (public accessor)
    pub fn get_year(env: Env) -> u64 {
        Self::get_current_year(&env)
    }

    /// Get current month (public accessor)
    pub fn get_month(env: Env) -> u32 {
        Self::get_current_month(&env)
    }
}

mod test;
