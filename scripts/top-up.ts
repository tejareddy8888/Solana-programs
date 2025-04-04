import * as anchor from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

async function main() {
  // Check if the public key is provided as a command-line argument
  if (process.argv.length < 3) {
    console.error("Usage: anchor run topup -- <PUBLIC_KEY>");
    process.exit(1);
  }

  // Get the public key from the command-line arguments
  const publicKeyString = process.argv[2];
  // Set up the provider to connect to the devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Replace with the public key you want to top up
  const publicKey = new anchor.web3.PublicKey(publicKeyString);

  // Request an airdrop of 1 SOL (1 SOL = 1 billion lamports)
  const airdropSignature = await provider.connection.requestAirdrop(publicKey, 1 * LAMPORTS_PER_SOL);

  // Confirm the transaction
  await provider.connection.confirmTransaction(airdropSignature, "confirmed");

  console.log(`Airdropped 1 SOL to ${publicKeyString} in the devnet using ${airdropSignature}`);
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});