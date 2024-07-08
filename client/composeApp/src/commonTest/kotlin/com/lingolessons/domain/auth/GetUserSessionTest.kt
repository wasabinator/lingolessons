package com.lingolessons.domain.auth

import com.lingolessons.common.BaseTest
import dev.mokkery.answering.returns
import dev.mokkery.everySuspend
import dev.mokkery.mock
import dev.mokkery.verify.VerifyMode.Companion.exactly
import dev.mokkery.verifySuspend
import kotlinx.coroutines.flow.MutableStateFlow
import kotlin.test.Test

class GetUserSessionTest : BaseTest() {
    private lateinit var sessionManager: SessionManager

    private lateinit var operation: GetUserSession

    override fun setup() {
        sessionManager = mock {
            everySuspend { get() } returns MutableStateFlow(null)
        }
        operation = GetUserSessionImpl(sessionManager)
    }

    @Test
    fun `expect an interaction with the session manager when get is performed`() {
        operation.perform(Unit)

        verifySuspend(exactly(1)) {
            sessionManager.get()
        }
    }
}