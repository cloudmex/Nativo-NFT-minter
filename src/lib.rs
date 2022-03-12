
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;

use near_sdk::json_types::{ValidAccountId};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, BorshStorageKey, Promise, PromiseOrValue,Gas,ext_contract
};
use std::convert::TryInto;
use near_sdk::env::BLOCKCHAIN_INTERFACE;

near_sdk::setup_alloc!();
/// Balance is type for storing amounts of tokens.
pub type Balance = u128;

pub const TGAS: u64 = 1_000_000_000_000;
/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = 30_000_000_000_000;

pub const GAS_FOR_UPGRADE_REMOTE_DEPLOY: Gas = 10_000_000_000_000;

const NATIVO_ACCOUNT: &str = "nativo-minter.testnet";
//const MARKET_ACCOUNT: &str = "dev-1646870160585-51485952860632";
 
#[derive(BorshDeserialize, BorshSerialize )]
pub struct OldContract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
    n_bid_per_token:u64,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize )]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
    n_bid_per_token:u64,
}
 
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Meta {
    token_id : String,
    owner_id : String,
    title : String,
    description : String,
    media : String,
    tags: String,
    creator : String,
    price : String,
    status: String, // sale status
    adressbidder: String,
    highestbidder: String,
    lowestbidder: String,
    expires_at: String,
    starts_at: String,
}
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Thegraphstructure {
    contract_name:String,
    collection:String,
    collection_id:String,
    token_id : String,
    owner_id : String,
    last_owner_id: String,
    title : String,
    description : String,
    media : String,
    creator : String,
    price : String,
    status: String, // sale status
    adressbidder: String,
    highestbid: String,
    lowestbid: String,
    expires_at: String,
    starts_at: String,
    extra:String,
}
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Extras {
    tags : String,
    creator : String,
    price : String,
    status: String, // sale status
    adressbidder: String,
    highestbidder: String,
    lowestbidder: String,
    expires_at: String,
    starts_at: String,
}

//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(ext_nft)]
trait NonFungibleToken {
    // view method
    fn save_mint_ttg(&self,info : String);
    fn save_buy_ttg(&self,info : String);
    fn save_sell_ttg(&self,info : String);
    fn save_remove_ttg(&self,info : String);

}

impl Default for Contract {
    
    fn default( ) -> Self {
      
     let meta =NFTContractMetadata {
         spec: NFT_METADATA_SPEC.to_string(),
         name: "Nativo NFT".to_string(),
         symbol: "NTV".to_string(),
         icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
         base_uri: None,
         reference: None,
         reference_hash: None,
     };
     Self {
     
         tokens:NonFungibleToken::new(
             StorageKey::NonFungibleToken,
             env::signer_account_id().try_into().unwrap(),
             Some(StorageKey::TokenMetadata),
             Some(StorageKey::Enumeration),
             Some(StorageKey::Approval),

         ) ,
         metadata: LazyOption::new(StorageKey::Metadata, Some(&meta)),
         n_total_tokens: 0,
         n_token_on_sale: 0,
         n_token_on_auction: 0,
         n_bid_per_token: 0,
         
     }   }
}
 
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    // Esta función incializa el contrato con los valores especificados en la metadata

    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Nativo NFT".to_string(),
                symbol: "NTV".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            n_total_tokens: 0,
            n_token_on_sale: 0,
            n_token_on_auction: 0,
            n_bid_per_token: 0,
        }
    }
    
    pub fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
        let metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .unwrap()
            .get(&token_id);
        let approved_account_ids = Some(
            self.tokens
                .approvals_by_id
                .as_ref()
                .unwrap()
                .get(&token_id)
                .unwrap_or_default(),
        );

        Token {
            token_id,
            owner_id,
            metadata,
            approved_account_ids,
        }
    }

  /// ///////////////////////////////////CREACION DE TOKENS
    #[payable]
    pub fn mint_token_ext(&mut self,token_owner_id: ValidAccountId,collection_id: String,collection:String,token_metadata: TokenMetadata) -> String {
       
        let token_id: TokenId =self.n_total_tokens.to_string();
        assert_eq!(
            collection != "",
            true,
            "Coleccion vacia "
        );
        assert_eq!(
            token_metadata.media != None,
            true,
            "media del token vacia "
        );
        log!("Before \n{:?}\n{:?}", env::prepaid_gas(), env::used_gas());
        
        let _mined= self.tokens.mint(
            token_id.clone(),
            token_owner_id.clone(),    
            Some(token_metadata.clone())
        );  
        log!("After \n{:?}\n{:?}", env::prepaid_gas(), env::used_gas());

        self.n_total_tokens  +=1;
        self.n_token_on_sale += 1;

        let newextradata = str::replace(&token_metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        
        let extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();  
       
        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
          
        let graphdata = Thegraphstructure {
            contract_name: NATIVO_ACCOUNT.to_string(),
            collection: collection.clone().to_string(),
            collection_id: collection_id.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : extradatajson.creator.to_string(),
            last_owner_id: "".to_string(),
            title : token_metadata.title.as_ref().unwrap().to_string(),
            description : token_metadata.description.as_ref().unwrap().to_string(),
            media : token_metadata.media.as_ref().unwrap().to_string(),
            creator : extradatajson.creator.to_string(),
            price : extradatajson.price.to_string(),
            status: "S".to_string(), // sale status  PENDIENTE
            adressbidder: extradatajson.adressbidder.to_string(),
            highestbid: extradatajson.highestbidder.to_string(),
            lowestbid: extradatajson.lowestbidder.to_string(),
            expires_at: extradatajson.expires_at.to_string(),
            starts_at: extradatajson.starts_at.to_string(),
            extra: ext,     
        };  
        let rett = serde_json::to_string(&graphdata).unwrap();
    return rett;
    }
    //////////////////////////////////////CREACION DE TOKENS
    
    //////////////////////////////////////COMPRA/VENTA DE TOKENS
    //#[payable]
    pub fn buy_token_ext(&mut self, token_id: TokenId,collection_id:String, collection:String, amount: Balance) -> String {
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
        //obtener los metadatos de ese token
        let mut originaltoken = self
            .tokens
            .token_metadata_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        
        //obtener el dueño del token   
        let owner_id = self.tokens.owner_by_id.get(&token_id);
        let owner_value = owner_id.as_deref().unwrap_or("default string");
        let metadataextra = Contract::get_token(self, token_id.clone());
      
        //let amount = env::attached_deposit();
        assert_eq!(
            metadataextra.price.parse::<u128>().unwrap(),
            amount,
            "Cantidad incorrecta,verifica el costo exacto!"
        );
        //revisar que este a la venta
        assert_eq!(
            metadataextra.status,
            "S".to_string(), 
            "no esta a la venta"
        );
        //obtener el comprador del token
        let buyer_id = &env::signer_account_id();
        // el dueñp no puede comprar su propio token
        assert_eq!(buyer_id == &owner_value, false, "eres el dueño del token ");
       
        //cambiar la metadata        
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();   
        //Se modifica el json
        extradatajson.status = "U".to_string();
        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
        //remplazamos la metadata
        originaltoken.extra = Some(finalextrajson);
        
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        
        //transferir el nft
        self.tokens
            .internal_transfer_unguarded(&token_id, &owner_value.to_string(), buyer_id);

        //cambiar el numero de nfts disponibles
        self.n_token_on_sale -= 1;

        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
         
        let graphdata = Thegraphstructure {
            contract_name: NATIVO_ACCOUNT.to_string(),
            collection: collection.clone().to_string(),
            collection_id: collection_id.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : buyer_id.to_string(),
            last_owner_id: owner_value.to_string().clone(),
            title : "".to_string(),
            description : "".to_string(),
            media : "".to_string(),
            creator : extradatajson.creator.to_string(),
            price : extradatajson.price.to_string(),
            status: extradatajson.status.to_string().to_string(), // sale status PENDIENTE
            adressbidder: extradatajson.adressbidder.to_string(),
            highestbid: extradatajson.highestbidder.to_string(),
            lowestbid: extradatajson.lowestbidder.to_string(),
            expires_at: extradatajson.expires_at.to_string(),
            starts_at: extradatajson.starts_at.to_string(),
            extra: ext  ,
            
        };  
        let rett = serde_json::to_string(&graphdata).unwrap();
    return rett;
    }
   
    pub fn sell_token_ext(&mut self, token_id: TokenId, price: u128,collection_id:String, collection: String ) -> String {
        // //comprobar que el token exista
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
        //comprobar que el revendedor sea el owner
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        assert_eq!(
            env::signer_account_id() == owner_id,
            true,
            "no eres el dueño del token "
        );
        //obtener los metadatos de ese token
        let mut originaltoken = self
            .tokens
            .token_metadata_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
        //Se modifica el json
        if price > 0 {
            extradatajson.price =  price.to_string().clone();
        }
        extradatajson.status = "S".to_string();

        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
      
        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //cambiar el numero de nfts disponibles
        self.n_token_on_sale += 1;
        
        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
          
          let graphdata = Thegraphstructure {
            contract_name: NATIVO_ACCOUNT.to_string(),
            collection:collection.clone().to_string(),
            collection_id: collection_id.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
            last_owner_id: "".to_string(),
            title : "".to_string(),
            description : "".to_string(),
            media : "".to_string(),
            creator : extradatajson.creator.to_string(),
            price : extradatajson.price.to_string(),
            status: extradatajson.status.to_string(), // sale status
            adressbidder: extradatajson.adressbidder.to_string(),
            highestbid: extradatajson.highestbidder.to_string(),
            lowestbid: extradatajson.lowestbidder.to_string(),
            expires_at: extradatajson.expires_at.to_string(),
            starts_at: extradatajson.starts_at.to_string(),
            extra: ext  ,
            
        };  
        let rett = serde_json::to_string(&graphdata).unwrap();
    return rett;
    }
    //////////////////////////////////////COMPRA/VENTA DE TOKENS

    //////////////////////////////////////OFERTAS POR TOKEN
    #[payable]
    pub fn bid_token_ext(&mut self, token_id: TokenId, collection_id: String, collection: String, amount: Balance) -> String {
        //asegurarnos de que el numero sea positivo y este dentro el rango de tokens minados
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
        //obtener los metadatos de ese token
        let mut originaltoken = self
            .tokens
            .token_metadata_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
            
        let owner_id = self.tokens.owner_by_id.get(&token_id);
        let owner_value = owner_id.as_deref().unwrap_or("default string");
        let metadataextra = Contract::get_token(self, token_id.clone());
      
        //let amount = env::attached_deposit();
        assert_eq!(
            metadataextra.highestbidder.parse::<u128>().unwrap_or_default() < amount && 0 < amount,
            true,
            "Su oferta es menor a la oferta actual!"
        );
        
        let new_account_bidder = &env::signer_account_id();
        let last_amount_bidder = metadataextra.highestbidder.parse::<u128>().unwrap_or_default();
        let last_account_bidder = metadataextra.adressbidder;
        // el dueñp no puede comprar su propio token
        assert_eq!(new_account_bidder == &owner_value, false, "eres el dueño del token ");

        //cambiar la metadata        
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();   
        //Se modifica el json
        extradatajson.adressbidder = new_account_bidder.clone().to_string();

        extradatajson.highestbidder = amount.clone().to_string();

        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");

        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        
        self.n_bid_per_token  +=1;

        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
         
        let graphdata = Thegraphstructure {
            contract_name: NATIVO_ACCOUNT.to_string(),
            collection: collection.clone().to_string(),
            collection_id: collection_id.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : owner_value.clone().to_string(),
            last_owner_id: last_account_bidder.to_string(),
            title : "".to_string(),
            description : "".to_string(),
            media : "".to_string(),
            creator : extradatajson.creator.to_string(),
            price : extradatajson.price.to_string(),
            status: extradatajson.status.to_string().to_string(), // sale status PENDIENTE
            adressbidder: new_account_bidder.to_string(),
            highestbid: amount.clone().to_string(),
            lowestbid: last_amount_bidder.to_string(),
            expires_at: self.n_bid_per_token.to_string(),
            starts_at: extradatajson.starts_at.to_string(),
            extra: ext  ,
        };  
        let rett = serde_json::to_string(&graphdata).unwrap();
    return rett;
    }

    pub fn close_bid_token_ext(&mut self, token_id: TokenId, collection_id: String, collection: String, status:bool ) -> String {
        //asegurarnos de que el numero sea positivo y este dentro el rango de tokens minados
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
        //obtener los metadatos de ese token
        let mut originaltoken = self
            .tokens
            .token_metadata_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
            
        let metadataextra = Contract::get_token(self, token_id.clone());

        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        
        assert_eq!(
            metadataextra.highestbidder != "notienealtos".to_string(),
            true, 
            "no existe una oferta anterior"
        );
        
        let last_amount_bidder = metadataextra.highestbidder.parse::<u128>().unwrap_or_default();
        //regresar el bid al singer anterior en caso de existir
        if status {
            if last_amount_bidder > 0 {
                if env::signer_account_id() == owner_id{
                    assert_eq!(
                        env::signer_account_id() == owner_id,
                        true,
                        "no eres el dueño del token "
                    );
                    //transferir el nft
                    self.tokens
                    .internal_transfer_unguarded(&token_id, &owner_id.to_string(), &metadataextra.adressbidder.to_string());
                    //cambiar el numero de nfts disponibles
                    self.n_token_on_sale -= 1;

                }
            }
        }else {
            if env::signer_account_id() == metadataextra.adressbidder {
                assert_eq!(
                    env::signer_account_id() == metadataextra.adressbidder,
                    true,
                    "no tienes ofertas en este token "
                );
            }
        }

        //cambiar la metadata        
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();   
        //Se modifica el json
        extradatajson.adressbidder = "accountbidder".to_string();
        extradatajson.highestbidder = "notienealtos".to_string();
        extradatajson.status = "U".to_string();
        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");

        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        
        let new_owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
         
        let graphdata = Thegraphstructure {
            contract_name: NATIVO_ACCOUNT.to_string(),
            collection: collection.clone().to_string(),
            collection_id: collection_id.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : new_owner_id.clone().to_string(),
            last_owner_id: owner_id.to_string(),
            title : "".to_string(),
            description : "".to_string(),
            media : "".to_string(),
            creator : extradatajson.creator.to_string(),
            price : extradatajson.price.to_string(),
            status: extradatajson.status.to_string().to_string(), // sale status PENDIENTE
            adressbidder: metadataextra.adressbidder.to_string(),
            highestbid: last_amount_bidder.to_string(),
            lowestbid: extradatajson.lowestbidder.to_string(),
            expires_at: extradatajson.expires_at.to_string(),
            starts_at: extradatajson.starts_at.to_string(),
            extra: ext  ,
            
        };
        let rett = serde_json::to_string(&graphdata).unwrap();
    return rett;
    }
    //////////////////////////////////////OFERTAS DE TOKENS
    
    //////////////////////////////////////MIS DE TOKENS    
    #[payable]
    pub fn extraer_token(&mut self, token_id: TokenId){
        // Verificar  si existe:
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() <= self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );
            //recuperar el token
            let _originaltoken = self
              .tokens
              .token_metadata_by_id.as_ref()
              .and_then(|by_id| by_id.get(&token_id))
              .unwrap();
              //recuperar el owner del token
            let token_owner_id = self.tokens.owner_by_id.get(&token_id);
         /*    assert_eq!(
                env::signer_account_id() != token_owner_id.as_ref().unwrap().to_string(),
                false,
                "no eres el dueño del token "
            );  */
            let contractaccount: AccountId = token_owner_id.as_ref().unwrap().clone();
            let  account: ValidAccountId = contractaccount.clone().try_into().unwrap(); 
            let msj: Option<String> = Some("withdraw succesfully,enjoy it! :)".to_string());
            //let msj2: String = Some("withdraw succesfully,enjoy it! :)".to_string());
             //   self.tokens.nft_approve(token_id.clone(),account.clone(),msj.clone());
            self.tokens.nft_transfer_call(account, token_id, None,msj, "".to_string());
        //   log!("transfer done");
    }
    #[payable]
    pub fn aproved_token(&mut self, token_id: TokenId){
        let token_owner_id = self.tokens.owner_by_id.get(&token_id);    
        let contractaccount: AccountId = token_owner_id.as_ref().unwrap().clone();
            let  account: ValidAccountId = contractaccount.clone().try_into().unwrap(); 
            let msj: Option<String> = Some("withdraw succesfully,enjoy it! :)".to_string());
            self.tokens.nft_approve(token_id.clone(),account.clone(),msj.clone());
    }
    
    pub fn remove_token_ext(&mut self, token_id: TokenId,collection_id:String, collection: String) -> String {
        //comprobar que el token exista
        assert_eq!(
            token_id.trim().parse::<u64>().unwrap() < self.tokens.owner_by_id.len(),
            true,
            "ese token no existe "
        );

        //comprobar que el revendedor sea el owner
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        assert_eq!(
            env::signer_account_id() == owner_id,
            true,
            "no eres el dueño del token "
        );
         //obtener los metadatos de ese token
         let mut originaltoken = self
         .tokens
         .token_metadata_by_id.as_ref()
         .and_then(|by_id| by_id.get(&token_id))
         .unwrap();
      
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();    
        //Se modifica el json
        extradatajson.status = "U".to_string();
        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
    
        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));

        //cambiar el numero de nfts disponibles
        self.n_token_on_sale += 1;
        
        let ext : String  ="".to_string()+&extradatajson.tags.to_string();  
        let graphdata = Thegraphstructure {
            contract_name: NATIVO_ACCOUNT.to_string(),
            collection:collection.clone().to_string(),
            collection_id: collection_id.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
            last_owner_id: "".to_string(),
            title : "".to_string(),
            description : "".to_string(),
            media : "".to_string(),
            creator : extradatajson.creator.to_string(),
            price : extradatajson.price.to_string(),
            status: extradatajson.status.to_string(), // sale status
            adressbidder: extradatajson.adressbidder.to_string(),
            highestbid: extradatajson.highestbidder.to_string(),
            lowestbid: extradatajson.lowestbidder.to_string(),
            expires_at: extradatajson.expires_at.to_string(),
            starts_at: extradatajson.starts_at.to_string(),
            extra: ext  ,
            
        };  
        let rett = serde_json::to_string(&graphdata).unwrap();
    return rett;
    }

    pub fn get_token(&self, token_id: TokenId) -> Meta {
        let metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();
        let token = Meta {
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
            title : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            media : metadata.media.unwrap_or_default().to_string(),
            tags: extradatajson.tags,
            creator : extradatajson.creator,
            price : extradatajson.price,
            status: extradatajson.status, // sale status
            adressbidder: extradatajson.adressbidder,
            highestbidder: extradatajson.highestbidder,
            lowestbidder:extradatajson.lowestbidder,
            expires_at: extradatajson.expires_at,
            starts_at: extradatajson.starts_at,
        };
        token
    }
    /// ///////////////////////////////////MIS DE TOKENS
   
    /// ///////////// CONTADORES TOTALES
    pub fn get_on_total_toks(&self) -> u64 {
        self.n_total_tokens
    }
    pub fn get_on_sale_toks(&self) -> u64 {
        self.n_token_on_sale
    }
    pub fn get_on_auction_toks(&self) -> u64 {
        self.n_token_on_auction
    }
    pub fn storage_byte_cost() -> Balance {
        env::storage_byte_cost()
    }
    //////////////// CONTADORES TOTALES

    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        // assert!(env::predecessor_account_id() == self.minter_account_id);
        //input is code:<Vec<u8> on REGISTER 0
        //log!("bytes.length {}", code.unwrap().len());
        const GAS_FOR_UPGRADE: u64 = 20 * TGAS; //gas occupied by this fn
        const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id().into_bytes();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_UPGRADE;
        unsafe {
            BLOCKCHAIN_INTERFACE.with(|b| {
                // Load input (new contract code) into register 0
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .input(0);

                //prepare self-call promise
                let promise_id = b
                    .borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_create(current_id.len() as _, current_id.as_ptr() as _);

                //1st action, deploy/upgrade code (takes code from register 0)
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

                // 2nd action, schedule a call to "migrate()".
                // Will execute on the **new code**
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_function_call(
                        promise_id,
                        migrate_method_name.len() as _,
                        migrate_method_name.as_ptr() as _,
                        0 as _,
                        0 as _,
                        0 as _,
                        attached_gas,
                    );
            });
        }
    }

/////////////////////METODOS DE MIGRACIÖN
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        log!("old state readed {}", old_state.n_total_tokens);
        Self {
            tokens:old_state.tokens,
            metadata: old_state.metadata,
            n_total_tokens:old_state.n_total_tokens,
            n_token_on_sale: old_state.n_token_on_sale,
            n_token_on_auction:old_state.n_token_on_auction,
            n_bid_per_token:old_state.n_bid_per_token,
        }
    }
/////////////////////METODOS DE MIGRACIÖN
}


near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}