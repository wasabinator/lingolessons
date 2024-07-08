package com.lingolessons.ui.lessons

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lingolessons.domain.lessons.GetLessons
import com.lingolessons.domain.lessons.Lesson
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class LessonsViewModel(
    private val action: GetLessons
) : ViewModel() {
    private val _state = MutableStateFlow<State?>(null)
    val state = _state.asStateFlow()

    init {
        viewModelScope.launch {
            action.perform(1).onSuccess {
                _state.value = State(it.total, it.lessons)
            }
        }
    }

    data class State(
        val total: Int,
        val lessons: List<Lesson>
    )
}
