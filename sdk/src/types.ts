import { PublicKey } from '@solana/web3.js';
import BN from 'bn.js';

export interface ScanConfig {
    /** Timeout in milliseconds */
    timeout?: number;
    /** Whether to include detailed logs */
    verbose?: boolean;
}

export interface RiskReport {
    address: string;
    riskScore: number;
    isSafe: boolean;
    flags: string[];
    timestamp: number;
}
