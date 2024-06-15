package ui.login

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import domain.auth.LoginDetails
import domain.auth.LoginUser
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import org.koin.core.component.KoinComponent
import ui.common.ScreenState
import ui.common.getUiErrorMessage

class LoginViewModel(
    private val action: LoginUser
) : ViewModel(), KoinComponent {
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
        viewModelScope.launch {
            if (state.value.username.isBlank() || state.value.password.isBlank()) {
                return@launch
            } else {
                _state.update {
                    it.copy(
                        status = ScreenState.Status.Busy
                    )
                }
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
