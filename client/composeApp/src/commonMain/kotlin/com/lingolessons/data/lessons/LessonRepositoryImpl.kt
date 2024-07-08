package com.lingolessons.data.lessons

import com.lingolessons.data.common.ApiCallProcessor
import com.lingolessons.domain.lessons.Lesson
import com.lingolessons.domain.lessons.LessonList
import com.lingolessons.domain.lessons.LessonRequest
import com.lingolessons.domain.lessons.LessonRepository

internal class LessonRepositoryImpl(
    private val lessonApi: LessonApi,
) : LessonRepository(), ApiCallProcessor {
    override suspend fun getList(key: LessonRequest): LessonList =
        processCall({
            lessonApi.getLessons(
                owner = key.owner,
                title = key.title,
                since = key.since,
                page = key.page,
            )
        }) {
            it?.let { response ->
                LessonList(
                    total = response.count,
                    lessons = response.results.map { lesson ->
                        Lesson(
                            id = lesson.id,
                            title = lesson.title,
                            owner = lesson.owner,
                            updated = lesson.updated_at,
                        )
                    }.toList(),
                    hasMore = response.next != null,
                )
            } ?: LessonList()
        }

    override suspend fun add(value: Lesson) {
        TODO("Not yet implemented")
    }

    override suspend fun put(value: Lesson) {
        TODO("Not yet implemented")
    }

    override suspend fun delete(value: Lesson) {
        TODO("Not yet implemented")
    }
}
