# token-contracts

$ENTROS is the Entros Protocol utility token. It is a standard SPL mint created by a public
launchpad, not by any program in this repository.

This repository holds no code. It maps where $ENTROS gets its utility and which repository
owns each part.

## Token Contract Address

```
fc7hPCHtpNezg7cAp6UoUksHU6Sy98hEGswf8SSEASY
```

[Launch page on EasyA Kickstart](https://kickstart.easya.io/token/fc7hPCHtpNezg7cAp6UoUksHU6Sy98hEGswf8SSEASY)

Standard SPL mint, 6 decimals, mint authority and freeze authority both revoked. Check it
against [Solscan](https://solscan.io/token/fc7hPCHtpNezg7cAp6UoUksHU6Sy98hEGswf8SSEASY)
before you transact.

## The mint is not the utility

A mint records balances. That is its whole job. It does not know the protocol exists.

Utility lives in programs that require the token to be deposited or locked. Such a program
owns a Program Derived Address, that address owns a token account, and the program records
who deposited what and decides when it returns. The mint never observes any of this. A stake,
a lock and a vesting release are all the same operation from the mint's side, which is a
balance moving between two accounts.

The protocol takes one thing from the launch: the mint address.

## Where each mechanism lives

| Mechanism | Repository | Status |
|-----------|-----------|--------|
| Validator registration | `protocol-core/entros-registry` | SOL deposit scaffolding deployed on devnet. Selection and rewards are not built |
| Delegation | `protocol-core/entros-registry` | Not built |
| Governance voting weight | `entros-governance-plugin` | On-chain addin deployed on devnet. Realms client integration is planned |
| Founder and treasury lockups | Streamflow | At launch |
| Integrator capacity tiers | reserved for this repository | Not built |
| Insurance pool | reserved for this repository | Not built |

Token-denominated validator staking is planned for `entros-registry`. The current registry
stores `ValidatorState.stake` and checks its minimum during registration. It does not assign
validation work, select a quorum, slash stake, or distribute rewards.

**The token has no on-chain coupling to verification today.** Verification runs on devnet with
SOL-denominated fees and devnet validator-registration scaffolding. Each planned mechanism
requires implementation, testing, and review before activation.

## What is fixed at mint creation

Mint extensions and mint authority are set when a mint is created and cannot be added later.
For $ENTROS this means:

- **No transfer hook.** The protocol cannot tax transfers or restrict who receives the token.
- **No Token-2022 extensions.** No confidential balances, no non-transferability.
- **No further minting.** The mint carries no mint authority, so supply is fixed.

The third point constrains the planned reward model. Any future rewards must come from
protocol revenue or an existing allocation because the mint cannot issue new supply.

## Not the Entros Anchor

The **Entros Anchor** is a separate mint in `protocol-core`. It uses Token-2022 with the
NonTransferable extension and acts as a wallet-bound protocol credential. The program derives
one Anchor PDA per wallet and prevents token transfer. Population uniqueness remains a
validator and research objective.

$ENTROS is the fungible utility token. Two different standards, two different purposes.

## Distribution

Entros launched through a public launchpad with no presale, no private round, and no VC
allocation. The team bought its tokens on the launch curve rather than receiving a grant, and
locks them through Streamflow on published schedules that anyone can inspect on-chain.

## Status

No code. Integrator capacity tiers and the insurance pool are the two mechanisms reserved
here, and neither is specified yet. Everything else in the table above lives in a repository
that already exists.

## License

MIT
