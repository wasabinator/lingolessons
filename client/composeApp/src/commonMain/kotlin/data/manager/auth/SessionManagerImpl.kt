package data.manager.auth

import data.api.auth.TokenApi
import data.api.common.ApiCallProcessor
import domain.auth.SessionManager
import domain.auth.SessionState
import io.ktor.client.HttpClient
import io.ktor.client.plugins.auth.Auth
import io.ktor.client.plugins.auth.providers.BearerAuthProvider
import io.ktor.client.plugins.auth.providers.BearerTokens
import io.ktor.client.plugins.auth.providers.bearer
import io.ktor.client.plugins.plugin
import io.ktor.http.encodedPath
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

/**
 * This class abstracts the token scheme. It makes it easy to change the scheme rather than
 * allowing the token to bleed out to other layers of the app.
 */
internal class SessionManagerImpl(
    private val client: HttpClient,
    private val tokenApi: TokenApi,
) : SessionManager, ApiCallProcessor {
    private val _state = MutableStateFlow<SessionState?>(null)
    override val state = _state.asStateFlow()

    private var token: BearerTokens? = null

    init {
        with(client.plugin(Auth)) {
            // TODO: load token from storage here
            _state.update { SessionState.None }

            if (providers.filterIsInstance<BearerAuthProvider>().isEmpty()) {
                bearer {
                    loadTokens {
                        token
                    }
                    refreshTokens {
                        token
                    }
                    sendWithoutRequest { request ->
                        request.url.encodedPath.contains("/token/login")
                    }
                }
            }
        }
    }

//    override suspend fun stop() {
//        client.plugin(Auth).providers.removeAll { it is BearerAuthProvider }
//    }

    override suspend fun login(username: String, password: String) = processCall(
        { tokenApi.login(username, password) }
    ) { result ->
        token = result?.let {
            BearerTokens(
                accessToken = result.authToken,
                refreshToken = "TODO",
            )
        }
        flush()

        _state.update {
            result?.let {
                SessionState.Authenticated(
                    username = username,
                )
            } ?: SessionState.None
        }
    }

    override suspend fun logout() {
        processCall(
            { tokenApi.logout() }
        ) {
            token = null
            flush()

            _state.update { SessionState.None }
        }
    }

    private fun flush() {
        client.plugin(Auth).providers.filterIsInstance<BearerAuthProvider>().first().clearToken()
    }
}
