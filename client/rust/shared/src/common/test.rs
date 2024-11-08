use std::future::Future;
use std::time::Duration;
use tokio::time::error::Elapsed;

/// Allows an async future to get tested against an expected condition within a time threshhold.
/// It will either fail with an Elapsed error, or will return the expected condition.
///
/// This is superior to adding sleeps in tests as this will only require suspensions until precisely the time
/// in which the condition occurs. At worst, it ends up waiting a few seconds prior to failing the test.
pub async fn await_condition<T, F, Fut>(mut op: F, condition: fn(&T) -> bool) -> Result<T, Elapsed>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = T>,
    T: PartialEq
{
    tokio::time::timeout(
        Duration::from_secs(5),
        async {
            loop  {
                let r = op().await;
                if condition(&r) {
                    return r;
                }
            }
        },
    ).await
}
