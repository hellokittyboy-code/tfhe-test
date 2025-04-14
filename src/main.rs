use reqwest::Client;
use sui_sdk::SuiClientBuilder;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Sui testnet -- https://fullnode.testnet.sui.io:443
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {}", sui_testnet.api_version());
    let test_apis = sui_testnet.available_rpc_methods();
    print!("Testnet APIs: {:?}", test_apis);
    

     // Sui devnet -- https://fullnode.devnet.sui.io:443
    let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
    println!("Sui devnet version: {}", sui_devnet.api_version());

    // Sui mainnet -- https://fullnode.mainnet.sui.io:443
    let sui_mainnet = SuiClientBuilder::default().build_mainnet().await?;
    println!("Sui mainnet version: {}", sui_mainnet.api_version());
    let main_apis = sui_mainnet.available_rpc_methods();
    print!("MainNet APIs: {:?}", main_apis);
    
    // 创建一个HTTP客户端
    let client = Client::new();
    // 构造请求体
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sui_getNormalizedMoveStruct",
        "params": [
            "0xc95b9e341bc3aba1654bdbad707dcd773bd6309363447ef3fe58a960de92aa93",
            "module",
            "StructName"
        ]
    });

    // 发送POST请求
    let response = client.post("https://fullnode.mainnet.sui.io:443")
        .json(&request_body)
        .send()
        .await?;

    // 打印响应状态
    println!("Response Status: {}", response.status());
    // 打印响应内容
    let response_text = response.text().await?;
    println!("Response Body: {}", response_text);
    Ok(())
}
