use std::error::Error;
use anyhow::Result;

#[tokio::test]
async fn test_quick_devs()  -> Result<()> {
    let hc = httpc_test::new_client("http://0.0.0.0:3000");
    hc?.do_get("/").await?.print().await?;
    //
    //
    Ok(())

}