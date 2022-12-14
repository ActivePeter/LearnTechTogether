//操作article的表

use crate::db::sql::DbHandler;
use crate::models::user::UserId;
use crate::models::comment::{CommentId, Comment};
use crate::models::article::ArticleId;
use deadpool_postgres::tokio_postgres::{Error, Row};
use crate::services::comment::CommentManager_Comment2Some;

// 表划分
impl DbHandler {
    pub async fn create_comment_table(&self) {}
    pub async fn db_comment_loadall(&self) -> CommentManager_Comment2Some {
        let res=self.get().await.query("select commentid,aid from comment_info",&[]).await.unwrap();
        let mut map:CommentManager_Comment2Some=Default::default();
        for r in res{
            let cid:i64=r.get(0);
            let aid:i64=r.get(1);
            map.insert(cid,aid);
        }
        map
    }
    pub async fn db_get_comments_of_article(&self,aid:ArticleId)->Vec<Comment>{
        let mut vec =Vec::new();
        let res=self.get().await.query(&*format!(
            "select commentid,userid,content,to_art_or_com, \
                   to_char(time, 'yyyy-mm-dd hh24:mi:ss'),aid,to_cid \
                   from comment_info \
            where aid={} order by time desc",aid),&[]).await;
        match res{
            Ok(res) => {
                for r in res{
                    let uid:i64=r.get(1);
                    let cid:i64=r.get(0);
                    let to_cid:i64=r.get(6);
                    let to_a_or_c:bool=r.get(3);
                    let c=Comment{
                        uid: uid as UserId,
                        content:r.get(2),
                        to_comment_or_article: !to_a_or_c,
                        cid: cid as CommentId,
                        to_cid: to_cid as CommentId,
                        aid,
                        time:r.get(4)
                    };
                    vec.push(c);
                }
            }
            Err(e) => {
                eprintln!("{:?}",e)
            }
        }

        return vec;
    }
    pub async fn db_create_comment(&self,
                                   uid: UserId,
                                   content: &String,
                                   to_comment_or_article: bool,
                                   to_cid: CommentId,
                                   aid: ArticleId, ) -> Option<CommentId> {
        let res=self.get().await
            .query("insert into comment_info(userid,content,to_art_or_com,time,aid,to_cid) \
                values($1::bigint,$2::TEXT,$3::boolean,now(),$4::bigint,$5::bigint) \
                RETURNING commentid",&[&(uid as i64),&content,
                &(!to_comment_or_article),
                &(aid as i64),&(to_cid as i64)]).await;
        match res{
            Ok(res) => {

                let cid_:i64=res[0].get(0);
                return Some(cid_ as CommentId)
            }
            Err(e) => {
                eprintln!("{:?}",e);
                None
            }
        }
    }
    pub async fn db_remove_comment(&self) {}
    pub async fn db_edit_comment(&self) {}
}