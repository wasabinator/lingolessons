package com.lingolessons.ui.profile

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.lingolessons.domain.auth.LogoutUser
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class ProfileViewModel(
    private val logout: LogoutUser
) : ViewModel() {
    private val _state = MutableStateFlow<ProfileState?>(null)
    val state = _state.asStateFlow()

    fun logout() {
        viewModelScope.launch {
            logout.perform(Unit)
        }
    }
}

data class ProfileState(
    val username: String,
)
