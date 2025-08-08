package com.lingolessons.app.ui.login

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.onNodeWithTag

@OptIn(ExperimentalTestApi::class)
class LoginScreenRobot(private val composeTest: ComposeUiTest) {
    fun assertIsShown() =
        with(composeTest) {
            onNodeWithTag("username").assertExists()
            onNodeWithTag("password").assertExists()
        }
}
