use dotenv::dotenv;
use std::env;

extern crate aws_sdk_dynamodb as dynamodb;
#[allow(unused_imports)]
extern crate tokio_stream;

#[path = "./geo_data.rs"]
mod geo_data;

use dynamodb::{config, Client, Credentials, Region, Error};
use regex::Regex;
use tokio_stream::StreamExt;
use geo_data::{Destination,creating_boundaries};

//This is for debugging purposes

#[allow(dead_code)]
fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(unreachable_code)]
fn get_aws_client() -> Result<Client, ()>{
    dotenv().ok();

    let key_id = env::var("AWS_ACCESS_KEY_ID").unwrap();
    let key_secret = env::var("AWS_SECRET_ACCESS_KEY").unwrap();
    let region = env::var("REGION").unwrap();

    let credentials = Credentials::new(key_id, key_secret, None, None, "The journey begins now" );
    let region = Region::new(region);

    let config_builder = config::Builder::new().region(region).credentials_provider(credentials);
    let config = config_builder.build();

    let client = Client::from_conf(config);

    return Ok(client);
    
    todo!();

}
#[allow(dead_code)]
async fn list_items(client: &Client, table: &str) -> Result<String, Error> {
    let mut item_storage : String = String::from("");
    let items: Result<Vec<_>, _> = client
        .scan()
        .table_name(table)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;

    for item in items? {
        item_storage = item.get("Area").unwrap().as_s().unwrap().clone();
    }

    return Ok(item_storage);
}

async fn list_items_vec(client: &Client, table: &str) -> Result<Vec<(String, String, String)>, Error> {
    let mut item_storage : Vec<(String, String, String)> = vec![];
    let items: Result<Vec<_>, _> = client
        .scan()
        .table_name(table)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;

    for item in items? {
        item_storage.push((item.get("Name").unwrap().as_s().unwrap().to_string(), 
        item.get("Area").unwrap().as_s().unwrap().to_string(), 
        item.get("ObjectPath").unwrap().as_s().unwrap().to_string()));
    }

    return Ok(item_storage);
}



#[allow(dead_code)]
fn string_to_vec(input : &mut String) -> Vec<&str> {
    let input_str = input.as_str();
    let mut storage : Vec<&str> = vec![];
    let re = Regex::new("-?[0-9]+.[0-9]+,-[0-9]+.[0-9]+").unwrap();
    for cap in re.captures_iter(input_str) {
        storage.push(&cap.get(0).unwrap().as_str());
    }
    return storage; 
}

pub async fn db_main () -> Result<Vec<(String, String, String)>, Error> {

    let client_dynamo = get_aws_client().unwrap();
    let paginator = client_dynamo.list_tables().into_paginator().items().send();

    let _table_names = paginator.collect::<Result<Vec<_>, _>>().await?;

    let items = list_items_vec(&get_aws_client().unwrap(),"Journey-destinations").await?;

    return Ok(items);
    
}