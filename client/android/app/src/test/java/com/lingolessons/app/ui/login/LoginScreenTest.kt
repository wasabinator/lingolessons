package com.lingolessons.app.ui.login

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.runComposeUiTest
import com.lingolessons.app.common.BaseUiTest
import com.lingolessons.app.common.MockMethod
import com.lingolessons.app.common.mockMethod
import com.lingolessons.app.ui.login.LoginViewModel.State
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Test

@OptIn(ExperimentalTestApi::class)
class LoginScreenTest : BaseUiTest() {
    private lateinit var state: MutableStateFlow<State>

    private lateinit var updateUsername: MockMethod
    private lateinit var updatePassword: MockMethod
    private lateinit var login: MockMethod
    private lateinit var dismissDialog: MockMethod

    override fun setup() {
        state = MutableStateFlow(State())
        updateUsername = mockMethod()
        updatePassword = mockMethod()
        login = mockMethod()
        dismissDialog = mockMethod()
    }

    private fun ComposeUiTest.setContent(state: State) {
        setContent {
            LoginScreen(
                state = state,
                updateUsername = { updateUsername.call(it) },
                updatePassword = { updatePassword.call(it) },
                login = { login.call() },
                dismissDialog = { dismissDialog.call() }
            )
        }
    }

    @Test
    fun `expect screen to be empty when state is empty`() = runComposeUiTest {
        setContent(State())

        with(LoginScreenRobot(this)) { assertIsShown() }
    }
}
