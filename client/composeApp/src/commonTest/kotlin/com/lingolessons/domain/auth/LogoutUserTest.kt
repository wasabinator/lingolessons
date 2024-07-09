package com.lingolessons.domain.auth

import com.lingolessons.common.BaseTest
import dev.mokkery.answering.returns
import dev.mokkery.everySuspend
import dev.mokkery.mock
import dev.mokkery.verify.VerifyMode.Companion.exactly
import dev.mokkery.verifySuspend
import kotlin.test.Test

class LogoutUserTest : BaseTest() {
    private lateinit var sessionManager: SessionManager
    private lateinit var operation: LogoutUser

    override fun setup() {
        sessionManager = mock()
        operation = LogoutUserImpl(dispatcher, sessionManager)
    }

    @Test
    fun expectInteractionWithSessionManagerWhenLogoutIsPerformed() =
        runTest {
            everySuspend {
                sessionManager.logout()
            } returns Unit

            operation.perform(Unit)

            verifySuspend(exactly(1)) {
                sessionManager.logout()
            }
        }
}
