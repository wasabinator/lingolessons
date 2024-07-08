package com.lingolessons.ui.login

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lingolessons.domain.auth.LoginDetails
import com.lingolessons.domain.auth.LoginUser
import com.lingolessons.ui.common.ScreenState
import com.lingolessons.ui.common.getUiErrorMessage
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class LoginViewModel(
    private val action: LoginUser
) : ViewModel() {
    private val _state = MutableStateFlow(LoginState())
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
            _state.update {
                it.copy(
                    status = ScreenState.Status.Busy
                )
            }
            viewModelScope.launch {
                val response = action.perform(
                    LoginDetails(
                        username = state.value.username,
                        password = state.value.password,
                    )
                )
                if (response.isSuccess) {
                    _state.update {
                        it.copy(
                            status = ScreenState.Status.None
                        )
                    }
                } else {
                    _state.update {
                        it.copy(
                            status = ScreenState.Status.Error(
                                message = response.getUiErrorMessage(),
                            ),
                        )
                    }
                }
            }
        }
    }

    fun dismissDialog() {
        _state.update {
            it.copy(
                status = ScreenState.Status.None,
            )
        }
    }

    data class LoginState(
        val username: String = "",
        val password: String = "",
        val enabled: Boolean = false,
        override val status: ScreenState.Status = ScreenState.Status.None,
    ) : ScreenState
}
