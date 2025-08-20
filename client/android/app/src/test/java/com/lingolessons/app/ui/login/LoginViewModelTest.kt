package com.lingolessons.app.ui.login

import com.lingolessons.app.common.BaseTest
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.app.ui.login.LoginViewModel.Errors
import com.lingolessons.app.ui.login.LoginViewModel.ScreenData
import com.lingolessons.shared.DomainException
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.mockk
import junit.framework.TestCase.assertEquals
import org.junit.Test

class LoginViewModelTest : BaseTest() {
    private lateinit var domain: DomainInterface
    private lateinit var domainState: DomainState
    private lateinit var viewModel: LoginViewModel

    override fun setup() {
        domain = mockk<DomainInterface>().apply { coEvery { getSession() } returns Session.None }
        domainState = DomainState(domain = domain)
        viewModel = LoginViewModel(domainState)
    }

    @Test
    fun expectUpdatedStateWhenUsernameIsUpdated() {
        viewModel.updateUsername("user123")
        advanceUntilIdle()
        assertEquals(
            ScreenState(ScreenData(username = "user123", enabled = false)),
            viewModel.state.value,
        )
    }

    @Test
    fun expectUpdatedStateWhenPasswordIsUpdated() {
        viewModel.updatePassword("pass")
        advanceUntilIdle()
        assertEquals(
            ScreenState(ScreenData(password = "pass", enabled = false)),
            viewModel.state.value,
        )
    }

    @Test
    fun expectEnabledStateWhenUsernameAndPasswordAreValid() = runTest {
        viewModel.updateUsername("user1234")
        viewModel.updatePassword("pass")
        assertEquals(
            ScreenState(ScreenData(username = "user1234", password = "pass", enabled = true)),
            viewModel.state.value,
        )
    }

    @Test
    fun expectBusyStateWhenLoginIsPerformed() = runTest {
        viewModel.updateUsername("user1234")
        viewModel.updatePassword("pass")

        coEvery { domain.login(any(), any()) } returns Session.Authenticated("user")

        viewModel.login()

        val expectedState =
            ScreenState(
                ScreenData(
                    username = "user1234",
                    password = "pass",
                    enabled = true,
                ),
                status = ScreenState.Status.Busy,
            )

        assertEquals(expectedState, viewModel.state.value)

        // Allow login to complete
        advanceUntilIdle()

        assertEquals(expectedState.copy(status = ScreenState.Status.None), viewModel.state.value)
    }

    @Test
    fun expectErrorStateWhenLoginFails() = runTest {
        viewModel.updateUsername("user1234")
        viewModel.updatePassword("pass")

        coEvery { domain.login(any(), any()) } throws DomainException.Api("Error logging in")

        viewModel.login()
        advanceUntilIdle()

        assertEquals(
            ScreenState(
                ScreenData(
                    username = "user1234",
                    password = "pass",
                    enabled = true,
                ),
                status = ScreenState.Status.Error(Errors.UnknownError),
            ),
            viewModel.state.value,
        )
    }
}
