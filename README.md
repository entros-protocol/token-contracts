# token-contracts

$ENTROS is the Entros Protocol utility token. It is a standard SPL mint created by a public
launchpad, not by any program in this repository.

This repository holds no code. It exists to answer one question correctly: where does the
token get its utility, and which repository owns each part.

## The mint is not the utility

A mint records balances. That is its whole job. It does not know the protocol exists.

Utility lives in programs that require the token to be deposited or locked. Such a program
owns a Program Derived Address, that address owns a token account, and the program records
who deposited what and decides when it returns. The mint never observes any of this. A stake,
a lock and a vesting release are all the same operation from the mint's side, which is a
balance moving between two accounts.

So the launchpad creating the mint costs the protocol nothing it needs. The only thing the
protocol takes from the launch is the mint address.

## Where each mechanism lives

| Mechanism | Repository | Status |
|-----------|-----------|--------|
| Validator staking | `protocol-core/entros-registry` | SOL-denominated today. Token-denominated staking is planned |
| Delegation | `protocol-core/entros-registry` | Not built |
| Governance voting weight | `entros-governance-plugin` | Deployed on devnet |
| Founder and treasury lockups | Streamflow | At launch |
| Integrator capacity tiers | reserved for this repository | Not built |
| Insurance pool | reserved for this repository | Not built |

Token-denominated validator staking extends `entros-registry` rather than becoming a separate
program. That registry already holds `ValidatorState.stake` and checks it against the minimum
required to join the Anonymity Ring. Keeping the stake and the eligibility check in one
program keeps one source of truth for validator admission.

**The token has no on-chain coupling to verification today.** Verification runs on devnet with
SOL-denominated fees and SOL validator stake. Each mechanism above activates in phases as the
validator network decentralizes, after the core-protocol audit.

## What is fixed at mint creation

Mint extensions and mint authority are set when a mint is created and cannot be added later.
For $ENTROS this means:

- **No transfer hook.** The protocol cannot tax transfers or restrict who receives the token.
- **No Token-2022 extensions.** No confidential balances, no non-transferability.
- **No further minting**, once the launchpad revokes mint authority.

The third point shapes the reward model. Rewards come from protocol revenue, never from
emissions, so validator returns track real verification volume rather than an issuance
schedule.

## Not the Entros Anchor

The **Entros Anchor** is a separate mint in `protocol-core`. It uses Token-2022 with the
NonTransferable extension and acts as a soulbound identity credential. One Anchor per verified
person, and it cannot be sold or moved.

$ENTROS is the fungible utility token. Two different standards for two different purposes. Do
not conflate them.

## Distribution

Entros launches through a public launchpad with no presale, no private round, and no VC
allocation. The team buys its tokens on the launch curve rather than receiving a grant, and
locks them through Streamflow on published schedules that anyone can inspect on-chain.

## Status

There is no program here, and there was never a working one. An earlier scaffold declared an
`initialize` instruction intended to create the mint. That instruction could not have served
$ENTROS, because the launchpad creates the mint and a second mint would be a second token. It
was removed rather than left to mislead.

Code returns to this repository when integrator capacity tiers or the insurance pool are
specified. Everything else in the table above belongs to a repository that already exists.

## License

MIT
