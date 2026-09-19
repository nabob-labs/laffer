# About Velox

## What is Velox?

Velox is a DeFi-native Layer 1 blockchain built from the ground up for trading. Where most blockchains are general-purpose platforms that happen to host DeFi apps, Velox inverts this: the chain is purpose-built around a DEX, with every infrastructure decision made to serve traders.

### Problems Velox Solves

#### 1. Capital Inefficiency

On today's platforms, collateral is siloed. A trader on Aave must deposit separately from their dYdX position, their Uniswap LP, and so on. Velox's Unified Margin Account lets a single pool of collateral back spot trades, perpetual positions, and lending simultaneously.

#### 2. Execution Quality & MEV

AMMs suffer from slippage and impermanent loss by design. Orders are also vulnerable to MEV — bots that front-run transactions for profit at the user's expense. Velox's on-chain Central Limit Order Book (CLOB) with periodic batch auctions eliminates both problems.

#### 3. Terrible UX

DeFi onboarding is notoriously difficult: manage private keys, pay gas in native tokens, bridge assets across chains, juggle multiple wallets. Velox introduces Smart Accounts — a keyless system where accounts are secured by passkeys (biometrics) instead of seed phrases.

#### 4. Developer Inflexibility

EVM and Cosmos SDK give developers limited control over gas mechanics, scheduling, and account logic. Velox's Bolt execution environment gives developers programmable gas fees, on-chain cron jobs, and customizable account logic — without hard forks.

### What Makes Velox Different

Most chains compete on speed (TPS). Velox competes on product design — specifically by building its own execution environment (Bolt) co-designed with the application layer. This "app-driven infra development" enables features impossible or prohibitively expensive on EVM chains:

* On-chain CLOB with sub-second batch settlement
* Protocol-native cron jobs for automatic funding rate calculation
* Smart account architecture enabling biometric signing
* Zero gas fees
* Unified cross-collateral margin for all trading products