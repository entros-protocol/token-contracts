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
| Validator staking | `protocol-core/entros-registry` | SOL-denominated today. Token-denominated staking is planned |
| Delegation | `protocol-core/entros-registry` | Not built |
| Governance voting weight | `entros-governance-plugin` | Deployed on devnet |
| Founder and treasury lockups | Streamflow | At launch |
| Integrator capacity tiers | reserved for this repository | Not built |
| Insurance pool | reserved for this repository | Not built |

Token-denominated validator staking extends `entros-registry`. That registry already holds
`ValidatorState.stake` and checks it against the minimum required to join the Anonymity Ring,
so keeping the stake and the eligibility check in one program keeps one source of truth for
validator admission.

**The token has no on-chain coupling to verification today.** Verification runs on devnet with
SOL-denominated fees and SOL validator stake. Each mechanism above activates in phases as the
validator network decentralizes, after the core-protocol audit.

## What is fixed at mint creation

Mint extensions and mint authority are set when a mint is created and cannot be added later.
For $ENTROS this means:

- **No transfer hook.** The protocol cannot tax transfers or restrict who receives the token.
- **No Token-2022 extensions.** No confidential balances, no non-transferability.
- **No further minting.** The mint carries no mint authority, so supply is fixed.

The third point shapes the reward model. Rewards come from protocol revenue, never from
emissions, so validator returns track verification volume.

## Not the Entros Anchor

The **Entros Anchor** is a separate mint in `protocol-core`. It uses Token-2022 with the
NonTransferable extension and acts as a soulbound identity credential. One Anchor per verified
person, and it cannot be sold or moved.

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
