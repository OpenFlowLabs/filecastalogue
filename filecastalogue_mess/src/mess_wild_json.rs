use serde_json::{Result, Value};

fn get_wild_index_data(input_data: &str) -> Result<Value> {
    Ok(serde_json::from_str(input_data)?)
}

fn main() {
    let mock_json_input =
r#"{
    "/etc/crontab": {
        "exists": {
            "hash": "blake3:8970ec4b3a6ea2bc7428dd679edcfd012fb0db23a17af829ff24129cc9f1ba9e",
            "attributes": {
                "posix_user": "root",
                "posix_group": "root"
            }
        }
    },
    "/etc/nginx/nginx.conf": {
        "exists": {
            "hash": "blake3:48c3c2f67bc0245370f7891c0ae536cc06af39c1592b82f91c6d2fea758e14e3",
            "attributes": {
                "posix_user": "nginx",
                "posix_group": "nginx",
                "posix_mode": "0644"
            }
        }
    },
    "/etc/resolv.conf": {
        "doesnt_exist": {}
    }
}"#;

    let wild_index_data: Value = match get_wild_index_data(mock_json_input) {
        Ok(x) => x,
        Err(e) => panic!("ERROR: {}", e)
    };
    println!("{}", wild_index_data["/etc/crontab"] );
    let reserialized_wild_index_string = match serde_json::to_string(&wild_index_data) {
        Ok(x) => x,
        Err(e) => panic!("ERROR: {}", e)
    };
    println!("{}", reserialized_wild_index_string);
}