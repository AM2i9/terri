use salvo::prelude::*;
use salvo::Handler;
use nacl::sign::verify;
use hex::FromHex;

pub struct Validator {
    public_key: String
}

impl Validator {
    pub fn new(public_key: String) -> Validator {
        Validator {
            public_key: public_key
        }
    }
}

#[async_trait]
impl Handler for Validator {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl){
        let signature = req.header("X-Signature-Ed25519").unwrap_or(String::from(""));
        let timestamp = req.header("X-Signature-Timestamp").unwrap_or(String::from(""));
    
        match req.payload().await {
            Ok(body) => {
                if !verify(
                    &<Vec<u8>>::from_hex(signature).unwrap().as_slice(),
                    [timestamp.as_bytes(), body].concat().as_slice(),
                    &<Vec<u8>>::from_hex(self.public_key.as_str()).unwrap().as_slice(),
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
}