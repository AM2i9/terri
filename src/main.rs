pub mod game;
pub mod board;

use salvo::prelude::*;
use salvo::Handler;
use nacl::sign::verify;
use hex::FromHex;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;
use reqwest;

use crate::game::Game;

const PUBLIC_KEY: &str = "";

macro_rules! component {
    ($type:expr, $label:expr, $style:expr, $custom_id:expr) => {
        json!({
            "type": $type,
            "label": $label,
            "style": $style,
            "custom_id": $custom_id
        })
    };
}

pub struct State {
    pub tx: Option<SyncSender<i32>>,
    pub current_player: Option<String>
}

pub struct InteractionHandler {
    pub state: Arc<Mutex<State>> 
}

#[async_trait]
impl Handler for InteractionHandler {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl){

        let inter = req.parse_json::<serde_json::Value>().await.unwrap();
        let mut state = &mut *self.state.lock().unwrap();

        match inter["type"].as_u64().unwrap() {
            1 => res.render(Text::Json(json!({"type": 1}).to_string())),
            2 => {
                if state.tx.is_none() {
                    let (tx, rx): (SyncSender<i32>, Receiver<i32>) = sync_channel(1);

                    state.tx.insert(tx);
        
                    let inter_token = inter["token"].as_str().unwrap().to_owned();
                    let application_id = inter["application_id"].as_str().unwrap().to_owned();

                    let player = inter["member"]["user"]["id"].as_str().unwrap().to_owned();
                    state.current_player.insert(player);
            
                    thread::spawn(move || {
                        let client = reqwest::blocking::Client::new();
                        let url = format!("https://discord.com/api/webhooks/{app_id}/{token}/messages/@original",
                                    app_id = application_id,
                                    token = inter_token
                                );
                        let game = Game::new();
                        loop {
                            thread::sleep(std::time::Duration::from_secs(1));
                            let msg = rx.try_recv();
                            
                            match msg {
                                Ok(msg) => {
                                    println!("{}", msg);
                                    if msg == -1 {
                                        break;
                                    }
                                },
                                Err(e) => {
                                    println!("{}", e);
                                }
                            }
            
                            match client.patch(&url).body(
                                    json!({
                                        "embeds": [
                                            {
                                                "description": format!("```ansi\n{}```", game.get_board().draw()),
                                            }
                                        ],
                                        "components" : [
                                            {
                                                "type": 1,
                                                "components": [
                                                    component!(2, "L", 1, "1"),
                                                    component!(2, "ML", 1, "2"),
                                                    component!(2, "D", 1, "3"),
                                                    component!(2, "MR", 1, "4"),
                                                    component!(2, "R", 1, "5"),

                                                ]
                                            }
                                        ]
                                    }).to_string()
                                ).header("Content-Type", "application/json").send()
                            {
                                Ok(_) => {},
                                Err(e) => {
                                    println!("{}", e);
                                }
                            };
                        }
                    });
                } else {
                    println!("test");
                    res.render(Text::Json(json!({"type": 4, "data": {"content": "Sorry, I can only play one game at a time right now (due to rate limits) :)", "flags": 64}}).to_string()));
                    return;
                }
        
                res.render(Text::Json(json!({"type": 5}).to_string()));
            },
            3 => {
                if state.current_player.as_ref().unwrap() == inter["member"]["user"]["id"].as_str().unwrap() {
                    let action = inter["data"]["custom_id"].as_str().unwrap().parse::<i32>().unwrap();
                    state.tx.as_ref().unwrap().send(action).unwrap();
                } else {
                    res.render(Text::Json(json!({"type": 6}).to_string()));
                }
            },
            _ => res.set_status_error(StatusError::bad_request())
    }
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

    let handler = InteractionHandler{
        state: Arc::new(Mutex::new(State {
            tx: None,
            current_player: None
        }))
    };

    let router = Router::new()
        .hoop(check_signature)
        .post(handler);

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .serve(router)
        .await;
}
