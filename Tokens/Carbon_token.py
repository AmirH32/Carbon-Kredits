from stellar_sdk import Server, Keypair, Network, TransactionBuilder

# Create a keypair for both issuer and reciever (public/private key) this should come from frontend
issuing_keypair = Keypair.random()
distribution_keypair = Keypair.random()

print(f"Public Key: {keypair.public_key}")
print(f"Secret Key: {keypair.secret}")

# Connect to the Stellar test network
server = Server(horizon_url="hhttps://soroban-testnet.stellar.org:443")

# Create the Carbon Token (Asset) and sign it with public key to authenticate issuer
carbon_token = Asset("CARBON", issuing_keypair.public_key)

# Create trustline from distribution account to issuing account
distribution_account = server.load_account(distribution_keypair.public_key)
trust_transaction = TransactionBuilder(
    source_account=distribution_account,
    network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,
    base_fee=100
).add_operation(
    Operation.change_trust(asset=carbon_token)
).set_timeout(30).build()

# Sign and submit the transaction to create the trustline
trust_transaction.sign(distribution_keypair)
response = server.submit_transaction(trust_transaction)
print("Trustline created:", response)