const StellarSdk = require('stellar-sdk');
const server = new StellarSdk.Server('https://horizon-testnet.stellar.org');

// Issuer account
const issuer = StellarSdk.Keypair.fromSecret('ISSUER_SECRET');
// Distribution account
const distributor = StellarSdk.Keypair.fromSecret('DISTRIBUTOR_SECRET');

// Create asset (e.g., CarbonToken)
const carbonToken = new StellarSdk.Asset('CarbonToken', issuer.publicKey());

// Establish a trustline to the asset in the distribution account
const transaction = new StellarSdk.TransactionBuilder(distributor, { fee: StellarSdk.BASE_FEE })
  .addOperation(StellarSdk.Operation.changeTrust({
    asset: carbonToken
  }))
  .setTimeout(30)
  .build();
// puts token into the wallet

transaction.sign(distributor);
server.submitTransaction(transaction);

const transaction = new StellarSdk.TransactionBuilder(issuer, { fee: StellarSdk.BASE_FEE })
  .addOperation(StellarSdk.Operation.payment({
    destination: distributor.publicKey(),
    asset: carbonToken,
    amount: '1000'  // 1000 Carbon Credits, for example
  }))
  .setTimeout(30)
  .build();

transaction.sign(issuer);
server.submitTransaction(transaction);
// Sends 1000 tokens t