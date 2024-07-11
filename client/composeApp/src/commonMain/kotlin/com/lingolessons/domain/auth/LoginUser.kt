package com.lingolessons.domain.auth

import com.lingolessons.common.suspendCatching
import com.lingolessons.domain.Operation
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.withContext

interface LoginUser : Operation<LoginDetails, Unit>

class LoginUserImpl(
    private val dispatcher: CoroutineDispatcher,
    private val sessionManager: SessionManager,
) : LoginUser {
    override suspend fun perform(param: LoginDetails): Result<Unit> =
        withContext(dispatcher) {
            suspendCatching {
                sessionManager.login(param.username, param.password)
            }
        }
}
