use axum::{Router, routing::get};

const BANNER: &str = r#"
          ######   ######   ######  
         ##    ## ##    ## ##    ## 
         ##       ##       ##       
         ##       ##       ##       
         ##       ##       ##       
         ##    ## ##    ## ##    ## 
          ######   ######   ######  

          Closed Circuit Consulting
               Twin Cities, MN            
  
  https://git.closedcircuitconsulting.com
  https://github.com/closedcircuitconsulting

"#;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { BANNER }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let server_handle = tokio::spawn(async move { axum::serve(listener, app).await });
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    if !server_handle.is_finished() {
        let _ = sd_notify::notify(true, &[sd_notify::NotifyState::Ready]);
    }

    server_handle.await.unwrap().unwrap();
}
