package com.lingolessons.ui.common

data class ViewModelState<T>(
    val status: Status = Status.Loading,
    val data: T? = null
) {
    fun toLoadingState() = copy(status = Status.Loading)
    fun toSuccessState(data: T) = copy(status = Status.Success, data = data)
    fun toErrorState(cause: Throwable) = copy(status = Status.Error(cause))

    fun isLoading() = status == Status.Loading
    fun isSuccess() = status == Status.Success
    fun isError() = status is Status.Error
}

sealed class Status {
    data object Loading : Status()
    data object Success : Status()
    data class Error(val cause: Throwable) : Status()
}
