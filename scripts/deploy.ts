import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import * as fs from 'fs';

// 1. Configure your deployment
const CONTRACT_WASM_PATH = '../target/ink/billing.wasm';
const CONTRACT_METADATA_PATH = '../target/ink/billing.json';
const NETWORK_ENDPOINT = 'wss://rococo-contracts-rpc.polkadot.io'; // Testnet

async function main() {
    // 2. Initialize Polkadot.js
    await cryptoWaitReady();
    const provider = new WsProvider(NETWORK_ENDPOINT);
    const api = await ApiPromise.create({ provider });
    
    // 3. Load contract artifacts
    const wasm = fs.readFileSync(CONTRACT_WASM_PATH);
    const metadata = JSON.parse(fs.readFileSync(CONTRACT_METADATA_PATH, 'utf8'));
    
    // 4. Create deployer keypair (use test account for development)
    const keyring = new Keyring({ type: 'sr25519' });
    const deployer = keyring.addFromUri('//Alice'); // Test account
    
    // 5. Create contract instance
    const contract = new ContractPromise(api, metadata, wasm);
    
    // 6. Estimate gas requirements
    const gasLimit = await api.registry.createType(
        'WeightV2',
        api.consts.system.blockWeights['maxBlock']
    );
    
    // 7. Deploy the contract
    console.log('Deploying contract...');
    const tx = contract.tx.new(
        { gasLimit }, // Constructor arguments
    );
    
    const hash = await tx.signAndSend(deployer, (result) => {
        if (result.status.isInBlock) {
            console.log(`Deployed in block: ${result.status.asInBlock}`);
        }
        if (result.status.isFinalized) {
            console.log(`Finalized at: ${result.status.asFinalized}`);
            process.exit(0);
        }
    });
    
    console.log(`Transaction hash: ${hash}`);
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});