

for i in {1..10}; do near call dev-1643314838825-38374018844065 market_mint_generic '{ "contractaddress":"dev-1643320681606-60313486665423", "token_owner_id": "'yairnava.testnet'", "colecction": "'Items_Nativo_X'", "token_metadata": { "title": "Thunder Sword", "description": "Thunder Sword", "media": "","extra":"{'"'attack'":"'3'","'defense'":"'0'","'speed'":"'0'"}'"}}'  --accountId yairnava.testnet --amount 0.1 --gas=300000000000000; done

near call dev-1643331107973-95015694722073 market_mint_generic '{ "contractaddress":"dev-1643329021198-22907018449665", "token_owner_id": "dokxo.testnet","colecction":"nativo","token_metadata": { "title": "Será este el bueno?x2", "description": "This is Hola x36", "media": "","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'5'","'status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000


for i in {1..1000}; do near call dev-1643329021198-22907018449665 nft_mint_token '{ "contractaddress":"dev-1643331107973-95015694722073", "token_owner_id": "dokxo.testnet","colecction":"nativo","token_metadata": { "title": "Será este el bueno?x2", "description": "This is Hola x36", "media": "","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'5'","'status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000; done



for i in {1..1000}; do near call dev-1643332282510-97924906831834 nft_mint_token '{"token_owner_id": "'dokxo.testnet'", "colecction": "'Burritos'", "token_metadata": { "title": "Doped Burrito", "description": "This is a doped burrito", "media": "","extra":""}}' --accountId dokxo.testnet --deposit 0.1 --gas=300000000000000; done


for i in {1..1000}; do near call dev-1643332286113-57268447308228 nft_mint_token '{"token_owner_id": "'dokxo.testnet'", "colecction": "'Items'", "token_metadata": { "title": "Thunder Sword", "description": "Thunder Sword 2", "media": "","extra":"{'"'attack'":"'3'","'defense'":"'0'","'speed'":"'0'"}'"}}' --accountId dokxo.testnet --deposit 0.1 --gas=300000000000000; done


near call dev-1643329021198-22907018449665 nft_mint_token '{ "contractaddress":"dev-1643331107973-95015694722073", "token_owner_id": "dokxo.testnet","colecction":"nativo100","token_metadata": status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000


near call dev-1643331107973-95015694722073 market_mint_generic '{ "contractaddress":"dev-1643329021198-22907018449665", "token_owner_id": "joehank.testnet","colecction":"dokxodeded","token_metadata": { "title": "Será este el bueno?x2", "description": "This is Hola x36", "media": "","extra":"{'"'culture'":"'Azteca'","'country'":"'Mexico'","'creator'":"'joehank.testnet'","'price'":"'5'","'status'":"'S'","'on_sale'":'false',"'on_auction'":'false',"'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId dokxo.testnet  --amount 0.1 --gas=300000000000000