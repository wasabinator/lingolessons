package com.lingolessons.data.auth

import com.lingolessons.data.common.ApiCallProcessor
import com.lingolessons.domain.auth.SessionManager
import com.lingolessons.domain.auth.SessionState
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

/**
 * This class abstracts the token scheme. It makes it easy to change the scheme rather than
 * allowing the token to bleed out to other layers of the app.
 */
internal class SessionManagerImpl(
    private val tokenApi: TokenApi,
    private val tokenRepository: TokenRepository,
    private val dispatcher: CoroutineDispatcher
) : SessionManager, ApiCallProcessor {
    private val state = MutableStateFlow<SessionState?>(null)

    private val scope by lazy {
        CoroutineScope(dispatcher)
    }

    init {
        scope.launch {
            tokenRepository.get().collect { sessionState ->
                if (sessionState == null) {
                    state.update { SessionState.None }
                } else {
                    state.update { SessionState.Authenticated(username = sessionState.username) }
                }
            }
        }
    }

    override fun get() = state.asStateFlow()

    override suspend fun login(username: String, password: String) = processCall(
        { tokenApi.login(username, password) }
    ) { result ->
        if (result == null) {
            tokenRepository.delete()
        } else {
            tokenRepository.put(
                SessionTokens(
                    username = username,
                    authToken = result.authToken,
                    refreshToken = "TODO",
                )
            )
        }
    }

    override suspend fun logout() = processCall(
        { tokenApi.logout() },
        always = {
            tokenRepository.delete()
        }
    ) {
        // No need to add anything here, as we're always doing the delete
    }
}
