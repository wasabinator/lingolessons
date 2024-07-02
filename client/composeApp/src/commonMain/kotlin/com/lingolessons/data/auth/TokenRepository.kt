package com.lingolessons.data.auth

import com.lingolessons.data.db.TokenQueries
import com.lingolessons.domain.common.Repository
import io.ktor.client.HttpClient
import io.ktor.client.plugins.auth.Auth
import io.ktor.client.plugins.auth.providers.BearerAuthProvider
import io.ktor.client.plugins.plugin
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

interface TokenRepository : Repository<SessionTokens>

internal class TokenRepositoryImpl(
    private val client: Lazy<HttpClient>, // Lazy to prevent a circular dependency
    private val queries: TokenQueries,
    private val dispatcher: CoroutineDispatcher,
) : TokenRepository {
    private val tokens = MutableStateFlow<SessionTokens?>(null)

    private val scope by lazy {
        CoroutineScope(dispatcher)
    }

    init {
        scope.launch {
            queries.get().executeAsOneOrNull().let { session ->
                tokens.update {
                    session?.let {
                        SessionTokens(
                            username = it.username,
                            authToken = it.authToken,
                            refreshToken = "TODO",
                        )
                    }
                }
                flush()
            }
        }
    }

    override fun get() = tokens.asStateFlow()

    override fun put(item: SessionTokens) {
        scope.launch {
            tokens.update { item }
            queries.save(item.username, item.authToken, item.refreshToken)
        }
    }

    override fun delete() {
        tokens.update { null }
    }

    private fun flush() {
        client.value.plugin(Auth).providers.filterIsInstance<BearerAuthProvider>().first()
            .clearToken()
    }
}

data class SessionTokens(
    val username: String,
    val authToken: String,
    val refreshToken: String,
)