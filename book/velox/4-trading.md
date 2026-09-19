# Trading

### Perpetual Futures

Perpetual futures are leveraged derivative contracts with no expiry date. They track the spot price of an asset via a funding rate mechanism — periodic payments between longs and shorts that keep the perp price anchored to spot.

Velox's perps are:

* Fully on-chain — matching through the same CLOB as spot
* Cross-margined — spot assets back perp positions via unified margin
* Keeper-free — funding rates computed and applied automatically via on-chain cron jobs

#### Funding Rates

* Calculated by on-chain cron jobs, no external keeper required
* Triggered by oracle price feeds (e.g., Chainlink)
* Long pays short when perp trades above spot; short pays long when below
* Converges perp price toward spot organically

### CLOB vs AMM

| Feature           | AMM (e.g. Uniswap)           | CLOB (Velox)                   |
| ----------------- | ---------------------------- | ------------------------------ |
| Price discovery   | Formula-based (x·y = k)      | Order book, bid/ask            |
| Slippage          | Always (size-dependent)      | Minimal (fills at limit price) |
| MEV exposure      | High                         | Low (batch auctions)           |
| Impermanent loss  | Yes (LPs)                    | No                             |
| Transparency      | Contracts on-chain           | Full order book on-chain       |
| Order types       | Swaps only                   | Market, limit, stop            |
| Execution latency | 1 block per tx               | 0.2–0.5s batch                 |

Velox's trading mechanics are closer to a centralized exchange than to Uniswap.

#### Fees

Fees are based on your rolling 14-day perpetual trading volume.

| Tier | 14-day volume | Maker  | Taker   |
| ---- | ------------- | ------ | ------- |
| 1    | `0 - $100k`     | `0.010%` | `0.038%`  |
| 2    | `$100k - $1m`   | `0.008%` | `0.032%`  |
| 3    | `$1m - $10m`    | `0.006%` | `0.026%`  |
| 4    | `$10m - $50m`   | `0.004%` | `0.020%`  |
| 5    | `$50m - $200m`  | `0.002%` | `0.016%`  |
| 6    | `$200m+`        | `0.00%`  | `0.014%`  |
