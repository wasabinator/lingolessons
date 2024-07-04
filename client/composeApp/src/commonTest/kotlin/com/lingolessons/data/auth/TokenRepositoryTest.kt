package com.lingolessons.data.auth

import com.lingolessons.data.db.TokenDao
import dev.mokkery.answering.returns
import dev.mokkery.every
import dev.mokkery.mock
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestCoroutineScheduler
import kotlinx.coroutines.test.TestDispatcher
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertNull

class TokenRepositoryTest {
    private lateinit var dao: TokenDao
    private lateinit var repository: TokenRepository

    private lateinit var scheduler: TestCoroutineScheduler
    private lateinit var dispatcher: TestDispatcher

    @BeforeTest
    fun setup() {
        scheduler = TestCoroutineScheduler()
        dispatcher = StandardTestDispatcher(scheduler)

        dao = mock()

        repository = TokenRepositoryImpl(
            dispatcher = dispatcher,
            dao = dao
        )
    }

    @Test
    fun `should return null when there is no token`() {
        every {
            dao.get()
        } returns null

        scheduler.runCurrent()

        assertNull(repository.get().value)
    }
}