mod oter;
use oter::{test_instrument, test_span};

fn main() {
    tracing_subscriber::fmt::init();

    // test_span();

    test_instrument(5);

    // tracing::info!(
    //     target = "hello",
    //     action = "login",
    //     "User performed an action"
    // );
}
