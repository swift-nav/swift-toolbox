use std::io::{Cursor, Write};

use console_backend::{
    types::Result,
    ipc::{Message, self},
};

fn main() -> Result<()> {

    let msg = Message::FileRequest(
        ipc::FileRequest { filename: "test.txt".into() });

    let mut buf: Vec<u8> = vec![];
    let _ = ciborium::ser::into_writer( &msg, &mut buf)?;

    let mut stdout = std::io::stdout();

    stdout.write_all(&buf[..])?;
    stdout.flush()?;

    let mut cursor = Cursor::new(&buf);
    let dec: Message = ciborium::de::from_reader(&mut cursor)?;

    eprintln!("{:?}", dec);

    Ok(())
}