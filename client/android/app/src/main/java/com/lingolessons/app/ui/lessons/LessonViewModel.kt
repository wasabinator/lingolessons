package com.lingolessons.app.ui.lessons

import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.DomainStateViewModel
import com.lingolessons.app.ui.common.ScreenState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

class LessonViewModel(
    domainState: DomainState,
    lessonId: String,
) : DomainStateViewModel(domainState = domainState) {
    private val _state = MutableStateFlow(
        State(
            lessonId = lessonId,
        )
    )
    val state = _state.asStateFlow()

    data class State(
        val lessonId: String,
        override val status: ScreenState.Status = ScreenState.Status.None,
    ) : ScreenState
}
