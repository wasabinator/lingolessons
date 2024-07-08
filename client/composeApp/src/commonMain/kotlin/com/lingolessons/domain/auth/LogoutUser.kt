package com.lingolessons.domain.auth

import com.lingolessons.common.suspendCatching
import com.lingolessons.domain.Operation
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.withContext

typealias LogoutUser = Operation<Unit, Unit>

class LogoutUserImpl(
    private val dispatcher: CoroutineDispatcher,
    private val sessionManager: SessionManager,
) : LogoutUser {
    suspend fun perform() = perform(Unit)

    override suspend fun perform(param: Unit): Result<Unit> =
        withContext(dispatcher) {
            suspendCatching {
                sessionManager.logout()
            }
        }
}
