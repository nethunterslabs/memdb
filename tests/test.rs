use memdb::Memdb;

use std::io;

#[async_std::test]
async fn set_get() -> io::Result<()> {
    let mut db = Memdb::open().await?;
    db.set("beep", "boop").await?;
    let val = db.get("beep").await?;
    assert_eq!(val, Some("boop".as_bytes().to_owned()));
    Ok(())
}

#[async_std::test]
async fn threaded_set_get() -> io::Result<()> {
    let db = Memdb::open().await?;

    let mut handle = db.clone();
    async_std::task::spawn(async move {
        handle.set("beep", "boop").await?;
        async_std::task::spawn(async move {
            let handle = handle.clone();
            let val = handle.get("beep").await?;
            assert_eq!(val, Some("boop".as_bytes().to_owned()));
            Ok::<(), std::io::Error>(())
        })
        .await?;
        Ok::<(), std::io::Error>(())
    })
    .await?;
    Ok::<(), std::io::Error>(())
}
