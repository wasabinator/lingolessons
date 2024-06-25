package domain.auth

import common.suspendCatching
import domain.Operation
import kotlinx.coroutines.withContext
import kotlin.coroutines.CoroutineContext

class LogoutUser(
    private val dispatcher: CoroutineContext,
    private val sessionManager: SessionManager,
) : Operation<Unit, Unit> {
    override suspend fun perform(param: Unit): Result<Unit> =
        withContext(dispatcher) {
            suspendCatching {
                sessionManager.logout()
            }
        }
}
