use std::fs::File;
use std::io::{BufWriter, BufReader, Write};
use asset_store::AssetStore;

fn main() {
    let mut file = File::create("./assets.tar.bz2").unwrap();
    let mut buf_writer = BufWriter::new(&mut file);

    let asset_store = AssetStore::new_from("./assets").unwrap();
    asset_store.serialize(&mut buf_writer).unwrap();

    buf_writer.flush().unwrap();
    buf_writer.into_inner().unwrap();

    let mut file = File::open("./assets.tar.bz2").unwrap();
    let buf_reader = BufReader::new(&mut file);
    let asset_store = AssetStore::deserialize(buf_reader).unwrap();

    println!("{:?}", asset_store.texture_positions);
}
