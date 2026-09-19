# Accounts

## Smart Accounts & User Experience

Velox's account system is one of its most differentiated features and is fundamental to the "one app for everything DeFi" vision. It is built on top of Bolt's native account abstraction capabilities.

### The Problem with Traditional Crypto UX

Standard crypto wallets require users to:

* Generate and store a 12- or 24-word seed phrase
* Never lose or expose it (lost = funds gone forever)
* Pay gas in the chain's native token before they can transact
* Manage multiple wallets across different chains
* Install browser extensions (MetaMask) to interact with dApps

These requirements create enormous friction for new users and security risks for all users.

### Smart Accounts

Velox's Smart Accounts are programmable accounts that replace the standard private-key model with a flexible authentication layer powered by Bolt.

#### Key Properties

1. Keyless / Wallet-less

No browser wallet extension or seed phrase required. Account access is secured by:

* Passkeys — a FIDO2 standard that stores a cryptographic key in your device's secure enclave (Touch ID, Face ID, Windows Hello)
* Multi-factor authentication — optional additional factors
* Biometric verification — on mobile, the phone's biometrics authenticate transactions

The private key is generated and stored in the device's secure enclave, never exposed to the user or the application.

2. Cloud-Synced Access

Because the key lives in the device's secure enclave / cloud keychain (iCloud, Google), your Velox account is accessible from any of your devices without manually importing a seed phrase.

3. Native Username

Each Smart Account has a human-readable username (e.g. @alice) registered on-chain. This replaces hex addresses for transfers and creates a persistent on-chain identity.

4. Programmable Logic

Developers can deploy custom authentication logic to Smart Accounts:

- Spending limits per session
- Time-locked transactions
- Multi-sig approval requirements
- Social recovery mechanisms

### Subaccounts

Every Velox account supports subaccounts — child accounts that inherit from the master account's margin pool but can be managed independently.

| Use Case               | How Subaccounts Help                                                    |
| ---------------------- | ------------------------------------------------------------------------|
| Portfolio separation   | Keep long-term holdings separate from active trading                    |
| Risk isolation         | Cap losses on a specific strategy without affecting other positions     |
| API / bot trading      | Grant a subaccount limited permissions to an automated strategy         |
| Team trading           | Multiple operators accessing a shared capital pool with defined limits  |

Subaccounts share collateral with the master account (unified margin), or can be isolated with dedicated collateral.

### Gas Abstraction

```table
All fees denominated in USDC
↓
Taxman contract collects fees automatically from trading balance
↓
USDC used to buy DNG on the open market
↓
DNG permanently burned (deflationary)
```

Developers can further customize gas logic:

* Sponsor gas on behalf of users (users pay nothing at all)
* Apply discounts to specific user groups (token holders, NFT holders, etc.)
* Implement dynamic fee tiers based on volume

### The Taxman Contract

The Taxman is a system-level smart contract governing all fee and gas logic on Velox. Instead of gas rules being hardcoded in the protocol (as on Ethereum), Taxman allows:

| Feature                     | Description                                                             |
| --------------------------- | ----------------------------------------------------------------------- |
| Customizable fee structures | Different dApps can have different fee models                           |
| Discount programming        | Rewards users algorithmically, no protocol changes needed               |
| Upgradeable                 | Fee logic updates are contract upgrades, not consensus-layer hard forks |

### Comparison: Traditional Wallet vs Velox Smart Account

| Feature         | MetaMask / Traditional          | Velox Smart Account              |
| --------------- | ------------------------------- | -------------------------------- |
| Onboarding      | 24-word seed phrase             | Passkey / biometric              |
| Gas token       | Must hold native token          | Gasless UX                       |
| Address format  | 0x4a2b...f91c                   | @username                        |
| Device sync     | Manual import                   | Cloud-synced automatically       |
| Recovery        | Seed phrase (lose = lose funds) | Cloud keychain / social recovery |
| Account logic   | Fixed                           | Programmable                     |
| Multi-device    | Manual                          | Automatic                        |
