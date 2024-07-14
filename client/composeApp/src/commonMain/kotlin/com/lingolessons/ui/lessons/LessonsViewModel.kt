package com.lingolessons.ui.lessons

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lingolessons.domain.lessons.GetLessons
import com.lingolessons.domain.lessons.Lesson
import com.lingolessons.ui.common.ViewModelState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class LessonsViewModel(
    private val action: GetLessons
) : ViewModel() {
    private val _state = MutableStateFlow(ViewModelState<LessonData>())
    val state = _state.asStateFlow()

    init {
        loadNextPage()
    }

    fun loadNextPage() {
        viewModelScope.launch {
            _state.update { it.toLoadingState() }
            val page = state.value.data?.page ?: 0
            action.perform(page + 1)
                .onSuccess { result ->
                    _state.update {
                        it.toSuccessState(
                            data = it.data.plus(
                                LessonData(
                                    page = result.page,
                                    total = result.total,
                                    lessons = result.lessons
                                )
                            )
                        )
                    }
                }
                .onFailure { error ->
                    _state.update {
                        it.toErrorState(cause = error)
                    }
                }
        }
    }

    data class LessonData(
        val page: Int = 0,
        val total: Int,
        val lessons: List<Lesson>,
    )

    fun LessonData?.plus(other: LessonData) = LessonData(
        page = other.page,
        total = this?.total ?: other.total,
        lessons = (this?.lessons ?: emptyList()).plus(other.lessons)
    )
}
