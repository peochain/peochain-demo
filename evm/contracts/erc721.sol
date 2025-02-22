// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * ----------------------------------------------------------------------------
 * PEOCHAIN-DEMO: ERC-721 EXAMPLE
 * ----------------------------------------------------------------------------
 * This contract demonstrates a simplified ERC-721 token.
 */

contract DemoNFT {
    string public name = "DemoNFT";
    string public symbol = "DNFT";

    mapping(uint256 => address) private _owners;

    function ownerOf(uint256 tokenId) public view returns (address) {
        return _owners[tokenId];
    }

    function mint(uint256 tokenId) public {
        require(_owners[tokenId] == address(0), "Token already exists.");
        _owners[tokenId] = msg.sender;
    }
}
