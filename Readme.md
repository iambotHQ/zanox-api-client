zanox-api-client
--------------------

Sample usage:

```rust
extern crate zanox_api_client;
extern crate tokio_core;
use tokio_core::reactor::Core;
use zanox_api_client::{ZanoxClient,SearchType};

fn main(){   
	let mut core = Core::new().unwrap();
    	let handle = core.handle();
	let client=ZanoxClient::new(String::from("CONNECT_ID"),&handle);
	let work=client.get_products().page(2).items(10).has_images(true).query("nike").search_type(SearchType::Contextual).programs("7408").run();
	let products=core.run(work).unwrap();
}
```
