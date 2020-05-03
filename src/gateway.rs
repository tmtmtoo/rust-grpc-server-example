mod save_greeting;

use crate::infrastructure::db;
use derive_new::*;

#[derive(new)]
pub struct Adaptor {
    tx: db::TransactionManager,
}

#[cfg(test)]
pub struct StubAdaptor {
    save_greeting_result: crate::domain::SaveResult<()>,
}

#[cfg(test)]
pub fn get_test_transaction_manager() -> db::TransactionManager {
    use dotenv::dotenv;
    use std::env;
    use std::sync::Once;

    static mut TRANSACTION: Option<db::TransactionManager> = None;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            dotenv().ok();
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = db::connection_pool(database_url.as_str(), 4).unwrap();
            db::migration(&pool).unwrap();
            TRANSACTION = Some(db::TransactionManager::new(pool));
        });

        TRANSACTION.clone().unwrap()
    }
}
