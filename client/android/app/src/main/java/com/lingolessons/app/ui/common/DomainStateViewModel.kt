package com.lingolessons.app.ui.common

import androidx.lifecycle.ViewModel
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.ScreenState.Status
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

abstract class DomainStateViewModel<T>(
    val domainState: DomainState,
    initData: T,
) : ViewModel() {
    private val _state = MutableStateFlow<ScreenState<T>>(ScreenState(initData))
    val state = _state.asStateFlow()

    protected val data
        get() = _state.value.data

    protected fun updateData(update: (T) -> T) {
        _state.update {
            it.copy(
                data = update(it.data),
                status = Status.None,
            )
        }
    }

    protected fun updateStatus(status: Status) {
        _state.update { it.copy(status = status) }
    }

    fun clearStatus() {
        _state.update { it.clearStatus() }
    }
}
