
extern crate zanox_api_client;
extern crate tokio_core;
extern crate pretty_env_logger;
use tokio_core::reactor::Core;
use zanox_api_client::{ZanoxClient,SearchType};

const CONNECT_ID:&'static str="43EEF0445509C7205827"; // DO NOT WORRY, THAT IS CONNECT ID FROM ZANOX DOCUMENTATION

#[test]
fn should_download_products(){   
	pretty_env_logger::init().unwrap();
	let mut core = Core::new().unwrap();
    let handle = core.handle();
	let client=ZanoxClient::new(String::from(CONNECT_ID),&handle);
	let work=client.get_products().page(2).items(10).has_images(true).query("nike").search_type(SearchType::Contextual).programs("7408").run();
	let products=core.run(work).unwrap();
	assert_eq!(products.page,2);
	assert_eq!(products.items,10);
	assert_eq!(products.products.product_item.len(),10);
}