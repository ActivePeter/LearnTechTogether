use crate::models::user::{User, UserId};
use tokio_postgres::{Client, Error, Row, NoTls, Connection, Socket};
use crate::*;
// use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use crate::readconfig::ServerConfig;
use crate::services::user_manager::{G_USER_MANAGER};
use tokio::time;
use tokio_postgres::tls::NoTlsStream;
use crate::db::sql::{DbHandler, get_dbhandler};
use crate::services::password_md5::hash_u64_to_i64;

//这里实现用户相关的增删改查
//对应user 表的dao层，持有到数据库的消息通道，
//后续数据库断开没有关系，在数据库重新链接后会来更新这里的client
impl DbHandler {
    //此处的增删改查由service层的user_manager调用，处理api请求
    // 此时可以保证client存在，因为数据库在结束启动时，
    // 会等待usermanager加载完内存，而加载内存就来自一次数据库读取
    pub async fn db_get_all_user(&self) -> Option<Vec<Row>> {
        let res = self.get().await
            .query("select * from user_info", &[])
            .await;
        if let Ok(res_) = res {
            return Some(res_);
        }

        return None;
    }
    pub async fn db_update_user_pw(&self,uid:UserId,salt:&String,hash_:u64){
        let hash=hash_u64_to_i64(hash_);
        let cmd=format!("update user_info set password=$1::TEXT,
                        pw_hash={} \
                    where id={};",hash,uid);
        self.get().await.query(&*cmd,&[&salt]).await.unwrap();
    }
    pub async fn db_create_user(&self, new_user: &mut User) -> bool {
        let hashi64=hash_u64_to_i64(new_user.pw_hash);
        // 由user_manger调用
        let mut insert_cmd = format!("insert into user_info(username,password,email,pw_hash)\
            values('{}','{}','{}',{})\
            RETURNING id"
                                     , new_user.username, new_user.pw_salt, new_user.email,hashi64);
        println!("insert_cmt {}", insert_cmd);

        let insert_result = self.get().await
            .query(&insert_cmd, &[]).await;

        match insert_result {
            Ok(v) => {
                for a in v {
                    let id = a.get(0);
                    new_user.id = id;
                }
                // println!("new user {}", new_user.id);
                true
            }
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }
}
