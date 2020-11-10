use std::sync::atomic::{AtomicUsize, Ordering};
use telexide::{
    client::{ClientBuilder, Context},
    model::{Update, UpdateContent},
    Result,
};

#[tokio::test]
async fn update_handler_gets_called() -> Result<()> {
    static B: AtomicUsize = AtomicUsize::new(0);

    let mut c = ClientBuilder::new().set_token("test").build();
    c.subscribe_handler_func(|_x, u| {
        Box::pin(async move {
            B.fetch_add(u.update_id as usize, Ordering::Acquire);
        })
    });

    c.fire_handlers(Update {
        update_id: 10,
        content: UpdateContent::Unknown,
    });

    tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;

    assert_eq!(B.load(Ordering::Relaxed), 10);
    Ok(())
}

static FUNC_B: AtomicUsize = AtomicUsize::new(0);

fn testing_func(
    _c: Context,
    u: Update,
) -> ::std::pin::Pin<Box<dyn Send + ::std::future::Future<Output = ()>>> {
    ::std::boxed::Box::pin(async move {
        FUNC_B.fetch_add(u.update_id as usize, Ordering::Acquire);
    })
}

#[tokio::test]
async fn test_using_func() -> Result<()> {
    let mut c = ClientBuilder::new().set_token("test").build();

    c.subscribe_handler_func(testing_func);

    c.fire_handlers(Update {
        update_id: 10,
        content: UpdateContent::Unknown,
    });

    tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;

    assert_eq!(FUNC_B.load(Ordering::Relaxed), 10);
    Ok(())
}
