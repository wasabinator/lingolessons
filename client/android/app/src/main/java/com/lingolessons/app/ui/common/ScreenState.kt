package com.lingolessons.app.ui.common

interface ErrorSource {
    fun canRetry(): Boolean = true
}

data class ScreenState<T>(
    val data: T,
    val status: Status = Status.None,
) {
    sealed class Status {
        data object None : Status()

        data object Busy : Status()

        data class Error(val source: ErrorSource) : Status()
    }

    val error: Status.Error?
        get() = status as? Status.Error

    fun clearStatus() = copy(status = Status.None)
}
