use clipstash::data::query::new_clip;
use clipstash::data::query::test_helpers::*;
use clipstash::web::test_helpers::new_db;

#[test]
fn test_db() {
    // let db = new_db();
    let rt = async_runtime();
    let db = new_db(rt.handle());
    let pool = db.get_pool();

    let clip = rt.block_on(async move { new_clip(model_new_clip("1"), &pool.clone()).await });
    assert!(clip.is_ok());
    let clip = clip.unwrap();
    assert!(clip.content == format!("content for clip '1'"));
}

pub fn async_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime")
}
