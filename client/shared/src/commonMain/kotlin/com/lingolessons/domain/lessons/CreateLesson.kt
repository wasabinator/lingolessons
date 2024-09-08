package com.lingolessons.domain.lessons

import com.lingolessons.domain.Operation
import kotlinx.coroutines.CoroutineDispatcher

interface CreateLesson : Operation<Int, Lesson>

class CreateLessonImpl(
    private val dispatcher: CoroutineDispatcher,
    private val lessonRepository: LessonRepository,
) : CreateLesson {
    override suspend fun perform(param: Int): Result<Lesson> {
        TODO("Not yet implemented")
    }
}
