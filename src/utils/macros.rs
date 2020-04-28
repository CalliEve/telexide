/// This macro creates the [`Framework`] object using the given bot name and
/// registers all the given commands on it. Call it as in
/// `create_framework!("bot_name", command1, command2, ...)`
///
/// [`Framework`]: framework/struct.Framework.html
#[macro_export]
macro_rules! create_framework {
    ($bot_name:expr, $( $c:expr ),*) => {
        {
            paste::expr! {
                let mut fr = telexide::framework::Framework::new($bot_name);
                $(
                     fr.add_command(&(&[<$c _COMMAND>]));
                )*
                ::std::sync::Arc::new(fr)
            }
        }
    }
}
