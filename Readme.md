# Getting Started with Solana Program in Anchor Framework
> I am learning how to build Solana programs using the Anchor framework. This might help others to start their journey too.

**Anchor framework** is a tool that simplifies the process of building Solana programs. Whether you're new to blockchain development or an experienced programmer, Anchor simplifies the process of writing, testing, and deploying Solana programs.

Walk through:

- Cloning this repository
- [Optionally] Running your own local network validator 
- Building and testing your program
- Deploying to Solana clusters
- Modifying the Solana Program code

## **Prerequisites**

Before you begin, ensure you have the following installed:

- Rust: The programming language for building Solana programs.
- Solana CLI: Command-line tool for Solana development.
- Anchor CLI: Command-line tool for the Anchor framework.

#### Rust
To verify Rust installation, open your terminal and run: 
```shell filename="Terminal"
rustc --version
```

Expected output:

```shell filename="Terminal"
rustc 1.80.1 
```

If not, please use this recommend rust installation guidelines [here](https://solana.com/docs/intro/installation#install-rust)

#### Solana CLI
To verify Solana CLI installation, open your terminal and run: 
```shell filename="Terminal"
solana --version
```

Expected output:

```shell filename="Terminal"
solana-cli 1.18.22 
```

If not, please use this recommend solana cli installation guidelines [here](https://solana.com/docs/intro/installation#install-the-solana-cli)

#### Anchor
To verify Anchor CLI installation, open your terminal and run:

```shell filename="Terminal"
anchor --version
```

Expected output:

```shell filename="Terminal"
anchor-cli 0.30.1
```

## Getting Started

This section covers the basic steps to create, build, and test your first local Anchor program.


### Cloning this repository

Clone this repo and the default Anchor Program is located at `/programs/<app-name>/src/lib.rs`.


The value in the `declare_id!` macro is the program ID, a unique identifier for
your program.

By default, it is the public key of the keypair generated in
`/target/deploy/my_project-keypair.json`.

```rs filename="lib.rs"
use anchor_lang::prelude::*;

declare_id!("3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg");

#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
```

The default Typescript test file is located at `/tests/my-project.ts`.


This file demonstrates how to invoke the default program's `initialize`
instruction in Typescript.

```ts filename="my-project.ts"
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyProject } from "../target/types/my_project";

describe("my-project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyProject as Program<MyProject>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
```

If you prefer Rust for testing, initialize your project with the
`--test-template rust` flag.

```shell
anchor init --test-template rust my-program
```

The Rust test file will be at `/tests/src/test_initialize.rs`.


```rust filename="test_initialize.rs"
use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};

#[test]
fn test_initialize() {
    let program_id = "3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let tx = program
        .request()
        .accounts(my_program::accounts::Initialize {})
        .args(my_program::instruction::Initialize {})
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
```
### [Optionally] Running your own local network validator 

Open a new terminal and start a local Solana validator by running the solana-test-validator command.

Just run: `solana-test-validator` in a new terminal  

Additionally, To inspect accounts and transaction logs on the Solana Explorer while developing locally connect the solana explorer to the custom rpc http://localhost:8899

### Build the Program

Build the program by running `anchor build`.

```shell filename="Terminal" copy
anchor build
```

The compiled program will be at `/target/deploy/my_project.so`. The content of
this file is what gets stored on the Solana network (as an executable account)
when you deploy your program.

### Test the Program

To test the program, run `anchor test`.

```shell filename="Terminal" copy
anchor test
```

By default, the `Anchor.toml` config file specifies the `localnet` cluster. When
developing on `localnet`, `anchor test` will automatically:

1. Start a local Solana validator
2. Build and deploy your program to the local cluster
3. Run the tests in the `tests` folder
4. Stop the local Solana validator

Alternatively, you can manually start a local Solana validator and run tests
against it. This is useful if you want to keep the validator running while you
iterate on your program. It allows you to inspect accounts and transaction logs
on the [Solana Explorer](https://explorer.solana.com/?cluster=custom) while
developing locally.

Open a new terminal and start a local Solana validator by running the
`solana-test-validator` command.

```shell filename="Terminal" copy
solana-test-validator
```

In a separate terminal, run the tests against the local cluster. Use the
`--skip-local-validator` flag to skip starting the local validator since it's
already running.

```shell filename="Terminal" copy
anchor test --skip-local-validator
```

### Deploy to Devnet

By default, the `Anchor.toml` config file in an Anchor project specifies the
localnet cluster.

```toml filename="Anchor.toml" {14}
[toolchain]

[features]
resolution = true
skip-lint = false

[programs.localnet]
my_program = "3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

To deploy your program to devnet, change the `cluster` value to `Devnet`. Note
that this requires your wallet to have enough SOL on Devnet to cover deployment
cost.

```diff
-cluster = "Localnet"
+cluster = "Devnet"
```

```toml filename="Anchor.toml"
[provider]
cluster = "Devnet"
wallet = "~/.config/solana/id.json"
```

Now when you run `anchor deploy`, your program will be deployed to the devnet
cluster. The `anchor test` command will also use the cluster specified in the
`Anchor.toml` file.

```shell
anchor deploy
```

To deploy to mainnet, simply update the `Anchor.toml` file to specify the
mainnet cluster.

```toml filename="Anchor.toml"
[provider]
cluster = "Mainnet"
wallet = "~/.config/solana/id.json"
```

### Update the Program

Solana programs can be updated by redeploying the program to the same program
ID.

To update a program, simply make changes to your program's code and run the
`anchor build` command to generated an updated `.so` file.

```shell
anchor build
```

Then run the `anchor deploy` command to redeploy the updated program.

```shell
anchor deploy
```

### Close the Program

To reclaim the SOL allocated to a program account, you can close your Solana
program.

To close a program, use the `solana program close <PROGRAM_ID>` command. For
example:

```shell
solana program close 3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg --bypass-warning
```

Note that once a program is closed, the program ID cannot be reused to deploy a
new program.

## Modifying the Solana Program code

### Firstly lets understand Project File Structure

Below is an overview of default file structure in an Anchor workspace:

```
.
├── .anchor
│   └── program-logs
├── app
├── migrations
├── programs
│   └── [project-name]
│       └── src
│           ├── lib.rs
│           ├── Cargo.toml
│           └── Xargo.toml
├── target
│   ├── deploy
│   │   └── [project-name]-keypair.json
│   ├── idl
│   │   └── [project-name].json
│   └── types
│       └── [project-name].ts
├── tests
│   └── [project-name].ts
├── Anchor.toml
├── Cargo.toml
└── package.json
```

### Programs Folder

The `/programs` folder contains your project's Anchor programs. A single
workspace can contain multiple programs.

### Tests Folder

The `/tests` folder contains test files for your project. A default test file is
created for you when you create your project.

### Target Folder

The `/target` folder contains build outputs. The main subfolders include:

- `/deploy`: Contains the keypair and program binary for your programs.
- `/idl`: Contains the JSON IDL for your programs.
- `/types`: Contains the TypeScript type for the IDL.

### Anchor.toml File

The `Anchor.toml` file configures workspace settings for your project.

### .anchor Folder

Includes a `program-logs` file that contains transaction logs from the last run
of test files.

### App Folder

The `/app` folder is an empty folder that can be optionally used for your
frontend code.


## Using the Anchor Language Documentation, Update the lib.rs file 

References: 
1. https://www.anchor-lang.com
2. https://github.com/solana-developers/program-examples/tree/main
3. https://www.quicknode.com/guides/solana-development/anchor/how-to-use-constraints-in-anchor