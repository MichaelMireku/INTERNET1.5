//solana wallet authentication

import { Connection, PublicKey } from "@solana/web3.js";

export async function connectWallet(): Promise<string> {
  if (!window.solana) throw new Error("Solana wallet not found");

  const wallet = window.solana;
  const response = await wallet.connect();
  return response.publicKey.toString();
}
