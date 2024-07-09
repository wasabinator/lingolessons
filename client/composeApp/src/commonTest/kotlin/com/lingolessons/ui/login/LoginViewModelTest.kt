package com.lingolessons.ui.login

import com.lingolessons.common.BaseTest
import com.lingolessons.domain.auth.LoginUser
import com.lingolessons.domain.common.DomainError
import com.lingolessons.ui.common.ScreenState
import dev.mokkery.answering.returns
import dev.mokkery.everySuspend
import dev.mokkery.matcher.any
import dev.mokkery.mock
import kotlin.test.Test
import kotlin.test.assertEquals

class LoginViewModelTest : BaseTest() {
    private lateinit var action: LoginUser
    private lateinit var viewModel: LoginViewModel

    override fun setup() {
        action = mock()
        viewModel = LoginViewModel(action)
    }

    @Test
    fun expectUpdatedStateWhenUsernameIsUpdated() {
        viewModel.updateUsername("user123")
        advanceUntilIdle()
        assertEquals(
            LoginViewModel.LoginState(username = "user123", enabled = false),
            viewModel.state.value
        )
    }

    @Test
    fun expectUpdatedStateWhenPasswordIsUpdated() {
        viewModel.updatePassword("pass")
        advanceUntilIdle()
        assertEquals(
            LoginViewModel.LoginState(password = "pass", enabled = false),
            viewModel.state.value
        )
    }

    @Test
    fun expectEnabledStateWhenUsernameAndPasswordAreValid() = runTest {
        viewModel.updateUsername("user1234")
        viewModel.updatePassword("pass")
        assertEquals(
            LoginViewModel.LoginState(username = "user1234", password = "pass", enabled = true),
            viewModel.state.value
        )
    }

    @Test
    fun expectBusyStateWhenLoginIsPerformed() = runTest {
        viewModel.updateUsername("user1234")
        viewModel.updatePassword("pass")

        everySuspend {
            action.perform(any())
        } returns Result.success(Unit)

        viewModel.login()

        val expectedState = LoginViewModel.LoginState(
            username = "user1234",
            password = "pass",
            enabled = true,
            status = ScreenState.Status.Busy
        )

        assertEquals(
            expectedState,
            viewModel.state.value
        )

        // Allow login to complete
        advanceUntilIdle()

        assertEquals(
            expectedState.copy(
                status = ScreenState.Status.None
            ),
            viewModel.state.value
        )
    }

    @Test
    fun expectErrorStateWhenLoginFails() = runTest {
        viewModel.updateUsername("user1234")
        viewModel.updatePassword("pass")

        everySuspend {
            action.perform(any())
        } returns Result.failure(DomainError(userFacingError = "An error", cause = Exception()))

        viewModel.login()
        advanceUntilIdle()

        assertEquals(
            LoginViewModel.LoginState(
                username = "user1234",
                password = "pass",
                enabled = true,
                status = ScreenState.Status.Error(message = "An error")
            ),
            viewModel.state.value
        )
    }
}
