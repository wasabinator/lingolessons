package com.lingolessons.ui.login

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.runComposeUiTest
import com.lingolessons.common.MockMethod
import com.lingolessons.common.mockMethod
import com.lingolessons.ui.login.LoginViewModel.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlin.test.Test
import androidx.compose.ui.test.*
import com.lingolessons.common.BaseUiTest

@OptIn(ExperimentalTestApi::class)
class LoginScreenTest : BaseUiTest() {
    private lateinit var state: MutableStateFlow<LoginState>

    private lateinit var updateUsername: MockMethod
    private lateinit var updatePassword: MockMethod
    private lateinit var login: MockMethod
    private lateinit var dismissDialog: MockMethod

    override fun setup() {
        state = MutableStateFlow(LoginState())
        updateUsername = mockMethod()
        updatePassword = mockMethod()
        login = mockMethod()
        dismissDialog = mockMethod()
    }

    private fun ComposeUiTest.setContent(state: LoginState) {
        setContent {
            LoginScreen(
                state = state,
                updateUsername = { updateUsername.call(it) },
                updatePassword = { updatePassword.call(it) },
                login = { login.call() },
                dismissDialog = { dismissDialog.call() },
            )
        }
    }

    @Test
    fun expectScreenToBeEmptyWhenStateIsEmpty() = runComposeUiTest {
        setContent(LoginState())

        onNodeWithTag("username").assertExists() // TODO
    }
}
