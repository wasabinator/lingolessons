package com.lingolessons.data.auth

import com.lingolessons.domain.auth.SessionManager
import com.lingolessons.domain.auth.SessionState
import dev.mokkery.answering.returns
import dev.mokkery.every
import dev.mokkery.mock
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestCoroutineScheduler
import kotlinx.coroutines.test.TestDispatcher
import kotlinx.coroutines.test.runTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals

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
            }.returns(
                tokenState
            )
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

}
