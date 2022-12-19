# GigaDaoTreasury

Mainnet Program ID: `2omcykYnUGQW8tDGKZFMuJHAswrfMDAgMTkBo3Kd6Woj`

This Solana on-chain program is implemented in anchorlang framework and serves as GigaDAO's primary treasury.

Team withdrawals are capped at $25k/mo.
This cannot be changed without a GIGS holders vote.

### Endpoints

The program implements two key instructions:
1) Initialize Treasury
2) Execute Withdraw

### Upgrade Authority

The UA for this contract is held by a GigaDAO governance contract which itself is not upgradeable.
The UA address is `CKk2EQ6ybz6qMxMAVgDxdRksjLAQgLTfW47t9LwERW3z`, which is a PDA owned by `GzMvD8AGSiRhHapNsJzUMoYR3pkbCg6vPnnopaeFZE7E`.
The latter is a non-upgradeable voting contract gated by GIGS voting. 

### Verification

This program was deployed using `anchor build --verifiable`, meaning that anyone can download this repo,
build it locally (using anchor's docker verified build routine), and confirm it matches the on-chain binary.

Prerequisites:
- solana-cli v1.11.10
- anchor-cli v0.25.0
- node v18.8.0
- yarn
- docker

After installing the prerequisites, clone this repo, install with `yarn install`, followed by:

`anchor verify -p gdvesting 2omcykYnUGQW8tDGKZFMuJHAswrfMDAgMTkBo3Kd6Woj`


