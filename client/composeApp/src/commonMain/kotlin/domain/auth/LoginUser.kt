package domain.auth

import common.suspendCatching
import domain.Operation
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.withContext

class LoginUser(
    private val dispatcher: CoroutineDispatcher,
    private val sessionManager: SessionManager,
) : Operation<LoginDetails, Unit> {
    override suspend fun perform(param: LoginDetails): Result<Unit> =
        withContext(dispatcher) {
            suspendCatching {
                sessionManager.login(param.username, param.password)
            }.onSuccess {
                val context = currentCoroutineContext()
                println("OK $context")
            }.onFailure {
                val context = currentCoroutineContext()
                println("YIKES $context")
            }
        }
}
