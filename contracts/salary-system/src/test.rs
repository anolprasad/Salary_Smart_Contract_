#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token, Address, Env, String,
};

fn create_token_contract<'a>(env: &Env, admin: &Address) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let token_address = env.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(env, &token_address.address()),
        token::StellarAssetClient::new(env, &token_address.address()),
    )
}

fn advance_months(env: &Env, months: u32) {
    let seconds_per_month = 30 * 24 * 60 * 60;
    env.ledger().set(LedgerInfo {
        timestamp: env.ledger().timestamp() + (months as u64 * seconds_per_month),
        protocol_version: 20,
        sequence_number: env.ledger().sequence(),
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 10,
        min_persistent_entry_ttl: 10,
        max_entry_ttl: 3110400,
    });
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, _) = create_token_contract(&env, &token_admin);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &5, &token_client.address);

    assert_eq!(client.get_annual_increment(), 5);
    assert_eq!(client.get_treasury_balance(), 0);
}

#[test]
fn test_fund_treasury() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let funder = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);

    token_admin_client.mint(&funder, &10_000_000);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &5, &token_client.address);
    client.fund_treasury(&funder, &5_000_000);

    assert_eq!(client.get_treasury_balance(), 5_000_000);
}

#[test]
fn test_add_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, _) = create_token_contract(&env, &token_admin);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &5, &token_client.address);

    let employee_name = String::from_str(&env, "Alice");
    client.add_employee(&employee1, &employee_name, &1_200_000);

    let employee_data = client.get_employee(&employee1);
    assert_eq!(employee_data.name, employee_name);
    assert_eq!(employee_data.base_salary, 1_200_000);

    let all_employees = client.get_all_employees();
    assert_eq!(all_employees.len(), 1);
}

#[test]
fn test_monthly_salary_calculation() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, _) = create_token_contract(&env, &token_admin);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &10, &token_client.address);

    let employee_name = String::from_str(&env, "Bob");
    client.add_employee(&employee1, &employee_name, &1_200_000);

    let monthly_salary = client.calculate_current_salary(&employee1);
    assert_eq!(monthly_salary, 100_000);
}

#[test]
fn test_pay_monthly_salary() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let funder = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);

    token_admin_client.mint(&funder, &10_000_000);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &5, &token_client.address);
    client.fund_treasury(&funder, &5_000_000);

    let employee_name = String::from_str(&env, "Charlie");
    client.add_employee(&employee1, &employee_name, &1_200_000);

    client.pay_salary(&employee1);

    assert_eq!(token_client.balance(&employee1), 100_000);
    assert_eq!(client.get_treasury_balance(), 4_900_000);
}

#[test]
fn test_pay_multiple_months() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let funder = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);

    token_admin_client.mint(&funder, &10_000_000);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &5, &token_client.address);
    client.fund_treasury(&funder, &5_000_000);

    let employee_name = String::from_str(&env, "Dave");
    client.add_employee(&employee1, &employee_name, &1_200_000);

    client.pay_salary(&employee1);
    assert_eq!(token_client.balance(&employee1), 100_000);

    advance_months(&env, 1);

    client.pay_salary(&employee1);
    assert_eq!(token_client.balance(&employee1), 200_000);

    advance_months(&env, 1);

    client.pay_salary(&employee1);
    assert_eq!(token_client.balance(&employee1), 300_000);
}

#[test]
fn test_sustainability_years_calculation() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let employee2 = Address::generate(&env);
    let funder = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);

    token_admin_client.mint(&funder, &50_000_000);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &10, &token_client.address);
    client.fund_treasury(&funder, &10_000_000);

    client.add_employee(&employee1, &String::from_str(&env, "Alice"), &1_200_000);
    client.add_employee(&employee2, &String::from_str(&env, "Bob"), &1_800_000);

    let years = client.calculate_sustainability_years();
    assert!(years >= 2);
    assert!(years <= 5);
}

#[test]
#[should_panic(expected = "Insufficient treasury balance")]
fn test_insufficient_treasury() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let funder = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);

    token_admin_client.mint(&funder, &1_000_000);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &5, &token_client.address);
    client.fund_treasury(&funder, &50_000);

    client.add_employee(&employee1, &String::from_str(&env, "Alice"), &1_200_000);

    client.pay_salary(&employee1);
}

#[test]
#[should_panic(expected = "Salary already paid for this month")]
fn test_double_payment_same_month() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let funder = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);

    token_admin_client.mint(&funder, &10_000_000);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &5, &token_client.address);
    client.fund_treasury(&funder, &5_000_000);

    client.add_employee(&employee1, &String::from_str(&env, "Alice"), &1_200_000);

    client.pay_salary(&employee1);
    client.pay_salary(&employee1);
}

#[test]
fn test_employee_claim_salary() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let funder = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);

    // Mint tokens to funder
    token_admin_client.mint(&funder, &10_000_000);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    // Initialize contract
    client.initialize(&admin, &5, &token_client.address);
    
    // Fund treasury
    client.fund_treasury(&funder, &5_000_000);
    assert_eq!(client.get_treasury_balance(), 5_000_000);

    // Add employee with 1,200,000 stroops/month base salary
    client.add_employee(&employee1, &String::from_str(&env, "Alice"), &1_200_000);

    // Employee claims their own salary
    client.claim_salary(&employee1);

    // Verify salary was paid
    assert_eq!(token_client.balance(&employee1), 100_000); // 1,200,000 / 12 = 100,000 monthly
    assert_eq!(client.get_treasury_balance(), 4_900_000); // 5,000,000 - 100,000

    // Verify employee record was updated
    let employee_info = client.get_employee(&employee1);
    assert_eq!(employee_info.last_payment_month, 1);
}

#[test]
#[should_panic(expected = "Salary already paid for this month")]
fn test_employee_cannot_claim_twice_same_month() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let funder = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let (token_client, token_admin_client) = create_token_contract(&env, &token_admin);

    token_admin_client.mint(&funder, &10_000_000);

    let contract_id = env.register(SalarySystemContract, ());
    let client = SalarySystemContractClient::new(&env, &contract_id);

    client.initialize(&admin, &5, &token_client.address);
    client.fund_treasury(&funder, &5_000_000);
    client.add_employee(&employee1, &String::from_str(&env, "Alice"), &1_200_000);

    // First claim succeeds
    client.claim_salary(&employee1);
    
    // Second claim in same month should fail
    client.claim_salary(&employee1);
}
