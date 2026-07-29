# token-contracts

The Entros utility token, a standard SPL mint on Solana.

Not to be confused with the **Entros Anchor** in `protocol-core`, which is a separate Token-2022 mint carrying the NonTransferable extension. The Anchor is a soulbound identity credential. This is the fungible utility token.

## Token

| Property | Value |
|----------|-------|
| Standard | SPL Token |
| Decimals | 6 |
| Supply | Fixed at genesis |

$ENTROS launches through a public bonding curve, which mints a standard SPL token. Balances and transfers are public, as they are for any token that trades on an AMM: a pool has to read balances to price a swap.

## Utility

None of the mechanisms below are live. Verification runs today on devnet with SOL-denominated
fees and SOL validator stake. The token has no on-chain coupling to verification. Each mechanism
activates in phases as the validator network decentralizes, after the core-protocol audit.

**Validator staking.** Validators stake Entros tokens as slashable collateral to join the
Anonymity Ring and take part in verification attestations. Rewards track validation accuracy
against ground-truth benchmarks rather than throughput, so passing borderline captures does not
increase yield.

**Delegation.** Holders who do not run a node delegate stake to a validator and share both the
accuracy-weighted rewards and the slashing risk.

**Capacity.** High-volume integrators stake for priority access, replacing per-verification fees
at scale.

**Economic governance.** Holders direct the protocol economy: treasury allocation, the
verification fee, validator admission policy, and ecosystem funding. Voting weight combines a
verified Entros Anchor with staked tokens under a lock multiplier.

Detection parameters are not governed by token vote. Trust Score weights, Hamming bounds, and
validation thresholds are set by calibration against measured data and red-team results, and
published as a changelog after they change.

## Distribution

Entros launches as a fair launch with no presale, no private round, and no VC allocation.
The team buys its tokens on the open market at launch rather than receiving a grant, and locks
them in public vesting contracts anyone can inspect.

## Revenue Model

Users pay ~0.005 SOL per verification as a protocol fee. Fees accumulate in an on-chain treasury
PDA, transparent and auditable. Integrators read on-chain verification state for free.

As the validator network decentralizes, a share of fees routes to validators in proportion to
validation accuracy, scored against ground-truth benchmarks rather than raw verification count.

```
User pays ~0.005 SOL per verification
  → protocol treasury PDA collects fees
  → validators earn a share weighted by validation accuracy
  → better security → more integrations → more verifications
```

## Architecture

The token program integrates with two other Entros Protocol programs:

- **entros-registry** (`protocol-core`): Reads validator stake amounts to determine Anonymity Ring eligibility and VRF selection weight. The registry's `register_validator` instruction will accept Entros token stakes alongside SOL.
- **executor-node**: Reads validator stake amounts for validation node eligibility. Validators earn in proportion to stake and validation accuracy — scored against ground-truth benchmarks — so passing borderline captures to lift throughput does not increase yield.

```
token-contracts/
└── programs/
    └── entros-token/
        └── src/
            └── lib.rs    # Token mint scaffold
```

## Status

This program is a scaffold. `initialize` is a stub and there is no mint, supply constant,
extension wiring, or vesting logic in it yet. The properties above describe the intended
configuration, not deployed code.

The $ENTROS token launches separately through a launchpad. Whether this program takes on a
protocol-wired role, or is retired in favour of the launchpad mint, is an open decision.

## Setup

```bash
# Prerequisites: Solana CLI >= 2.2, Anchor 0.32.1, Rust

anchor build          # Compile the program
anchor test           # Run integration tests
anchor deploy         # Deploy to localnet/devnet
```

## License

MIT
