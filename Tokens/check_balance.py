from stellar_sdk import Server

# Connect to the Stellar testnet or mainnet Horizon server
server = Server("https://horizon-testnet.stellar.org")  # Use 'https://horizon.stellar.org' for mainnet

# Public key of the account you want to check the balance for
account_id = "GBHCGV6I46FGKMICTBJWQJUH2GMJNZLICJKJ6QVLR4O6NMTCNQCC3BG3"  # Replace with the public key of the account

# Load the account from the Stellar network
try:
    account = server.accounts().account_id(account_id).call()
except Exception as e:
    print(f"Error fetching account details: {e}")
    exit()

# Loop through the balances and print them
for balance in account['balances']:
    asset_type = balance['asset_type']
    
    if asset_type == 'native':
        print(f"Asset: XLM, Balance: {balance['balance']}")
    else:
        asset_code = balance['asset_code']
        asset_issuer = balance['asset_issuer']
        print(f"Asset: {asset_code}, Issuer: {asset_issuer}, Balance: {balance['balance']}")
