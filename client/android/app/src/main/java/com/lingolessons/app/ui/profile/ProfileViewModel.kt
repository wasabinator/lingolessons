package com.lingolessons.app.ui.profile

import androidx.lifecycle.viewModelScope
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.DomainStateViewModel
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class ProfileViewModel @Inject constructor(
    private val domainState: DomainState,
) : DomainStateViewModel(domainState = domainState) {
    private val _state = MutableStateFlow<ProfileState?>(null)
    val state = _state.asStateFlow()

    fun logout() {
        viewModelScope.launch {
            domainState.domain.logout()
            domainState.refresh()
        }
    }
}

data class ProfileState(
    val username: String,
)
