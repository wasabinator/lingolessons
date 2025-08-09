package com.lingolessons.app.domain.lessons

import androidx.paging.PagingSource.LoadParams.Refresh
import androidx.paging.PagingSource.LoadResult
import com.lingolessons.app.common.BaseTest
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.mockk
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class LessonPagingSourceTest : BaseTest() {
    private lateinit var domain: DomainInterface
    private lateinit var mockLessons: List<Lesson>

    override fun setup() {
        mockLessons =
            listOf(
                Lesson(
                    id = "",
                    title = "lesson1",
                    type = LessonType.VOCABULARY,
                    language1 = "en",
                    language2 = "jp",
                    owner = "owner",
                    updatedAt = DateTime.now()))
        domain =
            mockk<DomainInterface>().apply {
                coEvery { getSession() } returns Session.Authenticated("user")
                coEvery { getLessons(0u) } returns mockLessons
                coEvery { getLessons(1u) } returns emptyList()
            }
    }

    @Test
    fun `should call api on pager load`() = runTest {
        val pagingSource = LessonsPagingSource(domain = domain)

        val result: LoadResult<Int, Lesson> =
            pagingSource.load(Refresh(key = null, loadSize = 2, placeholdersEnabled = false))

        val page = result as? LoadResult.Page
        assertTrue(page != null)

        assertEquals(LoadResult.Page(data = mockLessons, prevKey = null, nextKey = 1), result)

        coVerify(exactly = 1) { domain.getLessons(0u) }

        val result2: LoadResult<Int, Lesson> =
            pagingSource.load(
                Refresh(
                    key = 1,
                    loadSize = 2,
                    placeholdersEnabled = false,
                ),
            )

        coVerify(exactly = 1) { domain.getLessons(1u) }
    }
}
