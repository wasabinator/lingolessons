package com.lingolessons.common

import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.ensureActive

suspend fun <T> suspendCatching(
    block: suspend () -> T
): Result<T> = try {
    Result.success(block())
} catch (e: Throwable) {
    Result.failure(e)
}.also {
    // If we ate a CancellationException, this will throw one again for us.
    currentCoroutineContext().ensureActive()
}
