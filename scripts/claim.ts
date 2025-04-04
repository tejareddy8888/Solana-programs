import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MevTipDistribution } from "../target/types/mev_tip_distribution";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MevTipDistribution as Program<MevTipDistribution>;

  const [configPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("DISTRIBUTION_CONFIG_ACCOUNT", "utf8")],
    program.programId
  );

  const claimant = anchor.web3.Keypair.generate();
  const tipDistributor = anchor.web3.Keypair.generate(); // Ensure this is the same keypair used in initialize
  const amount = new anchor.BN(50000000);

  await program.methods
    .claim(amount)
    .accounts({
      config: configPda,
      claimant: claimant.publicKey,
      distribution_authority: tipDistributor.publicKey,
      payer: tipDistributor.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([tipDistributor])
    .rpc();

  console.log("Claimed lamports for claimant:", claimant.publicKey.toBase58());
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});