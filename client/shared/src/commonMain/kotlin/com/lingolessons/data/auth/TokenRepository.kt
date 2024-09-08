package com.lingolessons.data.auth

import com.lingolessons.data.db.TokenDao
import com.lingolessons.domain.common.Repository
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

interface TokenRepository : Repository<SessionTokens>

internal class TokenRepositoryImpl(
    private val dao: TokenDao,
    private val dispatcher: CoroutineDispatcher,
) : TokenRepository {
    private val tokens = MutableStateFlow<SessionTokens?>(null)

    private val scope by lazy {
        CoroutineScope(dispatcher)
    }

    init {
        scope.launch {
            dao.get().let { session ->
                tokens.update {
                    session?.let {
                        SessionTokens(
                            username = it.username,
                            authToken = it.authToken,
                            refreshToken = it.refreshToken,
                        )
                    }
                }
            }
        }
    }

    override fun get() = tokens.asStateFlow()

    override fun put(item: SessionTokens) {
        scope.launch {
            tokens.update { item }
            dao.save(item.username, item.authToken, item.refreshToken)
        }
    }

    override fun delete() {
        scope.launch {
            tokens.update { null }
            dao.delete()
        }
    }
}

data class SessionTokens(
    val username: String,
    val authToken: String,
    val refreshToken: String,
)