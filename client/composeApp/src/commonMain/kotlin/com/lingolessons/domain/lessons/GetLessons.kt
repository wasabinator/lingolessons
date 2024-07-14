package com.lingolessons.domain.lessons

import com.lingolessons.common.suspendCatching
import com.lingolessons.domain.Operation
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.withContext

interface GetLessons : Operation<Int, LessonList>

class GetLessonsImpl(
    private val dispatcher: CoroutineDispatcher,
    private val lessonRepository: LessonRepository,
) : GetLessons {
    override suspend fun perform(param: Int): Result<LessonList> = withContext(dispatcher) {
        suspendCatching {
            lessonRepository.getList(LessonRequest(page = param))
        }
    }
}
