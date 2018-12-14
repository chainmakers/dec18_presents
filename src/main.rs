#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_aux;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use serde::de::Visitor;

use serde_aux::prelude::*;
use std::path::Path;
use std::io::Write;
use std::error::Error;
use serde::Serialize;
use serde::Serializer;

#[derive(Deserialize, Debug)]
pub struct Utxo {
    pub start_time: u32,
    pub addresses: Vec<Address>,
    pub total: f64,
    pub average: f64,
    pub utxos: u32,
    pub total_addresses: u32,
    pub start_height: u32,
    pub ending_height: u32,
    pub end_time: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub addr: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub amount: f64
}

fn main() {
    let path = "./snapshot.json";
    let file = File::open(path).expect(&format!("Could not open file: {}", path));
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    let _number_of_bytes: usize = match buf_reader.read_to_string(&mut contents) {
        Ok(num_bytes) => num_bytes,
        Err(_err) => 0
    };

    let mut snapshot: Utxo = serde_json::from_str(&contents).expect("Could not parse JSON");

    let snapshot = snapshot.addresses.iter()
        .filter(|address| address.amount >= 0.01)
        .collect::<Vec<_>>();

    println!("{:#?}", snapshot);

    let path = Path::new("./snapshot0.01.json");
    let mut file = File::create(&path).expect("Could not create file");

    let serialized = serde_json::to_string(&snapshot).expect("Could not serialize snapshot");

    match file.write_all(&serialized.as_bytes()) {
        Err(err) => panic!("Could not write to file: {}", err.description()),
        Ok(_) => println!("Successfully wrote file.")
    }
}
