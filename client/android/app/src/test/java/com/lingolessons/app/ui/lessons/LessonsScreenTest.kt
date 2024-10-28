package com.lingolessons.app.ui.lessons

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.runComposeUiTest
import com.lingolessons.app.common.BaseUiTest
import com.lingolessons.app.ui.lessons.LessonsViewModel.State
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Test

@OptIn(ExperimentalTestApi::class)
class LessonsScreenTest : BaseUiTest() {
    private lateinit var state: MutableStateFlow<State>

    override fun setup() {
        state = MutableStateFlow(State())
    }

    private fun ComposeUiTest.setContent(state: State) {
        setContent {
            LessonsScreen(
                state = state,
                onLessonSelected = {},
                onRefresh = {},
            )
        }
    }

    @Test
    fun expectScreenToBeEmptyWhenStateIsEmpty() = runComposeUiTest {
        setContent(State())

        onNodeWithTag("lesson_list").assertDoesNotExist()
    }

    @Test
    fun expectScreenToDisplayLessonsWhenStateIsNotEmpty() = runComposeUiTest {
        setContent(State(
            lessons = listOf(
                Lesson(
                    id = "",
                    title = "",
                    type = LessonType.GRAMMAR,
                    language1 = "",
                    language2 = "",
                    owner = "",
                    updatedAt = DateTime.now()
                )
            )
        ))

        onNodeWithTag("lesson_list").assertExists()
    }
}