use serde::Serialize;
use tauri::command;
use url::Url;

#[derive(Serialize)]
struct ReadonlyResponse{status:u16,body:String}

#[command]
async fn readonly_get(url:String,token:String)->Result<ReadonlyResponse,String>{
 let parsed=Url::parse(&url).map_err(|_|"Invalid server URL".to_string())?;
 if parsed.scheme()!="https"&&parsed.scheme()!="http"{return Err("Only http:// and https:// INFO servers are allowed".to_string())}
 let client=reqwest::Client::builder().user_agent("INFO-Desktop/0.1.0-dev").redirect(reqwest::redirect::Policy::limited(5)).build().map_err(|e|e.to_string())?;
 let mut request=client.get(parsed).header(reqwest::header::ACCEPT,"application/json");
 if !token.trim().is_empty(){request=request.bearer_auth(token.trim());}
 let response=request.send().await.map_err(|e|e.to_string())?;
 let status=response.status().as_u16();let body=response.text().await.map_err(|e|e.to_string())?;
 Ok(ReadonlyResponse{status,body})
}

#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run(){tauri::Builder::default().invoke_handler(tauri::generate_handler![readonly_get]).run(tauri::generate_context!()).expect("error while running INFO Desktop");}
