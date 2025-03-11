import { create } from "ipfs-http-client";
import { CONFIG } from "./config";

const ipfs = create({ url: CONFIG.STORAGE_NODE_URL });

export async function uploadFile(file: File): Promise<string> {
  const result = await ipfs.add(file);
  return result.path; // Returns IPFS hash
}

export async function retrieveFile(hash: string): Promise<string> {
  return `${CONFIG.STORAGE_NODE_URL}/ipfs/${hash}`;
}
