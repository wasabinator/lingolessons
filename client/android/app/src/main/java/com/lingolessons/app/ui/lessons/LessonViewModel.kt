package com.lingolessons.app.ui.lessons

import androidx.lifecycle.viewModelScope
import com.lingolessons.app.common.logError
import com.lingolessons.app.common.logWarning
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.DomainStateViewModel
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.app.ui.common.ScreenState.Status
import com.lingolessons.shared.DomainException
import com.lingolessons.shared.Lesson
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class LessonViewModel(
    domainState: DomainState,
    lessonId: String,
) : DomainStateViewModel(domainState = domainState) {
    private val _state = MutableStateFlow(
        State(
            lessonId = lessonId,
            status = Status.Busy,
        )
    )
    val state = _state.asStateFlow()

    init {
        viewModelScope.launch {
            try {
                val lesson = domainState.domain.getLesson(lessonId)
                if (lesson != null) {
                    _state.update {
                        it.copy(
                            lesson = lesson,
                            status = Status.None,
                        )
                    }
                } else {
                    logWarning("Couldn't find lesson for id: $lessonId")
                    _state.update {
                        it.copy(
                            status = Status.Error(
                                "Couldn't find lesson",
                                canRetry = true
                            )
                        )
                    }
                }
            } catch (e: DomainException) {
                logError("Error fetching lesson", e)
            }
        }
    }

    data class State(
        val lessonId: String,
        val lesson: Lesson? = null,
        override val status: Status = Status.None,
    ) : ScreenState

    override fun updateStatus(status: Status) {
        _state.update {
            it.copy(
                status = status
            )
        }
    }
}
