import { RiskReport } from '../src/types';
import { Connection } from '@solana/web3.js';

describe('Lenz SDK', () => {
    describe('Types', () => {
        it('should structure risk report correctly', () => {
            const report: RiskReport = {
                address: '11111111111111111111111111111111',
                riskScore: 0,
                isSafe: true,
                flags: [],
                timestamp: 123456789
            };
            expect(report.isSafe).toBe(true);
        });
    });

    describe('Client', () => {
        it('should initialize connection', () => {
            const conn = new Connection('https://api.devnet.solana.com');
            expect(conn).toBeDefined();
        });
    });
});
