package com.lingolessons.app.ui.lessons

import androidx.lifecycle.viewModelScope
import com.lingolessons.app.common.logError
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.DomainStateViewModel
import com.lingolessons.app.ui.common.ErrorSource
import com.lingolessons.app.ui.common.ScreenState.Status
import com.lingolessons.app.ui.lessons.LessonViewModel.ScreenData
import com.lingolessons.shared.DomainException
import com.lingolessons.shared.Lesson
import kotlinx.coroutines.launch

class LessonViewModel(domainState: DomainState, lessonId: String) :
    DomainStateViewModel<ScreenData>(
        domainState = domainState,
        initData = ScreenData(lessonId = lessonId),
    ) {

    init {
        viewModelScope.launch {
            updateStatus(Status.Busy)
            try {
                val lesson = domainState.domain.getLesson(lessonId)
                if (lesson != null) {
                    updateData { it.copy(lesson = lesson) }
                } else {
                    logError("Couldn't find lesson for id: $lessonId")
                    updateStatus(Status.Error(Errors.UnknownLesson))
                }
            } catch (e: DomainException) {
                logError("Error fetching lesson", e)
                updateStatus(Status.Error(Errors.UnknownError))
            }
        }
    }

    enum class Errors : ErrorSource {
        UnknownLesson,
        UnknownError
    }

    data class ScreenData(
        val lessonId: String,
        val lesson: Lesson? = null,
    )
}
