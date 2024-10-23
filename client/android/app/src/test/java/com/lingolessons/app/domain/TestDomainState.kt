package com.lingolessons.app.domain

import com.lingolessons.app.common.BaseTest
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.mockk
import junit.framework.TestCase.assertTrue
import org.junit.Test

class TestDomainState : BaseTest() {
    private lateinit var domain: DomainInterface
    private lateinit var domainState: DomainState
    private lateinit var currentSession: Session

    override fun setup() {
        currentSession = Session.None
        domain = mockk<DomainInterface>().apply {
            coEvery { getSession() } answers { currentSession }
        }
        domainState = DomainState(domain)
        advanceUntilIdle()
    }

    @Test
    fun expectOneRefreshOnInit() {
        coVerify(exactly = 1) { domain.getSession() }
        assertTrue(domainState.state.value == Session.None)
    }

    @Test
    fun expectRefreshWithUpdateTheStateFlow() {
        currentSession = Session.Authenticated("test user")
        domainState.refresh()
        advanceUntilIdle()
        coVerify(exactly = 2) { domain.getSession() }
        assertTrue(domainState.state.value == Session.Authenticated("test user"))
    }
}
