package com.lingolessons.ui.profile

import com.lingolessons.common.BaseTest
import com.lingolessons.domain.auth.LogoutUser
import dev.mokkery.answering.returns
import dev.mokkery.everySuspend
import dev.mokkery.mock
import dev.mokkery.verify.VerifyMode.Companion.exactly
import dev.mokkery.verifySuspend
import kotlin.test.Test

class ProfileViewModelTest : BaseTest() {
    private lateinit var action: LogoutUser
    private lateinit var viewModel: ProfileViewModel

    override fun setup() {
        action = mock() {
            everySuspend {
                perform(Unit)
            } returns Result.success(Unit)
        }
        viewModel = ProfileViewModel(action)
    }

    @Test
    fun `expect action invoked when logout is performed`() {
        viewModel.logout()
        advanceUntilIdle()

        verifySuspend(exactly(1)) {
            action.perform(Unit)
        }
    }
}
