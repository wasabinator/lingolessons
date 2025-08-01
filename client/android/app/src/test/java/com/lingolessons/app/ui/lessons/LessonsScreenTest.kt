package com.lingolessons.app.ui.lessons

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performTextInput
import androidx.compose.ui.test.runComposeUiTest
import androidx.paging.PagingData
import com.lingolessons.app.common.BaseUiTest
import com.lingolessons.app.common.MockMethod
import com.lingolessons.app.common.mockMethod
import com.lingolessons.app.ui.lessons.LessonsViewModel.State
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType
import kotlinx.coroutines.flow.emptyFlow
import kotlinx.coroutines.flow.flowOf
import org.junit.Test

@OptIn(ExperimentalTestApi::class)
class LessonsScreenTest : BaseUiTest() {
    private lateinit var mockSearchTextUpdated: MockMethod

    override fun setup() {
        mockSearchTextUpdated = mockMethod()
    }

    private fun ComposeUiTest.setContent(state: State) {
        setContent {
            LessonsScreen(
                state = state,
                onLessonSelected = {},
                updateStatus = {},
                onSearchTextChanged = { mockSearchTextUpdated.call(it) },
            )
        }
    }

    @Test
    fun `expect screen to be empty when state is empty`() = runComposeUiTest {
        setContent(State(
            lessons = emptyFlow()
        ))

        onNodeWithTag("lesson_list").assertDoesNotExist()
    }

    @Test
    fun `expect screen to display lessons when state is not empty`() = runComposeUiTest {
        setContent(State(
            lessons = flowOf(
                PagingData.from(
                    listOf(
                        Lesson(
                            id = "",
                            title = "title",
                            type = LessonType.GRAMMAR,
                            language1 = "en",
                            language2 = "jp",
                            owner = "owner",
                            updatedAt = DateTime.now()
                        )
                    )
                )
            )
        ))

        onNodeWithTag("lesson_list").assertExists()
    }

    @Test
    fun `expect callback when text changed`() = runComposeUiTest {
        setContent(State(
            lessons = flowOf(
                PagingData.from(
                    listOf(
                        Lesson(
                            id = "",
                            title = "title",
                            type = LessonType.GRAMMAR,
                            language1 = "en",
                            language2 = "jp",
                            owner = "owner",
                            updatedAt = DateTime.now()
                        )
                    )
                )
            )
        ))

        onNodeWithTag("search_text").performTextInput("123")

        //mockSearchTextUpdated.expect("123")
    }
}
