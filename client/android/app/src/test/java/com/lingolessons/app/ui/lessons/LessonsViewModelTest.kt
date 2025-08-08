package com.lingolessons.app.ui.lessons

import androidx.paging.PagingSourceFactory
import androidx.paging.testing.asPagingSourceFactory
import com.lingolessons.app.common.BaseTest
import com.lingolessons.app.domain.DomainState
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.mockk
import junit.framework.TestCase.assertEquals
import org.junit.Test

class LessonsViewModelTest : BaseTest() {
    private lateinit var domain: DomainInterface
    private lateinit var domainState: DomainState
    private lateinit var viewModel: LessonsViewModel
    private lateinit var mockLessons: List<Lesson>
    private lateinit var pagingSources: MutableList<PagingSourceFactory<Int, Lesson>>

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
                    updatedAt = DateTime.now(),
                ),
            )

        pagingSources = mutableListOf()
        domain =
            mockk<DomainInterface>().apply {
                coEvery { getSession() } returns Session.Authenticated("user")
            }
        domainState = DomainState(domain = domain)
        viewModel =
            LessonsViewModel(domainState = domainState) { _ ->
                mockLessons.asPagingSourceFactory().also { pagingSources.add(it) }
            }
    }

    @Test
    fun `expect refresh to create a paging source`() {
        // Expect an initial pager to be created
        assertEquals(1, pagingSources.count())

        viewModel.refresh()

        // Expect a new pager on each refresh
        assertEquals(2, pagingSources.count())
    }

    @Test
    fun `expect search text filtering to create new paging sources`() {
        assertEquals(1, pagingSources.count())

        viewModel.updateFilterText("lesson")
        advanceUntilIdle()

        assertEquals(2, pagingSources.count())

        viewModel.updateFilterText("lessons")
        advanceUntilIdle()

        assertEquals(3, pagingSources.count())
    }
}
