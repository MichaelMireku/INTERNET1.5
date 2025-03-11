import { Connection, PublicKey, Transaction } from "@solana/web3.js";
import { CONFIG } from "./config";

export async function payForStorage(walletAddress: string, amount: number) {
  const connection = new Connection(CONFIG.BLOCKCHAIN_RPC);
  const sender = new PublicKey(walletAddress);
  const transaction = new Transaction();

  // Build transaction to pay storage node (stub)
  console.log(`User ${walletAddress} pays ${amount} SOL`);

  return transaction;
}
