package com.lingolessons.domain.auth

import com.lingolessons.common.BaseTest
import dev.mokkery.answering.returns
import dev.mokkery.everySuspend
import dev.mokkery.mock
import dev.mokkery.verify.VerifyMode.Companion.exactly
import dev.mokkery.verifySuspend
import kotlin.test.Test

class LoginUserTest : BaseTest() {
    private lateinit var sessionManager: SessionManager
    private lateinit var operation: LoginUser

    override fun setup() {
        sessionManager = mock()
        operation = LoginUserImpl(dispatcher, sessionManager)
    }

    @Test
    fun `expect an interaction with the session manager when login is performed`() = runTest {
        everySuspend {
            sessionManager.login("user123", "password")
        } returns Unit

        operation.perform(LoginDetails("user123", "password"))

        verifySuspend(exactly(1)) {
            sessionManager.login("user123", "password")
        }
    }
}
