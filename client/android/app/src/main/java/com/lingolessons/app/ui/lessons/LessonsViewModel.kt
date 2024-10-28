package com.lingolessons.app.ui.lessons

import androidx.lifecycle.viewModelScope
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.DomainStateViewModel
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.shared.DomainException
import com.lingolessons.shared.Lesson
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class LessonsViewModel(
    private val domainState: DomainState
) : DomainStateViewModel(domainState = domainState) {
    private val _state = MutableStateFlow(State())
    val state = _state.asStateFlow()

    fun refresh() {
        viewModelScope.launch {
            _state.update {
                it.copy(
                    status = ScreenState.Status.Busy,
                )
            }
            try {
                val lessons = domainState.domain.getLessons()
                _state.update {
                    it.copy(
                        lessons = lessons,
                        status = ScreenState.Status.None,
                    )
                }
            } catch(e: DomainException) {
                _state.update {
                    it.copy(
                        status = ScreenState.Status.Error(),
                    )
                }
            }
        }
    }

    data class State(
        val lessons: List<Lesson> = emptyList(),
        override val status: ScreenState.Status = ScreenState.Status.None,
    ) : ScreenState
}