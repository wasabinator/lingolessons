package com.lingolessons.data.lessons

import com.lingolessons.data.common.PagedResponse
import kotlinx.serialization.Serializable

@Serializable
internal data class LessonsResponse(
    override var count: Int,
    override val next: String?,
    override val results: List<LessonResponse>,
) : PagedResponse<LessonResponse>

@Serializable
internal data class LessonResponse(
    val id: String,
    val title: String,
    val owner: String,
    val updated_at: Int,
)
