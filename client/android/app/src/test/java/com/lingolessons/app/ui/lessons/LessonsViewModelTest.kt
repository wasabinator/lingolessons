package com.lingolessons.app.ui.lessons

import com.lingolessons.app.common.BaseTest
import com.lingolessons.app.domain.DomainState
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.mockk
import org.junit.Test

class LessonsViewModelTest : BaseTest() {
    private lateinit var domain: DomainInterface
    private lateinit var domainState: DomainState
    private lateinit var viewModel: LessonsViewModel

    override fun setup() {
        domain = mockk<DomainInterface>().apply {
            coEvery { getSession() } returns Session.Authenticated("user")
            coEvery { getLessons() } returns emptyList()
        }
        domainState = DomainState(
            domain = domain
        )
        viewModel = LessonsViewModel(domainState)
    }

    @Test
    fun `expect getLessons invoked when refresh is performed`() {
        viewModel.refresh()
        advanceUntilIdle()

        coVerify(exactly = 1) {
            domain.getLessons()
        }
    }
}
