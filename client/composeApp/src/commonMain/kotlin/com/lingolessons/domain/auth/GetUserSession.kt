package com.lingolessons.domain.auth

import com.lingolessons.domain.GetOperation
import kotlinx.coroutines.flow.Flow

class GetUserSession(
    private val sessionManager: SessionManager,
) : GetOperation<Unit, SessionState> {
    override fun perform(param: Unit): Flow<SessionState?> = sessionManager.state
}
