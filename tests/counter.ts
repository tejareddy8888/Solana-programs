import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { Keypair, SystemProgram } from "@solana/web3.js";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;

  const randomKeypair = anchor.web3.Keypair.generate();


  it("Is initialized correctly!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({ counter: randomKeypair.publicKey }).signers([randomKeypair]).rpc()

    console.log("Your Initiatiotransaction signature", tx);
  });

  it("Fail the initialization!", async () => {
    // Add your test here.
     await program.methods.initialize().accounts({ counter: randomKeypair.publicKey }).signers([randomKeypair]).rpc()
  });

  it("Initialize the counter after the account creation the initialization!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeCounter().accounts({ counter: randomKeypair.publicKey }).signers([randomKeypair]).rpc()

    console.log("Initialized counter transaction signature", tx);
  });

  it("Increment the counter!", async () => {
    // Add your test here.
    const tx = await program.methods.increment().accounts({ counter: randomKeypair.publicKey }).signers([randomKeypair]).rpc()

    console.log("Increment counter transaction signature", tx);
  });
});

