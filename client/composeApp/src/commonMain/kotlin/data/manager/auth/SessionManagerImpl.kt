package data.manager.auth

import com.lingolessons.data.db.SessionQueries
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
    private val client: HttpClient,
    private val tokenApi: TokenApi,
    private val session: SessionQueries,
    private val dispatcher: CoroutineDispatcher
) : SessionManager, ApiCallProcessor {
    private val _state = MutableStateFlow<SessionState?>(null)
    override val state = _state.asStateFlow()

    private var token: BearerTokens? = null
    private val scope by lazy {
        CoroutineScope(dispatcher)
    }

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
        scope.launch {
            session.get().executeAsOneOrNull().let { sessionState ->
                if (sessionState == null) {
                    token = null
                    _state.update { SessionState.None }
                } else {
                    token = BearerTokens(
                        accessToken = sessionState.authToken,
                        refreshToken = "TODO",
                    )
                    _state.update { SessionState.Authenticated(username = sessionState.username) }
                }
                flush()
            }
        }
    }

//    override suspend fun stop() {
//        client.plugin(Auth).providers.removeAll { it is BearerAuthProvider }
//    }

    override suspend fun login(username: String, password: String) = processCall(
        { tokenApi.login(username, password) }
    ) { result ->
        if (result == null) {
            session.clear()
            _state.update { SessionState.None }
            token = null
        } else {
            session.save(username, result.authToken, "TODO")
            _state.update {
                SessionState.Authenticated(
                    username = username,
                )
            }
            token = BearerTokens(
                accessToken = result.authToken,
                refreshToken = "TODO",
            )
        }
        flush()
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
