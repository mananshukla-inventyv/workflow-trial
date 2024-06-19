#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

// #[napi]
// pub fn sum(a: i32, b: i32) -> i32 {
//   a + b
// }

pub mod logger;
pub mod configration;
use logger::LoggerConfig;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tikv_client::{raw::Client, Config};
static TIKV: OnceCell<Client> = OnceCell::new();

#[derive(Clone, Serialize, Deserialize, Debug)]
#[napi(object)]
pub struct TikvConnParams {
    pub tlsclusterenabled: bool,
    pub sslcacerti: String,
    pub sslclientcerti: String,
    pub sslclientkeycerti: String,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[napi(object)]

pub struct BatchResponse {
    pub keys: Vec<String>,
    pub values: Option<Vec<String>>,
}

#[napi]
pub async fn init_client(tikv_conn_param: Option<TikvConnParams>) -> Result<String, napi::Error> {
    match create_client(tikv_conn_param).await{
        Ok(_res)=>Ok(String::from("Client Created")),
        Err(_error)=>Err(napi::Error::from_reason(String::from("Error in Client Creation!!")))
    }
}

pub async fn create_client(tikv_conn_param: Option<TikvConnParams>) -> Result<Client, napi::Error> {
    let cleint = TIKV.get();
    match cleint {
        Some(_client) =>
         {
          
          Ok(TIKV.get().unwrap().to_owned())
        },
        None => {
            let config = Config::default();
            let tls_cluster_enabled = false;
            println!("In tls cluster enable{}:-", tls_cluster_enabled);
            if tls_cluster_enabled {
                let with_security_config = config.to_owned().with_security(
                    tikv_conn_param.clone().unwrap().sslcacerti,
                    tikv_conn_param.clone().unwrap().sslclientcerti,
                    tikv_conn_param.clone().unwrap().sslclientkeycerti,
                );
                let client = Client::new_with_config(
                    vec![tikv_conn_param.clone().unwrap().host],
                    with_security_config,
                ).await;
                match client {
                    Ok(client) => {
                        let new_client: Client = client.with_atomic_for_cas();
                        TIKV.get_or_init(|| new_client.to_owned());
                        Ok(new_client)
                    }
                    Err(error) => Err(napi::Error::from_reason(error.to_string())),
                }
            } else {
                // let security = config.with_security("ca_path", "cert_path", "key_path");
                let client =
                    Client::new_with_config(vec![tikv_conn_param.unwrap().host], config).await;
                match client {
                    Ok(client) => {
                        let new_client: Client = client.with_atomic_for_cas();
                        TIKV.get_or_init(|| new_client.to_owned());
                        Ok(new_client)
                    }
                    Err(error) => Err(napi::Error::from_reason(error.to_string())),
                }
            }
        }
    }
}

#[napi]
pub fn startLogger() {
    // You can use handle to change logger config at runtime
    // just call startLogger() in main.rs and you can use log4rs in all your Project-crate.
    let Global_logs_config = LoggerConfig::create_Global_logs_config();
    let handle = log4rs::init_config(Global_logs_config).unwrap();
}


#[napi]
pub async fn get_single_record(
    key: String,
    project_name: Option<String>,
) -> Result<String, napi::Error> {
    let client = create_client(None).await;
    match client {
        Ok(client) => {
            let mut new_key = key;
            if !project_name.is_none() && project_name.as_ref().unwrap().len() > 0 {
                let res = get_project_level_key_with_global_prefix(
                    project_name.as_ref().unwrap(),
                    &new_key,
                );
                match res {
                    Ok(res) => {
                        new_key = res;
                    }
                    Err(error) => {
                        return Err(napi::Error::from_reason(error.to_string()));
                    }
                }
            }
            let value = client.get(new_key).await; // Returns a `tikv_client::Error` on failure.
            match value {
                Ok(value) => match value {
                    Some(value) => {
                        return Ok(String::from_utf8(value).unwrap());
                    }
                    None => {
                        Err(napi::Error::from_reason("Key Does Not Exits".to_string()))
                        // return Err("Key Does Not Exits".to_string());
                    }
                },
                Err(error) => {
                    return Err(napi::Error::from_reason(error.to_string()));
                }
            }
        }
        Err(error) => return Err(napi::Error::from_reason(error.to_string())),
    }
}

#[napi]
pub async fn add_single_record(
    key: String,
    value: String,
    old_value: Option<String>,
    project_name: Option<String>,
) -> Result<String, napi::Error> {
    let client = create_client(None).await;
    match client {
        Ok(client) => {
            let mut new_key = key.to_owned();
            if !project_name.is_none() && project_name.as_ref().unwrap().len() > 0 {
                let res = get_project_level_key_with_global_prefix(
                    project_name.as_ref().unwrap(),
                    &new_key,
                );
                match res {
                    Ok(res) => {
                        new_key = res;
                    }
                    Err(error) => {
                        return Err(napi::Error::from_reason(error.to_string()));
                    }
                }
            }
            if !old_value.is_none() && old_value.to_owned().unwrap().len() > 0 {
                let new_eqa = client
                    .compare_and_swap(
                        new_key.to_owned(),
                        Some(old_value.clone().unwrap().as_bytes().to_vec()),
                        value.as_bytes().to_vec(),
                    )
                    .await;
                match new_eqa {
                    Ok((_new_val, _flag)) => {
                        if !_flag {
                            return Err(napi::Error::from_reason("Could not Update".to_string()));
                        }
                        return Ok(String::from(
                            "Record Updated With CAS For Key".to_owned() + &key,
                        ));
                    }
                    Err(error) => {
                        return Err(napi::Error::from_reason(error.to_string()));
                    }
                }
            } else {
                let client_res = client.put(new_key.to_owned(), value).await; // Returns a `tikv_client::Error` on failure.
                match client_res {
                    Ok(_res) => {
                        return Ok(String::from("New Record Added With Key".to_owned() + &key));
                    }
                    Err(error) => {
                        return Err(napi::Error::from_reason(error.to_string()));
                    }
                }
            }
        }
        Err(error) => {
            return Err(napi::Error::from_reason(error.to_string()));
        }
    }
}

 
// pub async fn get_batch_using_scan(
//     start: String,
//     end: String,
//     batch_size: i32,
//     only_keys: bool,
//     project_name: Option<String>,
// ) -> Result<String, String> {
//     let client = create_client(None).await;
//     match client {
//         Ok(client) => {
//             let mut start_key = start;
//             let mut end_key = end.to_owned();
//             let mut prefix_value = "".to_string();
//             if !project_name.is_none() && project_name.as_ref().unwrap().len() > 0 {
//                 let start_key_with_project = get_project_level_key_with_global_prefix(
//                     project_name.as_ref().unwrap(),
//                     &start_key,
//                 );
//                 match start_key_with_project {
//                     Ok(start_key_with_project) => {
//                         let end_key_with_project = get_project_level_key_with_global_prefix(
//                             project_name.as_ref().unwrap(),
//                             &end_key,
//                         );
//                         match end_key_with_project {
//                             Ok(mut end_key_with_project) => {
//                                 start_key = start_key_with_project;
//                                 if end.len() == 0 {
//                                     end_key_with_project.push_str("~");
//                                 }
//                                 end_key_with_project.push_str("\\0");
//                                 end_key = end_key_with_project;
//                                 prefix_value = format!("k{}_", project_name.unwrap().to_lowercase())
//                             }
//                             Err(error) => {
//                                 return Err(napi::Error::from_reason(error.to_string()));
//                             }
//                         }
//                     }
//                     Err(error) => {
//                         return Err(napi::Error::from_reason(error.to_string()));
//                     }
//                 }
//             }
//             if only_keys {
//                 let scan_result = client
//                     .scan_keys((start_key..=end_key).into_inner(), batch_size as u32)
//                     .await;
//                 match scan_result {
//                     Ok(scan_result) => {
//                         let string_keys: Vec<String> = scan_result
//                             .iter()
//                             .map(|key| {
//                                 String::from_utf8_lossy(key.as_ref().into())
//                                     .to_string()
//                                     .replace(&prefix_value, "")
//                             })
//                             .collect();
//                         let res = serde_json::to_string(&BatchResponse {
//                             keys: string_keys,
//                             values: None,
//                         });
//                         match res {
//                             Ok(res) => {
//                                 return Ok(res);
//                             }
//                             Err(error) => return Err(napi::Error::from_reason(error.to_string())),
//                         }
//                     }
//                     Err(error) => return Err(napi::Error::from_reason(error.to_string())),
//                 }
//             } else {
//                 let scan = client
//                     .scan((start_key..=end_key).into_inner(), batch_size as u32)
//                     .await;
//                 match scan {
//                     Ok(scan) => {
//                         let string_keys: Vec<String> = scan
//                             .iter()
//                             .map(|key| {
//                                 String::from_utf8_lossy(key.0.as_ref().into())
//                                     .to_string()
//                                     .replace(&prefix_value, "")
//                             })
//                             .collect();
//                         let string_values: Vec<String> = scan
//                             .iter()
//                             .map(|key| String::from_utf8_lossy(&key.1.to_vec()).to_string())
//                             .collect();
//                         let res = serde_json::to_string(&BatchResponse {
//                             keys: string_keys,
//                             values: Some(string_values),
//                         });
//                         match res {
//                             Ok(res) => {
//                                 return Ok(res);
//                             }
//                             Err(error) => return Err(napi::Error::from_reason(error.to_string())),
//                         }
//                     }
//                     Err(error) => return Err(napi::Error::from_reason(error.to_string())),
//                 }
//             }
//         }
//         Err(error) => {
//             return Err(napi::Error::from_reason(error.to_string()));
//         }
//     }
// }

 
// pub async fn delete_single_record(
//     key: String,
//     project_name: Option<String>,
// ) -> Result<String, String> {
//     let client = create_client(None).await;
//     match client {
//         Ok(client) => {
//             let mut new_key = key.to_owned();
//             if !project_name.is_none() && project_name.as_ref().unwrap().len() > 0 {
//                 let res = get_project_level_key_with_global_prefix(
//                     project_name.as_ref().unwrap(),
//                     &new_key,
//                 );
//                 match res {
//                     Ok(res) => {
//                         new_key = res;
//                     }
//                     Err(error) => {
//                         return Err(napi::Error::from_reason(error.to_string()));
//                     }
//                 }
//             }
//             let delete_res = client.delete(new_key.to_owned()).await;
//             match delete_res {
//                 Ok(_delete_res) => {
//                     return Ok(String::from("Record Deleted With Key".to_owned() + &key));
//                 }
//                 Err(error) => {
//                     return Err(napi::Error::from_reason(error.to_string()));
//                 }
//             }
//         }
//         Err(error) => {
//             return Err(napi::Error::from_reason(error.to_string()));
//         }
//     }
// }

pub fn get_project_level_key_with_global_prefix(
    project: &str,
    key: &str,
) -> Result<String, napi::Error> {
    if project.trim().is_empty() {
        return Err(napi::Error::from_reason("tikv:project cannot be empty".to_string()));
    }
    if key.trim().contains("~") {
        return Err(napi::Error::from_reason("tikv:invalid character in key: ~".to_string()));
    }
    let key_with_project_name = format!("k{}_{}", project.trim().to_lowercase(), key.trim());
    Ok(key_with_project_name)
}

// pub fn caste
#[derive(Debug)]
pub struct ReturnError {
    pub error: String,
}
