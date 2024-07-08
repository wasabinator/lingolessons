package com.lingolessons.data.lessons

import com.lingolessons.domain.lessons.LessonRepository
import com.lingolessons.domain.lessons.LessonRequest
import dev.mokkery.answering.returns
import dev.mokkery.everySuspend
import dev.mokkery.matcher.any
import dev.mokkery.mock
import dev.mokkery.verify.VerifyMode.Companion.exactly
import dev.mokkery.verifySuspend
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestCoroutineScheduler
import kotlinx.coroutines.test.TestDispatcher
import kotlinx.coroutines.test.runTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse

class LessonRepositoryTest {
    private lateinit var lessonApi: LessonApi
    private lateinit var repository: LessonRepository

    private lateinit var scheduler: TestCoroutineScheduler
    private lateinit var dispatcher: TestDispatcher

    @BeforeTest
    fun setup() {
        scheduler = TestCoroutineScheduler()
        dispatcher = StandardTestDispatcher(scheduler)

        lessonApi = mock()

        repository = LessonRepositoryImpl(
            lessonApi = lessonApi,
        )
    }

    @Test
    fun `given a request to fetch a page of lessons expect a call to the lesson api`() = runTest {
        everySuspend {
            lessonApi.getLessons(any())
        } returns LessonsResponse(
            count = 0,
            next = null,
            results = emptyList(),
        )

        val result = repository.getList(LessonRequest())

        verifySuspend(exactly(1)) {
            lessonApi.getLessons(
                owner = null,
                title = null,
                since = null,
                page = 1,
            )
        }

        assertEquals(result.total, 0)
        assertEquals(result.lessons.size, 0)
        assertFalse(result.hasMore)
    }
}
