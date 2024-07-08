package com.lingolessons.data.auth

import com.lingolessons.common.BaseTest
import com.lingolessons.domain.auth.SessionManager
import com.lingolessons.domain.auth.SessionState
import dev.mokkery.answering.calls
import dev.mokkery.answering.returns
import dev.mokkery.answering.throws
import dev.mokkery.every
import dev.mokkery.everySuspend
import dev.mokkery.matcher.any
import dev.mokkery.mock
import dev.mokkery.verify
import dev.mokkery.verify.VerifyMode.Companion.exactly
import kotlinx.coroutines.flow.MutableStateFlow
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class SessionManagerTest : BaseTest() {
    private lateinit var tokenApi: TokenApi
    private lateinit var tokenRepository: TokenRepository

    private lateinit var tokenState: MutableStateFlow<SessionTokens?>

    private lateinit var sessionManager: SessionManager

    override fun setup() {
        tokenState = MutableStateFlow(null)
        tokenApi = mock()

        tokenRepository = mock {
            every {
                get()
            } returns tokenState

            every {
                put(any())
            } calls {
                tokenState.value = it.args.first() as SessionTokens
            }
        }

        sessionManager = SessionManagerImpl(
            tokenApi = tokenApi,
            tokenRepository = tokenRepository,
            dispatcher = dispatcher
        )
    }

    @Test
    fun `given no tokens then there should be no session`() = runTest {
        advanceUntilIdle()
        assertEquals(SessionState.None, sessionManager.get().value)
    }

    @Test
    fun `given tokens then there should be a session`() = runTest {
        tokenState.value = SessionTokens("username", "authToken", "refreshToken")
        advanceUntilIdle()
        assertEquals(SessionState.Authenticated("username"), sessionManager.get().value)
    }

    @Test
    fun `given a successful login there should be a session`() = runTest {
        everySuspend {
            tokenApi.login("user123", "password")
        } returns LoginResponse("a token", "refresh token")

        sessionManager.login("user123", "password")
        advanceUntilIdle()

        verify(exactly(1)) {
            tokenRepository.put(SessionTokens("user123", "a token", "refresh token"))
        }

        assertEquals(SessionState.Authenticated("user123"), sessionManager.get().value)
    }

    @Test
    fun `given a failed login there should be no session`() = runTest {
        everySuspend {
            tokenApi.login("user123", "password")
        } throws Exception()

        val result = runCatching {
            sessionManager.login("user123", "password")
            advanceUntilIdle()
        }

        assertTrue { result.isFailure }

        verify(exactly(0)) {
            tokenRepository.put(any())
        }

        assertEquals(SessionState.None, sessionManager.get().value)
    }
}
