from stellar_sdk import Server, Keypair, Network, TransactionBuilder, Asset, Operation

# Define issuer and distribution keypairs using the provided public and secret keys
issuing_keypair = Keypair.from_secret("SAU7KGZXJZEYHB22CKF2ADP7BPLLJZRYP4Z43CWBABHTBRQI54KMVEGD")
distribution_keypair = Keypair.from_secret("SDYMPYU6IKFHKDKKVQJ6SAUYNP6OIBBU7GNFVUOHQCNPSJBT6MNGDHHA")

print(f"Issuing Public Key: {issuing_keypair.public_key}")
print(f"Issuing Secret Key: {issuing_keypair.secret}")
print(f"Distribution Public Key: {distribution_keypair.public_key}")
print(f"Distribution Secret Key: {distribution_keypair.secret}")

# Connect to the Stellar test network
server = Server(horizon_url="https://horizon-testnet.stellar.org")

# Create the Carbon Token (Asset) and sign it with the public key to authenticate the issuer
carbon_token = Asset("CARBON", issuing_keypair.public_key)

# Fund the distribution account using the Stellar testnet faucet
# Ensure this step is done before proceeding
print("Please fund the distribution account with the testnet faucet before continuing.")

# Load the distribution account
try:
    distribution_account = server.load_account(distribution_keypair.public_key)
except Exception as e:
    print(e)
    print("Distribution account not found. Please create and fund it using the faucet.")
    exit()

# Create a trustline from distribution account to issuing account
trust_transaction = TransactionBuilder(
    source_account=distribution_account,
    network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,
    base_fee=100  # 100 stroops (default base fee)
).append_change_trust_op(asset=carbon_token, limit = 10000).set_timeout(30).build()

# Sign and submit the transaction to create the trustline
trust_transaction.sign(distribution_keypair)
try:
    response = server.submit_transaction(trust_transaction)
    print("Trustline created:", response)
except:
    print("Error creating trustline:", e)

# Add the issuer and recipient information to the issuing account
try:
    issuer_account = server.load_account(issuing_keypair.public_key)

    manage_data_transaction = TransactionBuilder(
        source_account=issuer_account,
        network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,
        base_fee=100  # Base fee in stroops
    ).append_manage_data_op(data_name="Issuer", data_value=issuing_keypair.public_key).append_manage_data_op(data_name="Recipient", data_value=distribution_keypair.public_key).set_timeout(30).build()

    # Sign and submit the transaction
    manage_data_transaction.sign(issuing_keypair)
    response = server.submit_transaction(manage_data_transaction)
    print("Data added to issuing account:", response)
except Exception as e:
    print("Error adding data to issuing account:", e)

# Load the issuing account to mint CARBON tokens
try:
    issuing_account = server.load_account(issuing_keypair.public_key)

    # Mint 1000 CARBON tokens from the issuing account to the distribution account
    mint_transaction = TransactionBuilder(
        source_account=issuing_account,
        network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,
        base_fee=100  # Base fee in stroops
    ).append_operation(
        Operation.payment(
            destination=distribution_keypair.public_key,
            asset=carbon_token,
            amount="1000"  # Mint 1000 CARBON tokens
        )
    ).set_timeout(30).build()

    # Sign the transaction with the issuing account's secret key
    mint_transaction.sign(issuing_keypair)

    # Submit the transaction to the Stellar network
    response = server.submit_transaction(mint_transaction)
    print("CARBON tokens minted and distributed:", response)
except NotFoundError:
    print("Issuing account not found.")
except Exception as e:
    print("Error minting tokens:", e)

# Optional LOCKING ACCOUNTS (Commented out to avoid premature execution)
# lock_transaction = TransactionBuilder(
#     source_account=issuing_account,
#     network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,
#     base_fee=100  # Base fee in stroops
# ).add_operation(
#     Operation.set_options(master_weight=0)  # Set master weight to 0 to lock the account
# ).set_timeout(30).build()

# # Sign and submit the transaction
# lock_transaction.sign(issuing_keypair)
# response = server.submit_transaction(lock_transaction)
# print("Issuing account locked:", response)
