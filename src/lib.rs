
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::sync::{Mutex};
use lazy_static::lazy_static;
use near_sdk::collections::LazyOption;
use substring::Substring;

use near_sdk::json_types::{ValidAccountId,Base64VecU8};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,Gas,ext_contract
};
use serde_json::json;
use std::convert::TryInto;

near_sdk::setup_alloc!();
/// Balance is type for storing amounts of tokens.
pub type Balance = u128;
 
#[derive(BorshDeserialize, BorshSerialize )]
pub struct OldContract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    minter_account_id: AccountId,
    market_account_id: AccountId,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
    n_chunks: u64,
    tokenHM:HashMap<TokenId, Vec<String>>,
    tk_chunk:Vec<HashMap<TokenId, Vec<String>>>,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize )]
pub struct Contract {
    minter_account_id: AccountId,
    market_account_id: AccountId,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_total_tokens: u64,
    n_token_on_sale: u64,
    n_token_on_auction: u64,
    n_chunks: u64,
    tokenHM:HashMap<TokenId, Vec<String>>,
    tk_chunk:Vec<HashMap<TokenId, Vec<String>>>,
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
    token_id : String,
    owner_id : String,
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
 
lazy_static! {
    static ref USER_TOKEN_HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref CONV_MAP: HashMap<String, String> = {
        let mut map = HashMap::new();
        
        map
    };
   

}

//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(ext_nft)]
trait NonFungibleToken {
    // view method
    fn getPromiseResult(&self  ) ;
    fn saveToTheGraph(&self, info: String) ;

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
         minter_account_id: "dev-1644523323613-61099606761670".to_string(),
         market_account_id: "dev-1644433845094-13612285357489".to_string(),
         n_total_tokens: 0,
         n_token_on_sale: 0,
         n_token_on_auction: 0,
         n_chunks: 0,
         tokenHM:HashMap::new(),
         tk_chunk:Vec::new(),
         
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
    pub fn new_default_meta(owner_id: ValidAccountId, minter_account_id: AccountId, market_account_id: AccountId,) -> Self {
        Self::new(
            owner_id,
            minter_account_id,
            market_account_id,
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
    pub fn new(owner_id: ValidAccountId, market_account_id: AccountId, minter_account_id: AccountId,  metadata: NFTContractMetadata) -> Self {
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
            market_account_id: market_account_id,
            minter_account_id: minter_account_id,
            n_total_tokens: 0,
            n_token_on_sale: 0,
            n_token_on_auction: 0,
            n_chunks: 0,
            tokenHM:HashMap::new(),
            tk_chunk:Vec::new(),
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
    pub fn mint_token(&mut self,token_owner_id: ValidAccountId,collection:String,token_metadata: TokenMetadata) ->String    {
        let market_account: ValidAccountId = self.market_account_id.clone().try_into().unwrap();
        let minter_account: ValidAccountId = self.minter_account_id.clone().try_into().unwrap();
       
       let token_id: TokenId =self.n_total_tokens.to_string();
       let mut info:Vec<String>=Vec::new();
       assert_eq!(
        token_metadata.media != None,
        true,
        "media del token vacia "
            );
            let mined= self.tokens.mint(
            token_id.clone(),
            minter_account.clone(),    
            Some(token_metadata.clone())
        );  
        
        self.tokens
        .internal_transfer_unguarded(&token_id, &minter_account.to_string(), &token_owner_id.to_string());
      
        self.n_total_tokens  +=1;
        self.n_token_on_sale += 1;
        //log!("{}",&token_metadata.extra.as_ref().unwrap().to_string());
        let newextradata = str::replace(&token_metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();  
       
       let ext : String  ="".to_string()+&extradatajson.tags.to_string();
          
          let mut graphdata = Thegraphstructure {
            contract_name: minter_account.clone().to_string(),
            collection:collection.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : extradatajson.creator.to_string(),
            title : token_metadata.title.as_ref().unwrap().to_string(),
            description : token_metadata.description.as_ref().unwrap().to_string(),
            media : token_metadata.media.as_ref().unwrap().to_string(),
            creator : extradatajson.creator.to_string(),
            price : extradatajson.price.to_string(),
            status: "S".to_string(), // sale status PENDIENTE
            adressbidder: extradatajson.adressbidder.to_string(),
            highestbid: extradatajson.highestbidder.to_string(),
            lowestbid: extradatajson.lowestbidder.to_string(),
            expires_at: extradatajson.expires_at.to_string(),
            starts_at: extradatajson.starts_at.to_string(),
            extra: ext  ,
            
        };  
        
     let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string(); 
    
     let p = ext_nft::saveToTheGraph(
        rett.clone(),
        &market_account.to_string(), //  account_id as a parameter
        env::attached_deposit(), // yocto NEAR to attach
        25_000_000_000_000 // gas to attach
     );

    return rett;    
    }
    #[payable]
    pub fn mint_token_ext(&mut self,token_owner_id: ValidAccountId,collection:String,token_metadata: TokenMetadata) ->String     {
        let market_account: ValidAccountId = self.market_account_id.clone().try_into().unwrap();
        let minter_account: ValidAccountId = self.minter_account_id.clone().try_into().unwrap();
       
       let token_id: TokenId =self.n_total_tokens.to_string();
       let mut info:Vec<String>=Vec::new();
       assert_eq!(
        token_metadata.media != None,
        true,
        "media del token vacia "
            );
            let mined= self.tokens.mint(
            token_id.clone(),
            minter_account.clone(),    
            Some(token_metadata.clone())
        );  
        
        self.tokens
        .internal_transfer_unguarded(&token_id, &minter_account.to_string(), &token_owner_id.to_string());
      
        self.n_total_tokens  +=1;
        self.n_token_on_sale += 1;

        let newextradata = str::replace(&token_metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();  
       
       let ext : String  ="".to_string()+&extradatajson.tags.to_string();
          
          let mut graphdata = Thegraphstructure {
            contract_name: minter_account.clone().to_string(),
            collection: collection.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : extradatajson.creator.to_string(),
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
            extra: ext  ,
            
        };  
           
     let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string();

     return rett;
    }
    //////////////////////////////////////CREACION DE TOKENS
    
    //////////////////////////////////////COMPRA/VENTA DE TOKENS
    #[payable]
    pub fn buy_token(&mut self, token_id: TokenId, collection: String) -> String {
        let market_account: ValidAccountId = self.market_account_id.clone().try_into().unwrap();
        let minter_account: ValidAccountId = self.minter_account_id.clone().try_into().unwrap();

        //asegurarnos de que el numero sea positivo y este dentro el rango de tokens minados
        //let token_id_u64 = token_id.parse::<u64>().unwrap();
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
            let mut metadataextra = Contract::get_token(self, token_id.clone());
      
        let amount = env::attached_deposit();
        assert_eq!(
            metadataextra.price.parse::<u128>().unwrap(),
            amount,
            "Cantidad incorrecta,verifica el costo exacto!"
        );
        assert_eq!(
            metadataextra.status,
            "S".to_string(), 
            "no esta a la venta"
        );
        
        //revisar que este a la venta
        //obtener el dueño del token
        //let token_owner_id = self.tokens.owner_by_id.get(token_id).unwrap();
        //obtener el creador del token
        let creator_id = metadataextra.creator;
        //obtener el comprador del token
        let buyer_id = &env::signer_account_id();

        // el dueñp no puede comprar su propio token
        assert_eq!(buyer_id == &owner_value, false, "eres el dueño del token ");
         //obtener la regalia,la comision de Nativo y el pagoa al autor del token
         let mut  res:f64=0.0;
         let mut  roy:f64=0.0;
         let mut  gains:f64=0.0;
         let mut  pay:f64=0.0;
         roy = amount as f64 *0.10;
         gains=amount as f64 *0.03;
         pay=amount as f64 *0.87;
       
        //cambiar la metadata
        
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();   
        //Se modifica el json
        extradatajson.status = "U".to_string();
        // se convierte el Json a un String plano
        let mut extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");

        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //transferir los nears
        //TODO: entender como consultar si la transferencia fue exitosa

        Promise::new(owner_value.clone().to_string()).transfer(pay as  u128);
        //TODO: transferir la regalia del token
        Promise::new(creator_id.clone()).transfer(roy as u128);
        //TODO: transferir la regalia del token
        Promise::new(market_account.clone().to_string()).transfer(gains as u128);
        //transferir el nft
        self.tokens
            .internal_transfer_unguarded(&token_id, &owner_value.to_string(), buyer_id);

        //cambiar el numero de nfts disponibles
        self.n_token_on_sale -= 1;
         
        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
          
          let mut graphdata = Thegraphstructure {
            contract_name: minter_account.clone().to_string(),
            collection:collection.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : buyer_id.to_string(),
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
        
     let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string(); 
    
     let p = ext_nft::saveToTheGraph(
        rett.clone(),
        &market_account.to_string(), //  account_id as a parameter
        env::attached_deposit(), // yocto NEAR to attach
        25_000_000_000_000 // gas to attach
     );

    return rett;
    }

    #[payable]
    pub fn buy_token_ext(&mut self, token_id: TokenId, collection:String) -> String {
        let market_account: ValidAccountId = self.market_account_id.clone().try_into().unwrap();
        let minter_account: ValidAccountId = self.minter_account_id.clone().try_into().unwrap();

        //asegurarnos de que el numero sea positivo y este dentro el rango de tokens minados
        //let token_id_u64 = token_id.parse::<u64>().unwrap();
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
            let mut metadataextra = Contract::get_token(self, token_id.clone());
      
        let amount = env::attached_deposit();
        assert_eq!(
            metadataextra.price.parse::<u128>().unwrap(),
            amount,
            "Cantidad incorrecta,verifica el costo exacto!"
        );
        assert_eq!(
            metadataextra.status,
            "S".to_string(), 
            "no esta a la venta"
        );
        
        //revisar que este a la venta
        //obtener el dueño del token
        //let token_owner_id = self.tokens.owner_by_id.get(token_id).unwrap();
        //obtener el creador del token
        let creator_id = metadataextra.creator;
        //obtener el comprador del token
        let buyer_id = &env::signer_account_id();

        // el dueñp no puede comprar su propio token
        assert_eq!(buyer_id == &owner_value, false, "eres el dueño del token ");
         //obtener la regalia,la comision de Nativo y el pagoa al autor del token
         let mut  res:f64=0.0;
         let mut  roy:f64=0.0;
         let mut  gains:f64=0.0;
         let mut  pay:f64=0.0;
         roy = amount as f64 *0.10;
         gains=amount as f64 *0.03;
         pay=amount as f64 *0.87;

     //   log!("{}",&originaltoken.extra.as_ref().unwrap().to_string());
       
        //cambiar la metadata        
        //se  reemplaza los ' por \" en un string plano                "'", "\""
        let newextradata = str::replace(&originaltoken.extra.as_ref().unwrap().to_string(), "'", "\"");
        //el string plano se convierte a JSon
        let mut extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();   
        //Se modifica el json
        extradatajson.status = "U".to_string();

        // se convierte el Json a un String plano
        let mut extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");

        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //transferir los nears
        //TODO: entender como consultar si la transferencia fue exitosa

        Promise::new(owner_value.clone().to_string()).transfer(pay as  u128);
        //TODO: transferir la regalia del token
        Promise::new(creator_id.clone()).transfer(roy as u128);
        //TODO: transferir la regalia del token
        Promise::new(market_account.clone().to_string()).transfer(gains as u128);
        //transferir el nft
        self.tokens
            .internal_transfer_unguarded(&token_id, &owner_value.to_string(), buyer_id);

        //cambiar el numero de nfts disponibles
        self.n_token_on_sale -= 1;

        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
         
        let mut graphdata = Thegraphstructure {
            contract_name: minter_account.clone().to_string(),
            collection: collection.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : buyer_id.to_string(),
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
        
        let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string();
        
    return rett;
    }
   
    pub fn sell_token(&mut self, token_id: TokenId, price: String, collection: String ) -> String {
        let market_account: ValidAccountId = self.market_account_id.clone().try_into().unwrap();
        let minter_account: ValidAccountId = self.minter_account_id.clone().try_into().unwrap();
        //comprobar que el token exista
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
        assert_eq!(
            extradatajson.status,
            "A".to_string(),
            "Lo sentimos,este token se encuentra en subasta y aun no termina!"
        );
        if price.trim().parse::<u128>().unwrap() > 0 {
            extradatajson.price =  price.clone();
        }
        extradatajson.status = "S".to_string();

        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
      //    log!("{}",&finalextrajson.to_string());
        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //cambiar el numero de nfts disponibles
        self.n_token_on_sale += 1;
        
        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
          
          let mut graphdata = Thegraphstructure {
            contract_name: minter_account.clone().to_string(),
            collection:collection.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
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
        
     let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string(); 
    
     let p = ext_nft::saveToTheGraph(
        rett.clone(),
        &market_account.to_string(), //  account_id as a parameter
        env::attached_deposit(), // yocto NEAR to attach
        25_000_000_000_000 // gas to attach
     );

    return rett;
    }

    pub fn sell_token_ext(&mut self, token_id: TokenId, price: String, collection: String ) -> String {
        let market_account: ValidAccountId = self.market_account_id.clone().try_into().unwrap();
        let minter_account: ValidAccountId = self.minter_account_id.clone().try_into().unwrap();
        //comprobar que el token exista
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
        // assert_eq!(
        //     extradatajson.status,
        //     "A".to_string(),
        //     "Lo sentimos,este token se encuentra en subasta y aun no termina!"
        // );
        if price.trim().parse::<u128>().unwrap() > 0 {
            extradatajson.price =  price.clone();
        }
        extradatajson.status = "S".to_string();

        // se convierte el Json a un String plano
        let extradatajsontostring  = serde_json::to_string(&extradatajson).unwrap();          // se  reemplaza los " por \' en un string plano
        let finalextrajson = str::replace(&extradatajsontostring.to_string(),"\"","'");
      //    log!("{}",&finalextrajson.to_string());
        originaltoken.extra = Some(finalextrajson);
        //remplazamos la metadata
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &originaltoken));
        //cambiar el numero de nfts disponibles
        self.n_token_on_sale += 1;
        
        let ext : String  ="".to_string()+&extradatajson.tags.to_string();
          
          let mut graphdata = Thegraphstructure {
            contract_name: minter_account.clone().to_string(),
            collection:collection.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
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
        
     let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string();

    return rett;
    }
     //////////////////////////////////////COMPRA/VENTA DE TOKENS

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
            let mut  originaltoken = self
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
            let Contractaccount: AccountId = token_owner_id.as_ref().unwrap().clone();
            let  account: ValidAccountId = Contractaccount.clone().try_into().unwrap(); 
            let msj: Option<String> = Some("withdraw succesfully,enjoy it! :)".to_string());
            //let msj2: String = Some("withdraw succesfully,enjoy it! :)".to_string());
            //let apro: Option<u64> = Some(apro);
             //   self.tokens.nft_approve(token_id.clone(),account.clone(),msj.clone());
            self.tokens.nft_transfer_call(account, token_id, None,msj, "".to_string());
        //   log!("transfer done");
    }
    #[payable]
    pub fn aproved_token(&mut self, token_id: TokenId){
        let token_owner_id = self.tokens.owner_by_id.get(&token_id);    
        let Contractaccount: AccountId = token_owner_id.as_ref().unwrap().clone();
            let  account: ValidAccountId = Contractaccount.clone().try_into().unwrap(); 
            let msj: Option<String> = Some("withdraw succesfully,enjoy it! :)".to_string());
            let apro: Option<u64> = Some(0);
            self.tokens.nft_approve(token_id.clone(),account.clone(),msj.clone());
    }
      
    pub fn remove_token(&mut self, token_id: TokenId, collection: String) -> String {
        let market_account: ValidAccountId = self.market_account_id.clone().try_into().unwrap();
        let minter_account: ValidAccountId = self.minter_account_id.clone().try_into().unwrap();

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
          
          let mut graphdata = Thegraphstructure {
            contract_name: minter_account.clone().to_string(),
            collection:collection.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
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
        
     let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string(); 
    
     let p = ext_nft::saveToTheGraph(
        rett.clone(),
        &market_account.to_string(), //  account_id as a parameter
        env::attached_deposit(), // yocto NEAR to attach
        25_000_000_000_000 // gas to attach
     );

    return rett;
    }

    pub fn remove_token_ext(&mut self, token_id: TokenId, collection: String) -> String {
        let market_account: ValidAccountId = self.market_account_id.clone().try_into().unwrap();
        let minter_account: ValidAccountId = self.minter_account_id.clone().try_into().unwrap();

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
          
          let mut graphdata = Thegraphstructure {
            contract_name: minter_account.clone().to_string(),
            collection:collection.clone().to_string(),
            token_id : token_id.to_string(),
            owner_id : owner_id.to_string(),
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
        
     let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string();

    return rett;
    }

    pub fn get_token(&self, token_id: TokenId) -> Meta {
        
        let mut metadata = self
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
            //culture : extradatajson.culture,
            //country : extradatajson.country,
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
    pub fn get_n_in_chuck(&self,chunk:usize) -> usize {
        return  self.tk_chunk[chunk].len() ;
      }
    pub fn storage_byte_cost() -> Balance {
        env::storage_byte_cost()
    }
    //////////////// CONTADORES TOTALES
    
    pub fn update_token(&mut self, token_id: TokenId, extra: String) -> TokenMetadata {
        //assert!(!env::state_exists(), "Already initialized");
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        let owner_id = self.tokens.owner_by_id.get(&token_id).unwrap();
        //assert_eq!(owner_id!= env::signer_account_id() && owner != ,false,"");
        metadata.extra = Some(extra);
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata));
        metadata
    }

/////////////////////METODOS DE MIGRACIÖN
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        log!("old state readed {}", old_state.n_total_tokens);
        Self {
            minter_account_id:old_state.minter_account_id,
            market_account_id:old_state.market_account_id,
            tokens:old_state.tokens,
            metadata: old_state.metadata,
            n_total_tokens:old_state.n_total_tokens,
            n_token_on_sale: old_state.n_token_on_sale,
            n_token_on_auction:old_state.n_token_on_auction,
            n_chunks: old_state.n_chunks,
            tokenHM:old_state.tokenHM,
            tk_chunk:old_state.tk_chunk,

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