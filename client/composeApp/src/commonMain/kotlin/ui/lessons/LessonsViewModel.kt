package ui.lessons

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import domain.lessons.GetLessons
import domain.lessons.Lesson
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import org.koin.core.component.KoinComponent

class LessonsViewModel(
    private val action: GetLessons
) : ViewModel(), KoinComponent {
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
