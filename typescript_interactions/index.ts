import { WalletUnlocked, Provider, PredicateParameters, PredicateParams, Predicate } from 'fuels';
import fs from 'fs';

const FUEL_NETWORK_URL = 'http://127.0.0.1:4000/v1/graphql';
 
// Create the provider
const provider = await Provider.create(FUEL_NETWORK_URL);

// Initialize the wallets
const wallet = await new WalletUnlocked("de97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c", provider);
const receiver = await WalletUnlocked.generate({
    provider,
});

// Read the bytecode from the file
const binary = fs.readFileSync('../out/debug/predicate.bin');
const abi = fs.readFileSync('../out/debug/predicate-abi.json');
const predicateParams: PredicateParams = {
    bytecode: binary,
    provider,
    abi: JSON.parse(abi.toString()),
};
// Create the predicate
const predicate = new Predicate(predicateParams);

// transferring funds to the predicate
const tx1 = await wallet.transfer(predicate.address, 500_000, provider.getBaseAssetId(), {
    gasLimit: 1000,
});

await tx1.waitForResult();

// transferring funds from the predicate to destination if predicate returns true
const tx2 = await predicate.transfer(
    receiver.address,
    500_000,
    provider.getBaseAssetId(),
    {
      gasLimit: 1000,
    }
);
   
await tx2.waitForResult();

console.log('Receiver\'s balance: ' + (await receiver.getBalance()).toString());