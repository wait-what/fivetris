use std::fs::File;
use std::io::{BufWriter, Write};
use asset_store::AssetStore;

fn main() {
    let mut file = File::create("./assets.tar.bz2").unwrap();
    let mut buf_writer = BufWriter::new(&mut file);

    let asset_store = AssetStore::new_from("./assets").unwrap();
    asset_store.serialize(&mut buf_writer).unwrap();

    buf_writer.flush().unwrap();
    let file = buf_writer.into_inner().unwrap();
    file.flush().unwrap();
}
