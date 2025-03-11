interface SolanaWallet {
    isPhantom?: boolean;
    connect: () => Promise<{ publicKey: { toString: () => string } }>;
    disconnect: () => Promise<void>;
}

interface Window {
    solana?: SolanaWallet;
}
