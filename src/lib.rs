use std::sync::Arc;

use futures_util::{Stream, TryFuture};
use genawaiter::sync::Gen;

pub struct Co<T, E>(Arc<genawaiter::sync::Co<Result<T, E>>>);

impl<T, E> Co<T, E> {
    pub async fn yield_(&self, value: T) {
        self.0.yield_(Ok(value)).await
    }
}

pub fn to_try_stream<T: Send, E: Send, F: Send + TryFuture<Output = Result<(), E>>>(
    f: impl Send + FnOnce(Co<T, E>) -> F,
) -> impl Send + Stream<Item = Result<T, E>> {
    Gen::new(async |co| {
        let co = Arc::new(co);
        if let Err(e) = f(Co(co.clone())).await {
            co.yield_(Err(e)).await;
        }
    })
}
