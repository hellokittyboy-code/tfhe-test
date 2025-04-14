use reqwest::Client;
use sui_sdk::{types::object, SuiClientBuilder};
use serde_json::json;



async fn start_test_client()-> Result<(), anyhow::Error>{
    // Sui testnet -- https://fullnode.testnet.sui.io:443
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {}", sui_testnet.api_version());
    let test_apis = sui_testnet.available_rpc_methods();
    print!("Testnet APIs: {:?}", test_apis);
    Ok(())
}
async fn start_dev_client()-> Result<(), anyhow::Error>{
     // Sui devnet -- https://fullnode.devnet.sui.io:443
     let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
     println!("Sui devnet version: {}", sui_devnet.api_version());
 
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    

    
    // Sui mainnet -- https://fullnode.mainnet.sui.io:443
    let sui_mainnet = SuiClientBuilder::default().build_mainnet().await?;
    println!("Sui mainnet version: {}", sui_mainnet.api_version());
    let main_apis = sui_mainnet.available_rpc_methods();
    print!("MainNet APIs: {:?}", main_apis);
    
    // 创建一个HTTP客户端
    let client = Client::new();
    


    // 定义一个变量来存储对象ID
    let object_id = "0x47ca2248bee2de9f44ea5c324f409763d29f56e5b08d5e849d03a2c101454717";
    
    // 发送POST请求
    let response = client.post("https://fullnode.mainnet.sui.io:443")
        .header("Content-Type", "application/json")
        .body(format!(r#"{{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_getObject",
            "params": [
                "{}",
                {{
                    "showType": true,
                    "showOwner": true,
                    "showPreviousTransaction": true,
                    "showDisplay": false,
                    "showContent": true,
                    "showBcs": false,
                    "showStorageRebate": true
                }}
            ]
        }}"#, object_id))
        .send()
        .await?;

    // 打印响应状态
    println!("Response Status: {}", response.status());
    // 打印响应内容
    let response_text = response.text().await?;
    println!("Response Body: {}", response_text);

    // 解析JSON响应
    let json_response: serde_json::Value = serde_json::from_str(&response_text)?;
    if let Some(fields) = json_response["result"]["data"]["content"]["fields"].as_object() {
        println!("Fields: {:?}", fields);
        // 提取具体字段
        if let Some(creator) = fields.get("creator").and_then(|v| v.as_str()) {
            println!("Creator: {}", creator);
        }
        if let Some(reserve_x) = fields.get("reserve_x").and_then(|v| v.as_str()) {
            println!("Reserve X: {}", reserve_x);
        }
        if let Some(reserve_y) = fields.get("reserve_y").and_then(|v| v.as_str()) {
            println!("Reserve Y: {}", reserve_y);
        }
        // 可以根据需要提取更多字段
    }

    Ok(())
}



