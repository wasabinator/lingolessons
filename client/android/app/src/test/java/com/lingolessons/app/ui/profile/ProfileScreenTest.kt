package com.lingolessons.app.ui.profile

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.runComposeUiTest
import com.lingolessons.app.common.BaseUiTest
import com.lingolessons.app.common.MockMethod
import com.lingolessons.app.common.mockMethod
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.app.ui.profile.ProfileViewModel.ScreenData
import org.junit.Test

@OptIn(ExperimentalTestApi::class)
class ProfileScreenTest : BaseUiTest() {
    private lateinit var logout: MockMethod

    override fun setup() {
        logout = mockMethod()
    }

    private fun ComposeUiTest.setContent() {
        setContent {
            ProfileScreen(
                state = ScreenState(ScreenData()),
                errorMessage = { "" },
                logout = { logout.call() },
                clearStatus = {},
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
