package com.lingolessons.ui.lesson

import androidx.lifecycle.ViewModel
import com.lingolessons.domain.lessons.CreateLesson
import com.lingolessons.domain.lessons.Lesson
import com.lingolessons.ui.common.ViewModelState
import com.lingolessons.ui.lessons.LessonsViewModel.LessonData
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

class LessonViewModel(
    private val action: CreateLesson
) : ViewModel() {
    private val _state = MutableStateFlow(ViewModelState<LessonData>())
    val state = _state.asStateFlow()

    data class LessonData(
        val title: String,
        val type: LessonType,
        val language1: String,
        val language2: String,
        val page: Int = 0,
        val total: Int,
        val lessons: List<Lesson>,
    )

    fun LessonData?.plus(other: LessonData) = LessonData(
        page = other.page,
        total = this?.total ?: other.total,
        lessons = (this?.lessons ?: emptyList()).plus(other.lessons)
    )

    enum class LessonType {
        Vocabulary,
        Grammar,
    }
}
