#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
extern crate url;
pub mod structs;
use  tokio_core::reactor::Handle;
use structs::*;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use futures::future::Future;
use hyper::{Client,StatusCode};
use std::clone::Clone;
use futures::Stream;
use url::Url;
const ALL_PRODUCTS_URL:&'static str="http://api.zanox.com/json/2011-03-01/products";

#[derive(Debug)]
pub struct ZanoxClient{
 client:Client<HttpsConnector<HttpConnector>>,
 connect_id:String
}

impl Clone for  ZanoxClient{
    fn clone(&self) -> ZanoxClient{
        ZanoxClient{
        client:self.client.clone(),
        connect_id:self.connect_id.clone()
    }
}

    fn clone_from(&mut self, source: &Self) { 
        self.client=source.client.clone();
        self.connect_id=source.connect_id.clone();
     }
}


#[derive(Debug, Clone)]
pub enum ZanoxClientError{
	SendError(String),
	ResponseReadError(String),
	DeserializationError(String),
	ServerError(StatusCode,String)
}

impl ZanoxClient{
    pub fn new(connect_id:String,handle:&Handle)->ZanoxClient{    
        let client = hyper::Client::configure().connector(HttpsConnector::new(4, handle).unwrap()).build(handle);
         ZanoxClient{
            client:client,
            connect_id:connect_id
        }
    }

    pub fn get_products(&self)->ZanoxAPIRequest{
        ZanoxAPIRequest{
            client:self,
            url:Url::parse(&format!("{}?connectid={}",ALL_PRODUCTS_URL,self.connect_id)).unwrap()
        }
    }

    fn get_products_using(&self,url:String)->Box<Future<Item=ProductsPage, Error=ZanoxClientError>>{
        info!("Fetching products using URL {:?}",url);
        let uri = url.parse::<hyper::Uri>().unwrap();

       let out=self.client.get(uri).map_err(|e|{
            ZanoxClientError::SendError(format!("{}",e))
        }).and_then(|r|{
            let status:StatusCode=r.status();
            r.body().concat2().map_err(|e|{
                ZanoxClientError::ResponseReadError(format!("{}",e))
            }).map(move |d|{(status,d)})
        }).and_then(|(status,data)|{
           String::from_utf8((&data).to_vec()).map_err(|e|{
                ZanoxClientError::ResponseReadError(format!("{}",e))
            }).map(move |str|{
                (status,str)
            })
        }).and_then(|(status,str)|{
            match status{
                StatusCode::Ok => Ok(str),
                _ => {
                    Err(ZanoxClientError::ServerError(status,str))
                }
            }
        }).and_then(|str|{
             let out:Result<ProductsPage,ZanoxClientError>=serde_json::from_str(&str).map_err(|e|{
                ZanoxClientError::DeserializationError(format!("{} {:?}",e,str))
            });
             out
        });
        Box::new(out)
    }
}
#[derive(Debug, Clone)]
pub enum SearchType{
    Phrase,
    Contextual
}
#[derive(Debug, Clone)]
pub enum PartnershipType{
    All,
    Confirmed
}
#[derive(Debug, Clone)]
pub struct ZanoxAPIRequest<'a>{
    client:&'a ZanoxClient,
    url:Url
}

impl <'a> ZanoxAPIRequest<'a>{
    pub fn run(self)->Box<Future<Item=ProductsPage, Error=ZanoxClientError>>{
        self.client.get_products_using(self.url.into_string())
    }

    fn param(mut self,key:&str,value:&str)->ZanoxAPIRequest<'a>{
        self.url.query_pairs_mut().append_pair(key,value);
        self
    }

    pub fn page(self,page:i32)->ZanoxAPIRequest<'a>{
        self.param("page",&page.to_string())
    }

    pub fn items(self,items:i32)->ZanoxAPIRequest<'a>{
        self.param("items",&items.to_string())
    }
    
    pub fn min_price(self,min_price:i32)->ZanoxAPIRequest<'a>{
        self.param("minprice",&min_price.to_string())
    }

    pub fn max_price(self,max_price:i32)->ZanoxAPIRequest<'a>{
        self.param("maxprice",&max_price.to_string())
    }

    pub fn adspace(self,adcpaceid:i32)->ZanoxAPIRequest<'a>{
        self.param("adspace",&adcpaceid.to_string())
    }

    pub fn ean(self,ean:i32)->ZanoxAPIRequest<'a>{
        self.param("ean",&ean.to_string())
    }

    pub fn programs(self,programs:&str)->ZanoxAPIRequest<'a>{
        self.param("programs",programs)
    }

    pub fn merchant_category(self,merchant_category:&str)->ZanoxAPIRequest<'a>{
        self.param("merchantcategory",merchant_category)
    }

    pub fn has_images( self,flag:bool)->ZanoxAPIRequest<'a>{
        self.param("hasimages",&flag.to_string())
    }

    pub fn query( self,query:&str)->ZanoxAPIRequest<'a>{
        self.param("q",query)
    }

    pub fn region( self,region:&str)->ZanoxAPIRequest<'a>{
        self.param("region",region)
    }

    pub fn search_type( self,search_type:SearchType)->ZanoxAPIRequest<'a>{
        self.param("searchtype",&format!("{:?}",search_type))
    }
    pub fn partnership( self,partnership:PartnershipType)->ZanoxAPIRequest<'a>{
        self.param("partnership",&format!("{:?}",partnership))
    }
}