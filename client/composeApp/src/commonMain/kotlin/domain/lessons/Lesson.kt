package domain.lessons

data class Lesson(
    val id: Int,
    val title: String,
    val owner: String,
    val updated: Int,
)
