package com.lingolessons.data.auth

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
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestCoroutineScheduler
import kotlinx.coroutines.test.TestDispatcher
import kotlinx.coroutines.test.runTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlin.test.assertTrue

class SessionManagerImplTest {
    private lateinit var tokenApi: TokenApi
    private lateinit var tokenRepository: TokenRepository

    private lateinit var tokenState: MutableStateFlow<SessionTokens?>

    private lateinit var sessionManager: SessionManager

    private lateinit var scheduler: TestCoroutineScheduler
    private lateinit var dispatcher: TestDispatcher

    @BeforeTest
    fun setup() {
        scheduler = TestCoroutineScheduler()
        dispatcher = StandardTestDispatcher(scheduler)

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
        scheduler.advanceUntilIdle()
        assertEquals(SessionState.None, sessionManager.get().value)
    }

    @Test
    fun `given tokens then there should be a session`() = runTest {
        tokenState.value = SessionTokens("username", "authToken", "refreshToken")
        scheduler.advanceUntilIdle()
        assertEquals(SessionState.Authenticated("username"), sessionManager.get().value)
    }

    @Test
    fun `given a successful login there should be a session`() = runTest {
        everySuspend {
            tokenApi.login("user123", "password")
        } returns (LoginResponse("a token", "refresh token"))

        sessionManager.login("user123", "password")
        scheduler.advanceUntilIdle()

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
            scheduler.advanceUntilIdle()
        }

        assertTrue { result.isFailure }

        verify(exactly(0)) {
            tokenRepository.put(any())
        }

        assertNull(sessionManager.get().value)
    }
}
