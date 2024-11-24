package com.lingolessons.app.ui.lessons

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.assertTextEquals
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.runComposeUiTest
import com.lingolessons.app.common.BaseUiTest
import com.lingolessons.app.common.MockMethod
import com.lingolessons.app.common.mockMethod
import com.lingolessons.app.ui.lessons.LessonViewModel.State
import org.junit.Test

@OptIn(ExperimentalTestApi::class)
class LessonScreenTest: BaseUiTest() {
    private lateinit var mockSearchTextUpdated: MockMethod

    override fun setup() {
        mockSearchTextUpdated = mockMethod()
    }

    private fun ComposeUiTest.setContent(state: State) {
        setContent {
            LessonScreen(
                state = state,
            )
        }
    }

    @Test
    fun `expect screen placeholder to exist`() = runComposeUiTest {
        setContent(State(
            lessonId = "123"
        ))

        onNodeWithTag("screen_title")
            .assertExists()
            .assertTextEquals("123")
    }
}
