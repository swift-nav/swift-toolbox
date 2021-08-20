use std::io::{Cursor, Write};

use console_backend::{
    types::Result,
    ipc::{Message, self},
};

fn main() -> Result<()> {

    let msg = Message::FileRequest(
        ipc::FileRequest { filename: "test.txt".into() });

    let mut buf: Vec<u8> = vec![];
    let _ = bson::to_writer(&mut buf, &msg)?;

    let mut stdout = std::io::stdout();

    stdout.write_all(&buf[..])?;
    stdout.flush()?;
    
    let mut cursor = Cursor::new(&buf);
    let dec: Message = bson::from_reader(&mut cursor)?;

    eprintln!("{:?}", dec);

    Ok(())
}