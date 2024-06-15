package common

import domain.common.RepositoryError

suspend fun <T> suspendCatching(
    block: suspend () -> T
): Result<T> = try {
    Result.success(block())
} catch (e: RepositoryError) {
    Result.failure(e)
}
