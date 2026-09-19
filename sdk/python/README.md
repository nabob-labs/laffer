# velox-python-sdk

Python SDK for [Velox](https://velox.zone) — a perpetual futures DEX. Two import paths in one package:

- `velox.*` — the native API. Snake_case wire shapes, signed sizes, Velox's actual contract message types.
- `velox.hyperliquid_compatibility.*` — HL-shaped wrapper. Drop-in for [`hyperliquid-python-sdk`](https://github.com/hyperliquid-dex/hyperliquid-python-sdk) users with import-line-only changes.

## Installation

```plain
uv add velox-python-sdk
# or
pip install velox-python-sdk
```

Requires Python 3.14+.

## Quick start: native

```python
import example_utils

from velox.utils.constants import TESTNET_API_URL
from velox.utils.types import Addr, OrderId, PairId

address, info, exchange = example_utils.setup_native(
    base_url=TESTNET_API_URL,
    skip_ws=True,
)

# Place a resting limit buy of 0.2 ETH at $1100.
result = exchange.submit_limit_order(
    PairId("perp/ethusd"),
    size="0.2",
    limit_price="1100",
)
print(result)

# Cancel by chain order id.
open_orders = info.orders_by_user(Addr(address))
if open_orders:
    oid = next(iter(open_orders))
    exchange.cancel_order(OrderId(oid))
```

## Quick start: HL-compat

```python
import example_utils

from velox.hyperliquid_compatibility import constants

address, info, exchange = example_utils.setup(
    base_url=constants.TESTNET_API_URL,
    skip_ws=True,
)

# HL's `order(name, is_buy, sz, limit_px, order_type)` signature.
order_result = exchange.order("ETH", True, 0.2, 1100, {"limit": {"tif": "Gtc"}})
print(order_result)

if order_result["status"] == "ok":
    status = order_result["response"]["data"]["statuses"][0]
    if "resting" in status:
        exchange.cancel("ETH", status["resting"]["oid"])
```

The HL-compat module's call surfaces — `info.user_state`, `info.l2_snapshot`, `exchange.order`, `exchange.cancel`, `exchange.market_open`, `exchange.market_close`, `exchange.subscribe`, etc. — match upstream HL signature-for-signature, including the wire camelCase response shapes (`assetPositions`, `marginSummary`, etc.).

## Examples

See `examples/` for runnable scripts:

- `native_basic_order.py` — place + query + cancel a resting limit order via the native API.
- `native_basic_ws.py` — subscribe to perps trades, candles, user events, and blocks.
- `native_market_order.py` — market open and reduce-only close.
- `hl_basic_order.py` — verbatim port of HL's `basic_order.py` (only the imports differ).
- `hl_basic_ws.py` — verbatim port of HL's `basic_ws.py` (only the imports differ).

Install the example deps and copy the env template:

```plain
uv sync --group examples
cp examples/.env.example examples/.env
```

Then fill in `examples/.env` with your `VELOX_SECRET_KEY` (or `VELOX_KEYSTORE_PATH`) and `VELOX_ACCOUNT_ADDRESS` before running. `VELOX_ACCOUNT_ADDRESS` is required — Velox decouples the signing key from the on-chain account, so the SDK does not auto-derive it. The `.env` file is gitignored at the repo root and access is denied to Claude in `.claude/settings.json`.

## Where Velox differs from HL

The HL-compat layer is high fidelity but not a perfect superset. Concrete divergences:

- **Perps only.** Velox has no spot product; spot-related calls (`spot_user_state`, `spot_meta`, spot subscriptions) raise `NotImplementedError`.
- **Cross-margin only.** No isolated-margin per-asset accounts. `crossMarginSummary` mirrors the global margin; isolated-margin fields are zeroed.
- **No funding history series.** `funding_history`, `user_funding_history`, and the `userFundings` subscription raise `NotImplementedError`. Per-user realized funding is in the perps events stream.
- **No builder fee marketplace.** The `builder=` argument on `order` raises if non-`None`.
- **No HL-style sub-accounts or vault accounts.** `create_sub_account`, `sub_account_transfer`, and `vault_address` raise.
- **Account address required.** The HL-compat `Exchange` constructor requires `account_address` explicitly — Velox decouples the signing key from the on-chain account, so silent EVM-address derivation would be wrong.
- **Cloid asymmetry.** HL's 16-byte `Cloid` is hashed (deterministic SHA-256 prefix) to a `Uint64` for Velox. Responses surface the `Uint64`, not the original 16-byte cloid. Round-trip identity is not preserved without your own mapping.
- **`set_expires_after`** is recorded but not yet threaded through the native sign path; expiry is enforced contract-side via TIF / conditional-order semantics for now.
- **Withdraw / deposit / agent** primitives that HL exposes as signed actions (`withdraw_from_bridge`, `usd_transfer`, `approve_agent`) raise `NotImplementedError`; planned for v0.2.

Methods that have no Velox equivalent (e.g. `update_leverage`, `schedule_cancel`, all spot-side methods, `extra_agents`, `portfolio`, builder fees) raise `NotImplementedError` with a one-line reason instead of silently no-op'ing — discover the gaps loudly. See the docstrings on each `NotImplementedError` for the full list.
