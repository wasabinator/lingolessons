package com.lingolessons.app.ui.profile

import androidx.lifecycle.viewModelScope
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.ui.common.DomainStateViewModel
import com.lingolessons.app.ui.profile.ProfileViewModel.ScreenData
import kotlinx.coroutines.launch

class ProfileViewModel(domainState: DomainState) :
    DomainStateViewModel<ScreenData>(domainState = domainState, initData = ScreenData()) {

    fun logout() {
        viewModelScope.launch {
            domainState.domain.logout()
            domainState.refresh()
        }
    }

    data class ScreenData(
        val username: String = "",
    )
}
