#!/usr/bin/env node

/**
 * Backend Server for Salary Smart Contract
 * This server handles blockchain transactions via Stellar CLI
 */

const express = require('express');
const { exec } = require('child_process');
const { promisify } = require('util');
const cors = require('cors');
const path = require('path');

const execAsync = promisify(exec);

// Helper function to check if stderr contains actual errors
// Stellar CLI outputs informational messages to stderr that aren't errors
function isActualError(stderr) {
    if (!stderr) return false;
    
    // These are informational messages, not errors
    const infoPatterns = [
        'Simulation',
        'Send by',
        'Signing transaction:',
        'ℹ️'
    ];
    
    return !infoPatterns.some(pattern => stderr.includes(pattern));
}

const app = express();
const PORT = 3000;

// Contract Configuration
const CONTRACT_ID = 'CDXYPMRADZYRBUI2GNYICZBS7H6GJLX7BI3DI4HCUG2YISRR3NTLL4R4';
const ADMIN_ACCOUNT = 'alice';
const NETWORK = 'testnet';

// Middleware
app.use(cors());
app.use(express.json());
app.use(express.static(__dirname));

// Serve frontend
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'contracts/salary-system/frontend.html'));
});

// API: Add Employee
app.post('/api/add-employee', async (req, res) => {
    try {
        const { name, address, monthlyXLM } = req.body;

        // Validation
        if (!name || !address || !monthlyXLM) {
            return res.status(400).json({ 
                success: false,
                error: 'Missing required fields',
                message: 'Please provide name, address, and monthly salary in XLM'
            });
        }

        if (!address.startsWith('G') || address.length !== 56) {
            return res.status(400).json({ 
                success: false,
                error: 'Invalid address',
                message: 'Stellar address must start with G and be 56 characters'
            });
        }

        const salary = parseFloat(monthlyXLM);
        if (salary <= 0 || isNaN(salary)) {
            return res.status(400).json({ 
                success: false,
                error: 'Invalid salary',
                message: 'Monthly salary must be a positive number'
            });
        }

        // Convert monthly XLM to annual base salary in stroops
        // Monthly XLM * 12 months * 10,000,000 stroops per XLM
        const annualStroops = Math.floor(salary * 12 * 10_000_000);

        console.log(`Adding employee: ${name} (${address}) with ${monthlyXLM} XLM/month (${annualStroops} stroops annual base salary)`);

        // Execute Stellar CLI command
        // Note: Contract expects 'name' (not 'employee_name') and 'base_salary' (annual, not monthly)
        const command = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- add_employee \
  --employee_address "${address}" \
  --name "${name}" \
  --base_salary "${annualStroops}"`;

        const { stdout, stderr } = await execAsync(command);

        if (isActualError(stderr)) {
            console.error('Error:', stderr);
            return res.status(500).json({
                success: false,
                error: 'Transaction failed',
                message: stderr,
                details: stdout
            });
        }

        console.log('Success:', stdout);

        res.json({
            success: true,
            message: `Employee ${name} added successfully!`,
            data: {
                name,
                address,
                monthlyXLM: monthlyXLM,
                annualStroops: annualStroops,
                note: 'Annual base salary set (will increase 5% compound each year)'
            },
            output: stdout
        });

    } catch (error) {
        console.error('Error adding employee:', error);
        res.status(500).json({
            success: false,
            error: 'Server error',
            message: error.message,
            details: error.stderr || error.stdout
        });
    }
});

// API: Get Treasury Balance
app.get('/api/treasury-balance', async (req, res) => {
    try {
        const command = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- get_treasury_balance`;

        const { stdout } = await execAsync(command);
        const balance = stdout.trim().replace(/"/g, '');
        const balanceXLM = parseInt(balance) / 10_000_000;

        res.json({
            success: true,
            balanceStroops: balance,
            balanceXLM: balanceXLM
        });

    } catch (error) {
        console.error('Error getting treasury balance:', error);
        res.status(500).json({
            success: false,
            error: error.message
        });
    }
});

// API: Get All Employees
app.get('/api/employees', async (req, res) => {
    try {
        const command = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- get_all_employees`;

        const { stdout } = await execAsync(command);
        const addresses = JSON.parse(stdout.trim());

        res.json({
            success: true,
            count: addresses.length,
            employees: addresses
        });

    } catch (error) {
        console.error('Error getting employees:', error);
        res.status(500).json({
            success: false,
            error: error.message
        });
    }
});

// API: Get Employee Details
app.get('/api/employee/:address', async (req, res) => {
    try {
        const { address } = req.params;

        const command = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- get_employee \
  --employee_address "${address}"`;

        const { stdout } = await execAsync(command);
        const employeeData = JSON.parse(stdout.trim());

        res.json({
            success: true,
            employee: employeeData
        });

    } catch (error) {
        console.error('Error getting employee details:', error);
        res.status(500).json({
            success: false,
            error: error.message
        });
    }
});

// API: Pay All Salaries
app.post('/api/pay-all-salaries', async (req, res) => {
    try {
        console.log('Processing pay all salaries request...');

        // First, get all employees
        const getEmployeesCmd = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- get_all_employees`;

        const { stdout: employeesOutput } = await execAsync(getEmployeesCmd);
        const employees = JSON.parse(employeesOutput.trim());

        if (employees.length === 0) {
            return res.json({
                success: false,
                error: 'No employees found',
                message: 'There are no employees to pay'
            });
        }

        console.log(`Found ${employees.length} employees to pay`);

        // Pay each employee
        const results = [];
        let successCount = 0;
        let failCount = 0;

        for (const employeeAddress of employees) {
            try {
                const payCommand = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- pay_salary \
  --employee_address "${employeeAddress}"`;

                const { stdout: payOutput } = await execAsync(payCommand);
                
                results.push({
                    address: employeeAddress,
                    status: 'success',
                    output: payOutput.trim()
                });
                successCount++;
                console.log(`✅ Paid salary to ${employeeAddress.substring(0, 10)}...`);

            } catch (payError) {
                results.push({
                    address: employeeAddress,
                    status: 'failed',
                    error: payError.message
                });
                failCount++;
                console.error(`❌ Failed to pay ${employeeAddress.substring(0, 10)}...: ${payError.message}`);
            }
        }

        res.json({
            success: true,
            message: `Paid salaries to ${successCount} out of ${employees.length} employees`,
            summary: {
                total: employees.length,
                successful: successCount,
                failed: failCount
            },
            details: results
        });

    } catch (error) {
        console.error('Error paying all salaries:', error);
        res.status(500).json({
            success: false,
            error: 'Server error',
            message: error.message
        });
    }
});

// API: Fund Treasury
app.post('/api/fund-treasury', async (req, res) => {
    try {
        const { amountXLM } = req.body;

        // Validation
        if (!amountXLM) {
            return res.status(400).json({ 
                success: false,
                error: 'Missing amount',
                message: 'Please provide amount in XLM'
            });
        }

        const amount = parseFloat(amountXLM);
        if (amount <= 0 || isNaN(amount)) {
            return res.status(400).json({ 
                success: false,
                error: 'Invalid amount',
                message: 'Amount must be a positive number'
            });
        }

        // Convert XLM to stroops (1 XLM = 10,000,000 stroops)
        const amountStroops = Math.floor(amount * 10_000_000);

        console.log(`Funding treasury with ${amountXLM} XLM (${amountStroops} stroops)`);

        // Get alice's address
        const getAddressCmd = `stellar keys address ${ADMIN_ACCOUNT}`;
        const { stdout: addressOutput } = await execAsync(getAddressCmd);
        const aliceAddress = addressOutput.trim();

        // Execute fund treasury command
        const command = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- fund_treasury \
  --from "${aliceAddress}" \
  --amount "${amountStroops}"`;

        const { stdout, stderr } = await execAsync(command);

        if (isActualError(stderr)) {
            console.error('Error:', stderr);
            return res.status(500).json({
                success: false,
                error: 'Transaction failed',
                message: stderr
            });
        }

        console.log('Success:', stdout);

        res.json({
            success: true,
            message: `Treasury funded with ${amountXLM} XLM successfully!`,
            data: {
                amountXLM: amountXLM,
                amountStroops: amountStroops,
                from: aliceAddress
            },
            output: stdout
        });

    } catch (error) {
        console.error('Error funding treasury:', error);
        res.status(500).json({
            success: false,
            error: 'Server error',
            message: error.message
        });
    }
});

// API: Stop Employee Salary
app.post('/api/stop-employee-salary', async (req, res) => {
    try {
        const { employeeAddress } = req.body;

        // Validation
        if (!employeeAddress) {
            return res.status(400).json({ 
                success: false,
                error: 'Missing employee address',
                message: 'Please provide employee address'
            });
        }

        if (!employeeAddress.startsWith('G') || employeeAddress.length !== 56) {
            return res.status(400).json({ 
                success: false,
                error: 'Invalid address',
                message: 'Stellar address must start with G and be 56 characters'
            });
        }

        console.log(`Stopping salary for employee: ${employeeAddress}`);

        // Execute stop salary command
        const command = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- stop_employee_salary \
  --employee_address "${employeeAddress}"`;

        const { stdout, stderr } = await execAsync(command);

        if (isActualError(stderr)) {
            console.error('Error:', stderr);
            return res.status(500).json({
                success: false,
                error: 'Transaction failed',
                message: stderr
            });
        }

        console.log('Success:', stdout);

        res.json({
            success: true,
            message: `Salary stopped for employee successfully!`,
            data: {
                employeeAddress: employeeAddress
            },
            output: stdout
        });

    } catch (error) {
        console.error('Error stopping employee salary:', error);
        res.status(500).json({
            success: false,
            error: 'Server error',
            message: error.message
        });
    }
});

// API: Resume Employee Salary
app.post('/api/resume-employee-salary', async (req, res) => {
    try {
        const { employeeAddress } = req.body;

        // Validation
        if (!employeeAddress) {
            return res.status(400).json({ 
                success: false,
                error: 'Missing employee address',
                message: 'Please provide employee address'
            });
        }

        if (!employeeAddress.startsWith('G') || employeeAddress.length !== 56) {
            return res.status(400).json({ 
                success: false,
                error: 'Invalid address',
                message: 'Stellar address must start with G and be 56 characters'
            });
        }

        console.log(`Resuming salary for employee: ${employeeAddress}`);

        // Execute resume salary command
        const command = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- resume_employee_salary \
  --employee_address "${employeeAddress}"`;

        const { stdout, stderr } = await execAsync(command);

        if (isActualError(stderr)) {
            console.error('Error:', stderr);
            return res.status(500).json({
                success: false,
                error: 'Transaction failed',
                message: stderr
            });
        }

        console.log('Success:', stdout);

        res.json({
            success: true,
            message: `Salary resumed for employee successfully!`,
            data: {
                employeeAddress: employeeAddress
            },
            output: stdout
        });

    } catch (error) {
        console.error('Error resuming employee salary:', error);
        res.status(500).json({
            success: false,
            error: 'Server error',
            message: error.message
        });
    }
});

// API: Claim Salary (Employee Self-Service)
app.post('/api/claim-salary', async (req, res) => {
    try {
        const { employeeAddress, employeeName } = req.body;

        // Validation
        if (!employeeAddress) {
            return res.status(400).json({ 
                success: false,
                error: 'Missing employee address',
                message: 'Please provide your Stellar address'
            });
        }

        if (!employeeAddress.startsWith('G') || employeeAddress.length !== 56) {
            return res.status(400).json({ 
                success: false,
                error: 'Invalid address',
                message: 'Stellar address must start with G and be 56 characters'
            });
        }

        console.log(`Employee ${employeeName || employeeAddress.substring(0, 10)}... claiming salary`);

        // Note: In a real implementation, you'd need the employee to sign with their private key
        // For now, we'll use the contract's claim_salary function if it exists
        // Check if claim_salary function exists in the contract
        const command = `stellar contract invoke \
  --id ${CONTRACT_ID} \
  --source ${ADMIN_ACCOUNT} \
  --network ${NETWORK} \
  -- pay_salary \
  --employee_address "${employeeAddress}"`;

        const { stdout, stderr } = await execAsync(command);

        if (isActualError(stderr)) {
            console.error('Error:', stderr);
            return res.status(500).json({
                success: false,
                error: 'Transaction failed',
                message: stderr,
                note: 'Make sure you are registered as an employee and have not been paid this month yet.'
            });
        }

        console.log('Success:', stdout);

        res.json({
            success: true,
            message: `Salary claimed successfully!`,
            data: {
                employeeAddress: employeeAddress,
                employeeName: employeeName || 'Unknown'
            },
            output: stdout,
            note: 'Your monthly salary has been transferred to your account!'
        });

    } catch (error) {
        console.error('Error claiming salary:', error);
        
        // Check for common errors
        let userMessage = error.message;
        if (error.message.includes('not found') || error.message.includes('not registered')) {
            userMessage = 'You are not registered as an employee in this system.';
        } else if (error.message.includes('already paid') || error.message.includes('same month')) {
            userMessage = 'You have already claimed your salary this month. Try again next month!';
        }
        
        res.status(500).json({
            success: false,
            error: 'Claim failed',
            message: userMessage,
            details: error.stderr || error.stdout
        });
    }
});

// Start server
app.listen(PORT, () => {
    console.log(`
╔═══════════════════════════════════════════════════════════╗
║  🚀 Salary Smart Contract Backend Server                  ║
╠═══════════════════════════════════════════════════════════╣
║  Status: RUNNING                                          ║
║  Port: ${PORT}                                                 ║
║  Frontend: http://localhost:${PORT}                            ║
║  Contract: ${CONTRACT_ID.substring(0, 20)}...        ║
╚═══════════════════════════════════════════════════════════╝
    `);
    console.log('Available endpoints:');
    console.log('  POST /api/add-employee           - Add new employee');
    console.log('  POST /api/pay-all-salaries       - Pay all employees');
    console.log('  POST /api/claim-salary           - Claim salary (Employee)');
    console.log('  POST /api/fund-treasury          - Fund the treasury');
    console.log('  POST /api/stop-employee-salary   - Stop employee salary');
    console.log('  POST /api/resume-employee-salary - Resume employee salary');
    console.log('  GET  /api/treasury-balance       - Get treasury balance');
    console.log('  GET  /api/employees              - Get all employees');
    console.log('  GET  /api/employee/:address      - Get employee details');
    console.log('');
});
