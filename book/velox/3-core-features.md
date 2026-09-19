# Core Features

Velox's product is built around four application-layer pillars — Spot, Perpetuals, Vaults, and Lending — all accessible from a single unified account. This page covers the platform-level features underpinning all four products.

### Unified Margin System

The centerpiece of Velox's design is its Unified Margin Account: a single account that holds all assets and allows them to serve as collateral across every product simultaneously.

#### How It Works

Traditional DeFi requires separate deposits per protocol:

```table
dYdX perp  →  deposit on dYdX
Uniswap    →  separate wallet balance
Aave       →  separate deposit on Aave
```

On Velox, one deposit covers all of the above. ETH holdings, spot positions, and vault allocations all exist within one margin pool. The system dynamically calculates aggregate risk and adjusts borrowing capacity accordingly.

#### Example

A user deposits 1 ETH:

* Used as collateral to open a 5× leveraged BTC perp

* Backs a USDC loan for spot trading simultaneously

* Vault yield accrues on the idle portion

* One unified liquidation price governs all positions

### Gasless Trading

From the user's perspective, trading on Velox is completely gasless — no native token needed. All fees are denominated in USDC and collected automatically from the trading account.

### Native Usernames

Velox accounts support human-readable usernames instead of hex addresses:

* @alice instead of 0x4a2b...f91c
* Registered on-chain as part of the smart account system
* Accounts accumulate reputation and history attached to a persistent username

### Subaccounts

| Use Case               | How Subaccounts Help                                                    |
| ---------------------- | ------------------------------------------------------------------------|
| Portfolio separation   | Keep long-term holdings separate from active trading                    |
| Risk isolation         | Cap losses on a specific strategy without affecting other positions     |
| API / bot trading      | Grant a subaccount limited permissions to an automated strategy         |
| Team trading           | Multiple operators accessing a shared capital pool with defined limits  |

### Notifications

A native notification system alerts users to fills, liquidation risk, funding rate changes, and other events.

### Built-in Block Explorer

A native block explorer is integrated into the platform, giving traders visibility into transaction history, order fills, and on-chain state without leaving the exchange.

### Mobile App

Native mobile applications have been initially implemented. The passkey/biometric authentication system is particularly well-suited to mobile — logging in with Face ID or fingerprint rather than a 24-word seed phrase.