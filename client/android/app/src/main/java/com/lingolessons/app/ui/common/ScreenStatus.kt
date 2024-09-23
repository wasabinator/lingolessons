package com.lingolessons.app.ui.common

interface ScreenState {
    val status: Status

    sealed class Status {
        data object None : Status()
        data object Busy : Status()
        data class Error(
            val message: String,
            val canRetry: Boolean = false,
        ) : Status()
    }

    val isBusy get() = status == Status.Busy
    val isError get() = status is Status.Error
    val errorMessage get() = (status as? Status.Error)?.message
}
