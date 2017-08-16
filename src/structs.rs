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



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product{
	#[serde(rename="@id")]
	pub id:Identifier,
	pub name: Option<String>,
	pub modified: Option<String>,
	pub program:Program,
	pub price:f32,
	pub currency:String,
	#[serde(rename="trackingLinks")]
	pub tracking_links:TrackingLinks,
	pub description: String,
	#[serde(rename="descriptionLong")]
	pub description_long: String,
	pub manufacturer: String,
	#[serde(rename="image")]
	pub images: ProductImages,
	#[serde(rename="priceOld")]
	pub price_old:Option<f32>,
	#[serde(rename="shippingCosts")]
	pub shipping_costs:Option<f32>,
	pub shipping:Option<f32>,
	#[serde(rename="merchantCategory")]
	pub merchant_category:Option<Value>,
	#[serde(rename="merchantProductId")]
	pub merchant_product_id:Value
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



