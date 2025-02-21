// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: ERC-20 EXAMPLE
 * ----------------------------------------------------------------------------
 * This contract serves as a minimal ERC-20 token for demonstration.
 */

contract DemoToken {
    string public name = "DemoToken";
    string public symbol = "DMT";
    uint8 public decimals = 18;
    uint256 public totalSupply = 1000000 * (10 ** decimals);

    mapping(address => uint256) balances;

    constructor() {
        balances[msg.sender] = totalSupply;
    }

    function balanceOf(address account) public view returns (uint256) {
        return balances[account];
    }

    function transfer(address to, uint256 amount) public returns (bool) {
        require(balances[msg.sender] >= amount, "Insufficient balance.");
        balances[msg.sender] -= amount;
        balances[to] += amount;
        return true;
    }
}
