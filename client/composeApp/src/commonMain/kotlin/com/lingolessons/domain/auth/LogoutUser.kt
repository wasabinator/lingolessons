package com.lingolessons.domain.auth

import com.lingolessons.common.suspendCatching
import com.lingolessons.domain.Operation
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
