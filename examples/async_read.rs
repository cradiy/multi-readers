#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    read().await;
}

#[cfg(feature = "async")]
async fn read() {
    use std::io::Cursor;

    use multi_readers::wrap;
    use tokio::io::AsyncReadExt;
    let cur1 = Cursor::new("Hello ");
    let cur2 = Cursor::new("world");
    let mut readers = wrap!(cur1, cur2);
    let mut buf = String::new();
    readers.read_to_string(&mut buf).await.unwrap();
    assert_eq!(buf.as_str(), "Hello world");
}
#[cfg(not(feature = "async"))]
fn main() {}
