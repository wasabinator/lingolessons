package com.lingolessons.app.ui.lessons

import com.lingolessons.app.common.BaseTest
import com.lingolessons.app.domain.DomainState
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.mockk
import junit.framework.TestCase.assertEquals
import org.junit.Test

class LessonsViewModelTest : BaseTest() {
    private lateinit var domain: DomainInterface
    private lateinit var domainState: DomainState
    private lateinit var viewModel: LessonsViewModel
    private lateinit var mockLessons: MutableList<Lesson>

    override fun setup() {
        mockLessons = mutableListOf()
        domain = mockk<DomainInterface>().apply {
            coEvery { getSession() } returns Session.Authenticated("user")
            coEvery { getLessons(any()) } returns mockLessons
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
            domain.getLessons(any())
        }
    }

    @Test
    fun `expect lesson filtering to work when filter text is specified`() {
        mockLessons.add(
            Lesson(
                id = "",
                title = "lesson1",
                type = LessonType.VOCABULARY,
                language1 = "en",
                language2 = "jp",
                owner = "owner",
                updatedAt = DateTime.now(),
            )
        )
        viewModel.refresh()
        advanceUntilIdle()

        coVerify(exactly = 1) {
            domain.getLessons(any())
        }

        assertEquals(1, viewModel.state.value.lessons.size)

        viewModel.updateFilterText("lesson")
        advanceUntilIdle()

        coVerify(exactly = 2) {
            domain.getLessons(any())
        }

        assertEquals(1, viewModel.state.value.lessons.size)

        viewModel.updateFilterText("lesson2")
        advanceUntilIdle()

        coVerify(exactly = 3) {
            domain.getLessons(any())
        }

        assertEquals(0, viewModel.state.value.lessons.size)
    }

    @Test
    fun `search text filtering should be case insensitive`() {
        mockLessons.add(
            Lesson(
                id = "",
                title = "lesson1",
                type = LessonType.VOCABULARY,
                language1 = "en",
                language2 = "jp",
                owner = "owner",
                updatedAt = DateTime.now(),
            )
        )
        viewModel.refresh()
        advanceUntilIdle()

        assertEquals(1, viewModel.state.value.lessons.size)

        viewModel.updateFilterText("lesson")
        advanceUntilIdle()

        assertEquals(1, viewModel.state.value.lessons.size)

        viewModel.updateFilterText("LESSON")
        advanceUntilIdle()

        assertEquals(1, viewModel.state.value.lessons.size)
    }
}
