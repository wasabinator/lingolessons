package com.lingolessons.data.repository.lessons

import com.lingolessons.data.api.common.ApiCallProcessor
import com.lingolessons.data.api.lessons.LessonsApi
import com.lingolessons.domain.lessons.Lesson
import com.lingolessons.domain.lessons.LessonList
import com.lingolessons.domain.lessons.LessonRequest
import com.lingolessons.domain.lessons.LessonRepository

internal class LessonRepositoryImpl(
    private val lessonsApi: LessonsApi,
) : LessonRepository(), ApiCallProcessor {
    override suspend fun getList(key: LessonRequest): LessonList =
        processCall({
            lessonsApi.getLessons(
                owner = key.owner,
                title = key.title,
                since = key.since,
                page = key.page,
            )
        }) {
            it?.let { response ->
                LessonList(
                    total = response.count,
                    lessons = response.results.map {
                        Lesson(
                            id = it.id,
                            title = it.title,
                            owner = it.owner,
                            updated = it.updated_at,
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
