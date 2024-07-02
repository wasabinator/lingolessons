package com.lingolessons.domain.auth

import kotlinx.coroutines.flow.StateFlow

interface SessionManager {
    fun get(): StateFlow<SessionState?>
    suspend fun login(username: String, password: String)
    suspend fun logout()
}

data class LoginDetails(
    val username: String,
    val password: String,
)

sealed class SessionState {
    data object None : SessionState()
    data class Authenticated(
        val username: String
    ) : SessionState()
}
