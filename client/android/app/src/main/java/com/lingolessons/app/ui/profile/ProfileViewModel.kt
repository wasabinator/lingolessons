package com.lingolessons.app.ui.profile

import androidx.lifecycle.viewModelScope
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.DomainStateViewModel
import com.lingolessons.app.ui.common.ScreenState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class ProfileViewModel(private val domainState: DomainState) : DomainStateViewModel(domainState = domainState) {
    private val _state = MutableStateFlow<ProfileState?>(null)
    val state = _state.asStateFlow()

    fun logout() {
        viewModelScope.launch {
            domainState.domain.logout()
            domainState.refresh()
        }
    }

    override fun updateStatus(status: ScreenState.Status) {}
}

data class ProfileState(val username: String)
