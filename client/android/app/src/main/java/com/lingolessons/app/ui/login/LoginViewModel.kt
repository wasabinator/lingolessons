package com.lingolessons.app.ui.login

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.ErrorSource
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.app.ui.common.ScreenState.Status
import com.lingolessons.shared.AuthError
import com.lingolessons.shared.DomainException
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class LoginViewModel(private val domainState: DomainState) : ViewModel() {
    private val _state = MutableStateFlow(State())
    val state = _state.asStateFlow()

    fun updateUsername(username: String) {
        _state.update {
            it.copy(
                username = username,
                enabled = it.username.isNotBlank() && it.password.isNotBlank()
            )
        }
    }

    fun updatePassword(password: String) {
        _state.update {
            it.copy(
                password = password,
                enabled = it.username.isNotBlank() && password.isNotBlank()
            )
        }
    }

    fun login() {
        if (state.value.enabled) {
            _state.update { it.copy(status = ScreenState.Status.Busy) }
            viewModelScope.launch {
                try {
                    domainState.domain.login(
                        username = state.value.username,
                        password = state.value.password
                    )
                    _state.update { it.copy(status = ScreenState.Status.None) }
                } catch (e: DomainException) {
                    _state.update {
                        it.copy(
                            status = Status.Error(
                                source = when (e) {
                                    is DomainException.Auth -> when (e.v1) {
                                        AuthError.INVALID_CREDENTIALS -> Errors.UnauthorisedError
                                    }
                                    else -> Errors.UnknownError
                                }
                            )
                        )
                    }
                } finally {
                    domainState.refresh()
                }
            }
        }
    }

    fun dismissDialog() {
        _state.update { it.copy(status = Status.None) }
    }

    enum class Errors : ErrorSource {
        UnauthorisedError,
        UnknownError
    }

    data class State(
        val username: String = "",
        val password: String = "",
        val enabled: Boolean = false,
        override val status: Status = Status.None
    ) : ScreenState
}
