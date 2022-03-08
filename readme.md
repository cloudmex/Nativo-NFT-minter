# Coming Soon

### This is a command to Mint a Token in the minter SC
near call dev-1645632654382-28045928413066 mint_token '{ "token_owner_id": "joehank.testnet","collection_id":"4","collection":"JoeHank Collection","token_metadata": { "title": "Será este el bueno?", "description": "This is Hola", "media": "bafybeiaovkg2bqc2czsvepo5vtlh7uqyyr6jol25pzibcbrix2oq4ndk54","copies":1,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'joehank.testnet'","'price'":"'2'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId joehank.testnet  --amount 0.1 --gas=300000000000000

near call dev-1645215283232-83708320064039 mint_token '{ "token_owner_id": "joehank.testnet","collection":"Hola","token_metadata": { "title": "Será este el bueno?x3", "description": "This is Hola", "media": "","copies":1,"extra":"{'"'tags'":"'#Azteca'","'creator'":"'joehank.testnet'","'price'":"'3'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}}' --accountId joehank.testnet  --amount 0.1 --gas=300000000000000

### This is the method to view to information of a token in the minter SC
near view dev-1644435847594-74924129418171 nft_token '{"token_id":"1","token_owner_id":"joehank.testnet" }'  --accountId joehank.testnet

near call dev-1644435847594-74924129418171 update_token '{"token_id":"1","extra":"{'"'tags'":"'#Azteca'","'creator'":"'joehank.testnet'","'price'":"'2000000000000000000000000'","'status'":"'S'","'adressbidder'":"'accountbidder'","'highestbidder'":"'notienealtos'","'lowestbidder'":"'notienebajos'","'expires_at'":"'noexpira'","'starts_at'":"'noinicia'"}'"}' --accountId joehank.testnet