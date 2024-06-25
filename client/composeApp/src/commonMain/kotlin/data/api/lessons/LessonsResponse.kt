package data.api.lessons

import data.api.common.PagedResponse
import kotlinx.serialization.Serializable

@Serializable
internal data class LessonsResponse(
    override var count: Int,
    override val next: String?,
    override val results: List<LessonResponse>,
) : PagedResponse<LessonResponse>

@Serializable
internal data class LessonResponse(
    val id: Int,
    val title: String,
    val owner: String,
    val updated_at: Int,
)
