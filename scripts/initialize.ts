import { Connection, Keypair } from "@solana/web3.js";

import * as anchor from "@coral-xyz/anchor";
import { Program, Idl, AnchorProvider } from "@coral-xyz/anchor";

import * as idl from "../target/idl/mev_tip_distribution.json";
import type { MevTipDistribution } from "../target/types/mev_tip_distribution";
import bs58 from 'bs58';

const connection = new Connection('https://nd-132-021-973.p2pify.com/e9fd3971adcb438ff5c925f4722b223a');

const b = bs58.decode(process.env.DISTRIBUTOR_SECRET_KEY);
const keyAsBuffer = new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
const tipDistributor = Keypair.fromSecretKey(keyAsBuffer);

const wallet = new anchor.Wallet(tipDistributor);

const provider = new AnchorProvider(connection, wallet, {});

const program = new anchor.Program(idl as Idl as MevTipDistribution, provider)

async function main() {
    const [configPda, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("DISTRIBUTION_CONFIG_ACCOUNT", "utf8")],
        program.programId
    );

    const distributionAuthority = tipDistributor.publicKey;

    console.log(program.programId);
    console.log(program.methods);

    console.log("tip distributor:", distributionAuthority.toBase58());

    const maxTipAmount = new anchor.BN(50000000);

    await program.methods
        .initialize(distributionAuthority, maxTipAmount, bump)
        .accounts({
            config: configPda,
            initializer: tipDistributor.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([tipDistributor])
        .rpc();

    console.log("Initialized distribution config with PDA:", configPda.toBase58());
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});