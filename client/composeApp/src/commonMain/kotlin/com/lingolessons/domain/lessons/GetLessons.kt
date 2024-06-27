package com.lingolessons.domain.lessons

import com.lingolessons.common.suspendCatching
import com.lingolessons.domain.Operation
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.withContext

class GetLessons(
    private val dispatcher: CoroutineDispatcher,
    private val lessonRepository: LessonRepository,
) : Operation<Int, LessonList> {
    override suspend fun perform(param: Int): Result<LessonList> =
        withContext(dispatcher) {
            suspendCatching {
                lessonRepository.getList(LessonRequest())
            }.onSuccess {
                println("OK $it")
            }.onFailure {
                println("YIKES $it")
            }
        }
}
