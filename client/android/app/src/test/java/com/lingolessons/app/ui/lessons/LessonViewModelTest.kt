package com.lingolessons.app.ui.lessons

import com.lingolessons.app.common.BaseTest
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType
import com.lingolessons.shared.Session
import io.mockk.coEvery
import io.mockk.mockk
import junit.framework.TestCase.assertEquals
import junit.framework.TestCase.assertTrue
import org.junit.Test

class LessonViewModelTest : BaseTest() {
    private lateinit var domain: DomainInterface
    private lateinit var domainState: DomainState
    private lateinit var viewModel: LessonViewModel
    private lateinit var mockLesson: Lesson

    override fun setup() {
        mockLesson = Lesson(
            id = "123",
            title = "lesson1",
            type = LessonType.VOCABULARY,
            language1 = "en",
            language2 = "jp",
            owner = "owner",
            updatedAt = DateTime.now()
        )
        domain = mockk<DomainInterface>().apply {
            coEvery { getSession() } returns Session.Authenticated("user")
        }
        domainState = DomainState(domain = domain)
    }

    @Test
    fun `expect initial state to match the specified lesson`() {
        coEvery { domain.getLesson("123") } returns mockLesson
        viewModel = LessonViewModel(domainState = domainState, lessonId = mockLesson.id)
        advanceUntilIdle()

        assertEquals(mockLesson.id, viewModel.state.value.lessonId)
        assertEquals(ScreenState.Status.None, viewModel.state.value.status)
    }

    @Test
    fun `expect error state if no lesson available`() {
        coEvery { domain.getLesson("123") } returns null
        viewModel = LessonViewModel(domainState = domainState, lessonId = mockLesson.id)
        advanceUntilIdle()

        assertTrue(viewModel.state.value.status is ScreenState.Status.Error)
    }
}
