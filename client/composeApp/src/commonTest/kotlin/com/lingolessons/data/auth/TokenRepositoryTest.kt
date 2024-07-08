package com.lingolessons.data.auth

import com.lingolessons.common.BaseTest
import com.lingolessons.data.db.TokenDao
import dev.mokkery.answering.returns
import dev.mokkery.every
import dev.mokkery.mock
import kotlin.test.Test
import kotlin.test.assertNull

class TokenRepositoryTest : BaseTest() {
    private lateinit var dao: TokenDao
    private lateinit var repository: TokenRepository

    override fun setup() {
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

        advanceUntilIdle()
        assertNull(repository.get().value)
    }
}