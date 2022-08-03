const DB_PATH: str = ""; //TODO set up a database file for later

struct Item {
    //TODO maybe implement stntrading shit here?
    name: String,
    category: String,
    is_craftable: bool,
    related_item: Item,
    mptf_price: u32,
    mptf_converted_price_ref: u32,
    mptf_converted_price_keys: u32,
    mptf_converted_price_pretty: (u16, u32),
    scrap_tf_price_ref: u32,
    scrap_tf_price_key: f32,
    scrap_tf_price_pretty: (u16, u32),
}

fn main() {}
