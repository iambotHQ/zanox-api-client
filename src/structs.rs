use serde::{Deserialize,Deserializer};
use serde_json::Value;
pub type Identifier=String;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program{
	#[serde(rename="@id")]
	pub id:Identifier,
	#[serde(rename="$")]
	pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackingLink{
	#[serde(rename="@adspaceId")]
	pub adspace_id:Identifier,
	pub ppv:Option<String>,
	pub ppc:Option<String>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackingLinks{
	#[serde(rename="trackingLink")]
	pub tracking_link: Vec<TrackingLink>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductImages{
	pub large:Option<String>,
	pub small:Option<String>,
	pub medium:Option<String>
}
fn parse_to_option_string<'de, D>(d: D) -> Result<Option<String>, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.map(|d:Value|{d.to_string()})
        }).or_else(|_|{Ok(None)})
}
fn parse_to_string<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Value| {
           x.to_string()
        }).or_else(|_|{Ok(String::new())})
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product{
	#[serde(rename="@id")]
	pub id:Identifier,
	pub name: Option<String>,
	pub modified: Option<String>,
	pub program:Program,
	#[serde(deserialize_with="parse_to_string")]
	pub price:String,
	pub currency:String,
	#[serde(rename="trackingLinks")]
	pub tracking_links:TrackingLinks,
	pub description: String,
	#[serde(rename="descriptionLong")]
	pub description_long: String,
	pub manufacturer: String,
	#[serde(rename="image")]
	pub images: ProductImages,
	#[serde(deserialize_with="parse_to_option_string")]
	#[serde(rename="priceOld")]
	pub price_old:Option<String>,
	#[serde(rename="shippingCosts")]
	#[serde(deserialize_with="parse_to_option_string")]
	pub shipping_costs:Option<String>,
	#[serde(deserialize_with="parse_to_option_string")]
	pub shipping:Option<String>,
	#[serde(rename="merchantCategory")]
	pub merchant_category:Option<String>,
	#[serde(rename="merchantProductId")]
	#[serde(deserialize_with="parse_to_option_string")]
	pub merchant_product_id:Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Products{
	#[serde(rename="productItem")]
	pub product_item:Vec<Product>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductsPage{
	pub page:i32,
	pub items:i32,
	pub total:i32,
	#[serde(rename="productItems")]
	pub products:Products
}



