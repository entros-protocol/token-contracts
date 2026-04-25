# token-contracts

The Entros utility token. An SPL Token-2022 mint on Solana with the Confidential Balances extension, enabling private balance and transfer amounts while maintaining on-chain verifiability.

## Token

| Property | Value |
|----------|-------|
| Standard | SPL Token-2022 |
| Extension | Confidential Balances (ElGamal + Sigma proofs) |
| Decimals | 6 |
| Supply | Fixed at genesis |

Confidential Balances encrypt token amounts using twisted ElGamal encryption. The network verifies transfers via zero-knowledge range and equality proofs without learning the amounts. Balances remain private to the holder.

## Utility

**Validator staking.** Validators stake Entros tokens to join the Anonymity Ring and participate in verification attestations. Stake size determines selection weight in VRF-based validator assignment.

**Governance.** Entros token holders vote on protocol parameters: minimum stake thresholds, trust score weights, challenge expiry, fee structure. One token, one vote. Staked tokens carry governance weight.

## Distribution

| Allocation | Share | Vesting |
|------------|-------|---------|
| Community | 40% | Unlocked at genesis |
| Ecosystem grants | 20% | 12-month linear |
| Treasury | 15% | Protocol-controlled |
| Team | 15% | 24-month linear, 6-month cliff |
| Initial liquidity | 10% | Unlocked at genesis |

Launch mechanism: MetaDAO or curated community sale. The first airdrop goes to Entros-verified humans only.

## Revenue Model

Users pay ~0.005 SOL per verification as a protocol fee. Fees accumulate in an on-chain treasury PDA — transparent, auditable. Integrators read on-chain verification state for free. Revenue flows to the treasury, which buys Entros tokens on the open market and distributes them to active validators as staking rewards.

```
User pays ~0.005 SOL per verification
  → protocol treasury PDA collects fees
  → treasury buys Entros tokens on open market
  → validators earn staking rewards
  → better security → more integrations → more verifications
```

## Architecture

The token program integrates with two other Entros Protocol programs:

- **entros-registry** (`protocol-core`): Reads validator stake amounts to determine Anonymity Ring eligibility and VRF selection weight. The registry's `register_validator` instruction will accept Entros token stakes alongside SOL.
- **executor-node**: Reads validator stake amounts for validation node eligibility. Validators earn protocol fee revenue proportional to stake and performance.

```
token-contracts/
└── programs/
    └── entros-token/
        └── src/
            └── lib.rs    # Token mint, Confidential Balances, vesting
```

## Status

Phase 7. The program scaffold is in place. Implementation begins after the external security audit of the core protocol (Phase 6).

## Setup

```bash
# Prerequisites: Solana CLI >= 2.2, Anchor 0.32.1, Rust

anchor build          # Compile the program
anchor test           # Run integration tests
anchor deploy         # Deploy to localnet/devnet
```

## License

MIT
