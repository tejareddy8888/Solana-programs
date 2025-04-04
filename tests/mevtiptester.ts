import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MevTipDistribution } from "../target/types/mev_tip_distribution";
import { assert } from "chai";
import { Keypair, PublicKey } from "@solana/web3.js";
import bs58 from 'bs58';

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const tipDistribution = anchor.workspace.MevTipDistribution as Program<MevTipDistribution>;

const b = bs58.decode(process.env.DISTRIBUTOR_SECRET_KEY);
const keyAsBuffer = new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
const tipDistributor = Keypair.fromSecretKey(keyAsBuffer);

describe("mev_tip_distribution", () => {
    // let intruder = anchor.web3.Keypair.generate();

    let configPda, bump;
    before(async () => {
        [configPda, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("DISTRIBUTION_CONFIG_ACCOUNT", "utf8")],
            tipDistribution.programId
        );
        // const signature = await provider.connection.requestAirdrop(tipDistributor.publicKey, 1000000000);
        // await provider.connection.confirmTransaction(signature, "confirmed");

        // const Isignature = await provider.connection.requestAirdrop(tipDistributor.publicKey, 1000000000);
        // await provider.connection.confirmTransaction(Isignature, "confirmed");
    })

    it.skip("Initializes the distribution config", async () => {


        const distributionAuthority = tipDistributor.publicKey;
        const maxTipAmount = new anchor.BN(50000000);

        await tipDistribution.methods
            .initialize(distributionAuthority, maxTipAmount, bump)
            .accounts({
                config: configPda,
                initializer: tipDistributor.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).signers([tipDistributor])
            .rpc();

        const configAccount = await tipDistribution.account.distributionConfig.fetch(configPda);
        assert.equal(configAccount.distributionAuthority.toBase58(), distributionAuthority.toBase58());
        assert.equal(configAccount.maxTipAmount.toNumber(), maxTipAmount.toNumber());
    });

    it.skip("Topup lamports on DistributionConfig", async () => {
        const [configPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from("DISTRIBUTION_CONFIG_ACCOUNT", "utf8")],
            tipDistribution.programId
        );

        const signature = await provider.connection.requestAirdrop(configPda, 1000000000);
        await provider.connection.confirmTransaction(signature, "confirmed");

        const claimantBalance = await provider.connection.getBalance(configPda);
        assert.equal(claimantBalance, 1001280640);
    });

    it("Claims lamports", async () => {


        const claimant = anchor.web3.Keypair.generate();
        const amount = new anchor.BN(50000000);

        // Airdrop some lamports to the distribution authority for testing


        await tipDistribution.methods
            .claim(amount)
            .accounts({
                config: new PublicKey('5kAFfi7CofNYiPJDpE7JZkRJfmWsn9KsuEEH6r9sNnE3'),
                claimant: new PublicKey('9Ea6yQZWUMw4BTTXKoauusxCjTGWPotuGD9aMFTDzMzb'),
                distributionAuthority: tipDistributor.publicKey,
                payer: tipDistributor.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([tipDistributor])
            .rpc();

        const claimantBalance = await provider.connection.getBalance(new PublicKey('9Ea6yQZWUMw4BTTXKoauusxCjTGWPotuGD9aMFTDzMzb'));
        assert.equal(claimantBalance, amount.toNumber());
    });

    // it("Should fail to Claims lamports", async () => {
    //     const [configPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
    //         [Buffer.from("DISTRIBUTION_CONFIG_ACCOUNT", "utf8")],
    //         tipDistribution.programId
    //     );

    //     const claimant = anchor.web3.Keypair.generate();
    //     const amount = new anchor.BN(50000000);

    //     // Airdrop some lamports to the distribution authority for testing


    //     await tipDistribution.methods
    //         .claim(amount)
    //         .accounts({
    //             config: configPda,
    //             claimant: claimant.publicKey,
    //             distributionAuthority: tipDistributor.publicKey,
    //             payer: intruder.publicKey,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //         })
    //         .signers([intruder])
    //         .rpc();

    //     const claimantBalance = await provider.connection.getBalance(claimant.publicKey);
    //     assert.equal(claimantBalance, amount.toNumber());
    // });


    // it("Should fail to Claims lamports", async () => {
    //     const [configPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
    //         [Buffer.from("DISTRIBUTION_CONFIG_ACCOUNT", "utf8")],
    //         tipDistribution.programId
    //     );

    //     const claimant = anchor.web3.Keypair.generate();
    //     const amount = new anchor.BN(50000000);

    //     // Airdrop some lamports to the distribution authority for testing


    //     await tipDistribution.methods
    //         .claim(amount)
    //         .accounts({
    //             config: configPda,
    //             claimant: claimant.publicKey,
    //             distributionAuthority: intruder.publicKey,
    //             payer: intruder.publicKey,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //         })
    //         .signers([intruder])
    //         .rpc();

    //     const claimantBalance = await provider.connection.getBalance(claimant.publicKey);
    //     assert.equal(claimantBalance, amount.toNumber());
    // });
});