#![feature(await_macro, async_await)]

use futures::Future;
use futures03::{FutureExt as _, TryFutureExt as _};

struct Q;

impl Q {
    fn wrapper(&self) -> Box<Future<Item = u8, Error = String>> {
        Box::new(self.inner().boxed().compat())
    }

    async fn inner(&self) -> Result<u8, String> {
        Ok(42)
    }
}

fn main() {
}
