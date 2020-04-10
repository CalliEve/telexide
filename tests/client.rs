use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
    RwLock,
};
use telegram_botoxide::{
    client::{Client, Context},
    macros::{command, prepare_listener},
    model::{Update, UpdateContent},
    options::OptionsBuilder,
    Result,
};

#[tokio::test]
async fn update_handler_gets_called() -> Result<()> {
    static b: AtomicUsize = AtomicUsize::new(0);

    let mut c = Client::new(&OptionsBuilder::new().set_token("test".to_owned()).build()?);
    c.subscribe_handler(|_x, u| {
        Box::pin(async move {
            b.fetch_add(u.update_id as usize, Ordering::Acquire);
        })
    });

    c.fire_handlers(Update {
        update_id: 10,
        content: UpdateContent::Unknown,
    })
    .await;

    assert_eq!(b.load(Ordering::Relaxed), 10);
    Ok(())
}

static func_b: AtomicUsize = AtomicUsize::new(0);

fn testing_func(
    _c: Context,
    u: Update,
) -> ::std::pin::Pin<Box<dyn Send + ::std::future::Future<Output = ()>>> {
    ::std::boxed::Box::pin(async move {
        func_b.fetch_add(u.update_id as usize, Ordering::Acquire);
    })
}

#[tokio::test]
async fn test_using_func() -> Result<()> {
    let mut c = Client::new(&OptionsBuilder::new().set_token("test".to_owned()).build()?);

    c.subscribe_handler(testing_func);

    c.fire_handlers(Update {
        update_id: 10,
        content: UpdateContent::Unknown,
    })
    .await;

    assert_eq!(func_b.load(Ordering::Relaxed), 10);
    Ok(())
}
