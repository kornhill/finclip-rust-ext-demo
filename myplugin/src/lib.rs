use std::collections::HashMap;
use serde_json::json;

type FinClipCall = fn(&String) -> String;

fn api_drinker(input: &String) -> String {
    println!("invoked with parameter {}", input);

    let john = json!({
        "name": "john doe",
        "phones": "1234567"
    });

    john.to_string()
}

fn api_whisky(input: &String) -> String {
    println!("invoked with parameter {}", input);

    let brands = json!({
        "whisky": {
            "jack": "daniel",
            "johny": "walker",
            "henry": "Mckenna",
            "suntory": "toki"
        }
    });

    brands.to_string()
}

#[no_mangle]
pub unsafe extern "C" fn myplugin_register_apis() -> *mut HashMap<String, FinClipCall> {
    let mut map: HashMap<String, FinClipCall> = HashMap::new();
    map.insert("api_drinker".to_string(), api_drinker);
    map.insert("api_whisky".to_string(), api_whisky);

    Box::into_raw(Box::new(map))
}

#[no_mangle]
pub unsafe extern "C" fn myplugin_release(ptr: *mut HashMap<String, FinClipCall>) {
    if !ptr.is_null() {
        drop(Box::from_raw(ptr));
    }
}