use async_std::task;

use kebab_lib::get_db;

fn main()  {
    task::block_on(async {
        let db = get_db().await;
    });
}
