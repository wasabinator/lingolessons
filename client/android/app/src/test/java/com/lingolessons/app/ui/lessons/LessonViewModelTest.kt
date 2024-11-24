package com.lingolessons.app.ui.lessons

import androidx.paging.PagingSourceFactory
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
import org.junit.Test

class LessonViewModelTest  : BaseTest() {
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
            updatedAt = DateTime.now(),
        )
        domain = mockk<DomainInterface>().apply {
            coEvery { getSession() } returns Session.Authenticated("user")
        }
        domainState = DomainState(
            domain = domain
        )
        viewModel = LessonViewModel(
            domainState = domainState,
            lessonId = mockLesson.id,
        )
    }

    @Test
    fun `expect initial state to match the specified lesson`() {
        assertEquals(mockLesson.id, viewModel.state.value.lessonId)
        assertEquals(ScreenState.Status.None, viewModel.state.value.status)
   }
}
