package com.lingolessons.ui.profile

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.runComposeUiTest
import com.lingolessons.common.BaseUiTest
import com.lingolessons.common.MockMethod
import com.lingolessons.common.mockMethod
import kotlin.test.Test

@OptIn(ExperimentalTestApi::class)
class ProfileScreenTest : BaseUiTest() {
    private lateinit var logout: MockMethod

    override fun setup() {
        logout = mockMethod()
    }

    private fun ComposeUiTest.setContent() {
        setContent {
            ProfileScreen(
                logout = { logout.call() },
            )
        }
    }

    @Test
    fun `should perform logout on click`() = runComposeUiTest {
        setContent()

        onNodeWithTag("logout").performClick()
        logout.verify(times = 1)
    }
}
