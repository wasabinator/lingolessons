package com.lingolessons.app

import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.test.core.app.ActivityScenario
import com.lingolessons.app.common.BaseUiTest
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.mockk
import org.junit.Rule
import org.junit.Test
import org.koin.core.context.GlobalContext.loadKoinModules
import org.koin.dsl.module
import org.koin.test.KoinTest

class IntegrationTests: BaseUiTest(), KoinTest {
    private lateinit var scenario: ActivityScenario<MainActivity>

    private lateinit var mockSession: Session

    @get:Rule
    val composeTestRule = createEmptyComposeRule()

    override fun setup() {
        mockSession = Session.None

        loadKoinModules(
            module {
                single<DomainInterface> {
                    mockk {
                        coEvery { getSession() } answers { mockSession }
                    }
                }
            }
        )
        scenario = ActivityScenario.launch(MainActivity::class.java)
    }

    override fun teardown() {
        scenario.close()
    }

    @Test
    fun `expect to see the the login screen on launch when there is no session`() {
        scenario.onActivity {
            composeTestRule.onNodeWithTag("username").assertExists()
        }
    }
}
