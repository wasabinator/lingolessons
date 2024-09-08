package com.lingolessons.domain.lessons

data class LessonRequest(
    val title: String? = null,
    val owner: String? = null,
    val since: Int? = null,
    val page: Int = 1,
)
