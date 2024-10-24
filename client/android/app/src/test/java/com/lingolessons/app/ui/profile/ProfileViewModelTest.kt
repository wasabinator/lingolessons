package com.lingolessons.app.ui.profile

import com.lingolessons.app.common.BaseTest
import com.lingolessons.app.domain.DomainState
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.mockk
import org.junit.Test

class ProfileViewModelTest : BaseTest() {
    private lateinit var domain: DomainInterface
    private lateinit var domainState: DomainState
    private lateinit var viewModel: ProfileViewModel

    override fun setup() {
        domain = mockk<DomainInterface>().apply {
            coEvery { getSession() } returns Session.None
        }
        domainState = DomainState(
            domain = domain
        )
        viewModel = ProfileViewModel(domainState)
    }

    @Test
    fun `expect action invoked when logout is performed`() {
        viewModel.logout()
        advanceUntilIdle()

        coVerify(exactly = 1) {
            domain.logout()
        }
    }
}
