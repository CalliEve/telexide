#[macro_export]
macro_rules! create_framework {
    ($bot_name:expr, $( $c:expr ),*) => {
        {
            paste::expr! {
                let mut fr = telegram_botoxide::framework::Framework::new($bot_name);
                $(
                     fr.add_command(&(&[<$c _COMMAND>]).into());
                )*
                ::std::sync::Arc::new(fr)
            }
        }
    }
}
