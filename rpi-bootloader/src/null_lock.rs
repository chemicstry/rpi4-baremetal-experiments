/// Lock that doesn't do anything and is not safe for multiple cores.
/// Use spin locks instead when MMU is configured to support atomics.
pub struct NullLock {}

unsafe impl lock_api::RawMutex for NullLock {
    const INIT: NullLock = NullLock {};

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = lock_api::GuardNoSend;

    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        true
    }

    unsafe fn unlock(&self) {}
}
