use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio::sync::oneshot::Sender;
use futures_util::stream::{SplitStream, SplitSink};
use futures_util::{StreamExt, SinkExt};
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio_tungstenite::tungstenite::error::Error;
use lazy_static::__Deref;
use tokio::time::Duration;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use axum::http::Response;
use serde_json::Value;

lazy_static::lazy_static! {
    pub static ref G_ROBOT_MAN : RobotMan = RobotMan::new();
}

enum MsgTarget{
    Group(i64),
    Person(i64),
}
impl MsgTarget{
    pub fn parse_from_value(v:&serde_json::Value)->Option<MsgTarget>{
        if v.is_object(){
            match v.as_object().unwrap().get("message_type"){
                None => {return None;}
                Some(v_) => {
                    if v_.is_string()&&v_.as_str().unwrap()=="group"{
                        return Some(MsgTarget::Group(
                            v.as_object().unwrap().get("group_id").unwrap().as_i64().unwrap()
                        ));
                    }
                    if v_.is_string()&&v_.as_str().unwrap()=="private"{
                        return Some(MsgTarget::Person(
                            v.as_object().unwrap().get("user_id").unwrap().as_i64().unwrap()
                        ));
                    }
                }
            }
        }
        None
    }
}

struct MsgBuilder{
    targets:Vec<MsgTarget>,
    msg:String
}
impl MsgBuilder{
    pub fn new() -> MsgBuilder {
        MsgBuilder{
            targets: vec![],
            msg: "".to_string()
        }
    }
    pub fn add_target(&mut self, tar:MsgTarget) -> &mut MsgBuilder {
        self.targets.push(tar);

        self
    }
    pub fn set_msg(&mut self, msg:String) -> &mut MsgBuilder {
        self.msg=msg;

        self
    }
    pub async fn send(&mut self){
        for t in &self.targets{
            if let MsgTarget::Group(id)=t{
                let mut sender=G_ROBOT_MAN.sender.write().await;
                match sender.as_mut().unwrap().send(Message::from(
                    format!("{{\
                        \"action\": \"send_group_msg\", \
                        \"params\": {{ \
                            \"group_id\": \"{}\", \
                            \"message\": \"{}\" \
                        }}, \
                    }}",id,self.msg)
                )).await{
                    Ok(_) => {}
                    Err(_) => {}
                }
            }else if let MsgTarget::Person(id)=t{
                let mut sender=G_ROBOT_MAN.sender.write().await;
                match sender.as_mut().unwrap().send(Message::from(
                    format!("{{\
                        \"action\": \"send_private_msg\", \
                        \"params\": {{ \
                            \"user_id\": \"{}\", \
                            \"message\": \"{}\" \
                        }}, \
                    }}",id,self.msg)
                )).await{
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
}

pub struct RobotMan {
    sender: RwLock<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
}

impl RobotMan {
    pub fn new() -> RobotMan {
        RobotMan {
            sender: Default::default()
        }
    }
    pub async fn send_str(&self){

    }
    pub async fn send_msg(&self,msg:&String){
        let mut sender = self.sender.write().await;
        let s =
            sender.as_mut().unwrap();
        match s.send(Message::from(
            format!("{{\
                        \"action\": \"send_group_msg\", \
                        \"params\": {{ \
                            \"group_id\": \"1070262019\", \
                            \"message\": \"{}\" \
                        }}, \
                    }}",msg)
        )).await{
            Ok(_) => {}
            Err(_) => {}
        }
    }
    pub async fn send_helloworld(&self) {
        let mut sender = self.sender.write().await;
        let s =
            sender.as_mut().unwrap();
        println!("{:?}",Message::from("{\
                        \"action\": \"send_group_msg\", \
                        \"params\": { \
                            \"group_id\": \"1070262019\", \
                            \"message\": \"hello world\" \
                        }, \
                    }"));
        s.send(Message::from("{\
                        \"action\": \"send_group_msg\", \
                        \"params\": { \
                            \"group_id\": \"1070262019\", \
                            \"message\": \"hello world\" \
                        }, \
                    }")).await.unwrap();

        // {
        //     "action": "???????????????, ?????? 'send_group_msg'",
        //     "params": {
        //     "?????????": "?????????",
        //     "?????????2": "?????????"
        // },
        //     "echo": "'??????', ??????????????? echo ??????, ??????????????????????????????????????? echo ??????, ????????????????????????"
        // }
    }
}


// #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn start_robot() {
    loop{
        println!("begin connect");
        // loop{
        let c=connect_async("ws://s5.nsloop.com:4439").await;
        match c{
            Ok(c) => {
                let (ws_stream, _) =c;
                    println!("WebSocket handshake has been successfully completed");

                let (write, read) =
                    ws_stream.split();


                G_ROBOT_MAN.sender.write().await.replace(write);

                let (t, r) = tokio::sync::oneshot::channel();
                read_loop(read, t).await;

                // tokio::time::sleep(Duration::from_millis(3000));
                // G_ROBOT_MAN.send_msg(&"hello".to_string()).await;
                //??????websocket????????????
                r.await.unwrap();
                println!("robot end");
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
            Err(_) => {
                println!("co to robot fail");
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }
    // }
}
mod prints{
    use crate::services::robot_service::{MsgTarget, MsgBuilder};

    pub async fn help_list(v:serde_json::Value)->Option<()>{
        MsgBuilder::new()
            .set_msg("current cmds:\n\
            - list_tags\n\
            - ".to_string())
            .add_target(MsgTarget::parse_from_value(&v)?)
            .send().await
        ;

        Some(())
    }
    pub async fn list_tags(v:serde_json::Value)->Option<()>{
        MsgBuilder::new()
            .set_msg("haha".to_string())
            .add_target(MsgTarget::parse_from_value(&v)?)
            .send().await
        ;

        Some(())
    }
}

pub async fn handle_manager_msg(v:serde_json::Value){
    let cmd=v.as_object().unwrap().get("message").unwrap().as_str().unwrap();
    let iter=cmd.split_whitespace();
    let (len,_)=iter.size_hint();

    println!("msg4 {}",len);
    if len==0{
        match cmd{
            "?????????"=>{
                prints::help_list(v).await;
            }
            "list_tags"=>{
                prints::list_tags(v).await;
            }
            &_ => {}
        }
    }else{

    }
}

pub async fn read_loop(read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
                       sender: Sender<()>,
) {
    tokio::spawn(async move {
        read.for_each(|message| async {
            match message {
                Ok(message) =>{
                    println!("msg {}",message.to_string());
                    let v:serde_json::Value=serde_json::from_str(&*message.to_string()).unwrap();
                    if v.is_object(){
                        if let Some(id)=v.as_object().unwrap().get("user_id")
                        {
                            println!("msg2 {}",id);
                            if id.is_number()&&id.as_i64().unwrap()==1020401660{
                                println!("msg3");
                                handle_manager_msg(v).await;
                            }
                        }
                    }
                    // tokio::io::stdout().write_all(message).await.unwrap();
                }
                Err(_) => {
                    return;
                }
            }
        }).await;

        sender.send(()).unwrap();
    });
}