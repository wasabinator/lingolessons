package com.lingolessons.app

import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.runAndroidComposeUiTest
import com.lingolessons.app.common.BaseUiTest
import com.lingolessons.app.ui.login.LoginScreenRobot
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.mockk
import org.junit.Test
import org.koin.core.context.GlobalContext.loadKoinModules
import org.koin.dsl.module
import org.koin.test.KoinTest

@OptIn(ExperimentalTestApi::class)
class IntegrationTests() : BaseUiTest(), KoinTest {
    private lateinit var mockSession: Session

    override fun setup() {
        mockSession = Session.None

        loadKoinModules(
            module {
                single<DomainInterface> {
                    mockk { coEvery { getSession() } answers { mockSession } }
                }
            })
    }

    @Test
    fun `expect to see the the login screen on launch when there is no session`() =
        runAndroidComposeUiTest(activityClass = MainActivity::class.java) {
            with(LoginScreenRobot(this)) { assertIsShown() }
        }
}
