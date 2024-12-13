use deadpool_postgres::{Config, Pool};
use deadpool_postgres::tokio_postgres::NoTls; //trasnport layer security, mzyana la kan dakchi
//secure aslan 

pub async fn create_pool() -> Pool { // async hitach connection t9ed takhod wa9t o makhessch tblocki l main thread 
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("123456".to_string()); 
    cfg.dbname = Some("todo_app".to_string());

    cfg.create_pool(NoTls).unwrap() 
}
