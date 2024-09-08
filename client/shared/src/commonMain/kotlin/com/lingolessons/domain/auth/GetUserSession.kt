package com.lingolessons.domain.auth

import com.lingolessons.domain.GetOperation
import kotlinx.coroutines.flow.Flow

interface GetUserSession : GetOperation<Unit, SessionState>

class GetUserSessionImpl(
    private val sessionManager: SessionManager,
) : GetUserSession {
    override fun perform(param: Unit): Flow<SessionState?> = sessionManager.get()
}
