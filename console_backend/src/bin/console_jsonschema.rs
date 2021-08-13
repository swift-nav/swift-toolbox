use schemars::schema_for;

use console_backend::{
    types::Result,
    ipc::{Message, self},
};

fn main() -> Result<()> {

    let schema = schema_for!(Message);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());


    let msg = Message::FileRequest(
        ipc::FileRequest { filename: "test.txt".into() });

    let s = serde_json::to_string(&msg).unwrap();
    eprintln!("{}", s);

    Ok(())
}