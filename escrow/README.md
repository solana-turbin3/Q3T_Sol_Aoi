# Escrow program

## Entrypoint
Entrypoints are the only way to call a program; all calls go through the function declared as the entrypoint

> When called, a program is passed to its [`BPF Loader`](https://docs.solana.com/developing/on-chain-programs/overview#loaders), which processes the call. Different BPF loaders may require different entrypoints.

entrypoint function to take 3 arguments.
- `program_id`
- `instruction_data`
- `accounts`

## Token Program

1. Mint account

- This account represents the token itself and tracks the total supply.
- It holds the authority for minting new tokens and freezing accounts but does not hold any actual token balances.

2. Token account

- This is a generic term for accounts that hold the balance of tokens.
- Each token account is specific to a particular token type and user. It holds the actual token balance for a user.

## Associated Token Account (ATA)

1. Standardization:

- The ATA follows a standardized naming and structure convention, which makes it easier for wallets, exchanges, and other programs to interact with token balances without ambiguity.
- The ATA is derived using a deterministic formula based on the wallet address and the mint address. This means you can predict the ATA address for any given wallet and token mint, ensuring consistency.

2. Convenience:

- It simplifies the user experience by automatically associating a wallet with a specific token account. Users don't need to manually manage multiple token accounts for each token they hold.
- Wallets and applications can automatically find or create the ATA when needed, reducing the need for manual intervention and improving interoperability.

3. Security and Best Practices:

- By using ATAs, wallets and applications can follow best practices for token management, ensuring that token balances are managed securely and consistently.
- It reduces the risk of errors or conflicts that might arise from using non-standard or manually managed token accounts.


## Resources
- https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
- https://github.com/deanmlittle/anchor-escrow-2024
