use salvo::prelude::*;
use salvo::hyper::body::to_bytes;
use nacl::sign::verify;
use hex::FromHex;
use serde_json::json;

const PUBLIC_KEY: &str = "";

#[fn_handler]
async fn interaction(req: &mut Request, res: &mut Response){
    let inter = req.parse_json::<serde_json::Value>().await.unwrap();
    if inter["type"] == 1{
        res.render(Text::Json(json!({"type": 1}).to_string()));
    } else {
        res.render(Text::Json(json!({
            "type": 4,
            "data": {
                "content": "Hello, world!"
            }
        }).to_string()));
    }
}

#[fn_handler]
async fn check_signature(req: &mut Request, res: &mut Response) {

    let signature = req.header("X-Signature-Ed25519").unwrap_or(String::from(""));
    let timestamp = req.header("X-Signature-Timestamp").unwrap_or(String::from(""));

    match req.payload().await {
        Ok(body) => {
            if !verify(
                &<Vec<u8>>::from_hex(signature).unwrap().as_slice(),
                [timestamp.as_bytes(), body].concat().as_slice(),
                &<Vec<u8>>::from_hex(PUBLIC_KEY).unwrap().as_slice(),
            ).unwrap() {
                res.set_status_error(StatusError::unauthorized());
                return;
            }
        },
        Err(_) => {
            res.set_status_error(StatusError::bad_request());
        }
    }
}

#[tokio::main]
async fn main() {

    let router = Router::new()
        .hoop(check_signature)
        .post(interaction);

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .serve(router)
        .await;
}
