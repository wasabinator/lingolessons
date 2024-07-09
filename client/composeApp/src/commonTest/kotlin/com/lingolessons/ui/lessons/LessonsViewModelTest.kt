package com.lingolessons.ui.lessons

import com.lingolessons.common.BaseTest
import com.lingolessons.domain.lessons.GetLessons
import com.lingolessons.domain.lessons.LessonList
import dev.mokkery.answering.returns
import dev.mokkery.everySuspend
import dev.mokkery.matcher.any
import dev.mokkery.matcher.eq
import dev.mokkery.mock
import dev.mokkery.verify.VerifyMode.Companion.exactly
import dev.mokkery.verifySuspend
import kotlin.test.Test

class LessonsViewModelTest : BaseTest() {
    private lateinit var action: GetLessons
    private lateinit var viewModel: LessonsViewModel

    override fun setup() {
        action = mock {
            everySuspend {
                perform(any())
            } returns Result.success(LessonList())
        }

        viewModel = LessonsViewModel(action)
    }

    @Test
    fun expectInteractionWithActionWhenGetLessonsIsPerformed() = runTest {
        advanceUntilIdle()

        verifySuspend(exactly(1)) {
            action.perform(eq(1))
        }
    }
}
