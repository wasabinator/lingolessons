package com.lingolessons.app.ui.login

import androidx.lifecycle.viewModelScope
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.DomainStateViewModel
import com.lingolessons.app.ui.common.ErrorSource
import com.lingolessons.app.ui.common.ScreenState.Status
import com.lingolessons.app.ui.login.LoginViewModel.ScreenData
import com.lingolessons.shared.AuthError
import com.lingolessons.shared.DomainException
import kotlinx.coroutines.launch

class LoginViewModel(domainState: DomainState) :
    DomainStateViewModel<ScreenData>(
        domainState = domainState,
        initData = ScreenData(),
    ) {

    fun updateUsername(username: String) {
        updateData {
            it.copy(
                username = username,
                enabled = it.username.isNotBlank() && it.password.isNotBlank(),
            )
        }
    }

    fun updatePassword(password: String) {
        updateData {
            it.copy(
                password = password,
                enabled = it.username.isNotBlank() && password.isNotBlank(),
            )
        }
    }

    fun login() {
        if (data.enabled) {
            updateStatus(Status.Busy)
            viewModelScope.launch {
                try {
                    domainState.domain.login(
                        username = data.username,
                        password = data.password,
                    )
                    clearStatus()
                } catch (e: DomainException) {
                    updateStatus(
                        Status.Error(
                            source =
                                when (e) {
                                    is DomainException.Auth ->
                                        when (e.v1) {
                                            AuthError.INVALID_CREDENTIALS ->
                                                Errors.UnauthorisedError
                                        }
                                    else -> Errors.UnknownError
                                },
                        ),
                    )
                } finally {
                    domainState.refresh()
                }
            }
        }
    }

    enum class Errors : ErrorSource {
        UnauthorisedError,
        UnknownError
    }

    data class ScreenData(
        val username: String = "",
        val password: String = "",
        val enabled: Boolean = false,
    )
}
