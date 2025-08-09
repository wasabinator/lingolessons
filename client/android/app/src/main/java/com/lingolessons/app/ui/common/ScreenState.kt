package com.lingolessons.app.ui.common

interface ErrorSource {
    fun canRetry(): Boolean = true
}

interface ScreenState {
    val status: Status

    sealed class Status {
        data object None : Status()

        data object Busy : Status()

        data class Error(val source: ErrorSource) : Status()
    }

    val isBusy
        get() = status == Status.Busy

    val isError
        get() = status is Status.Error

    val error: Status.Error?
        get() = status as? Status.Error
}
