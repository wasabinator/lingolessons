package com.lingolessons.app.ui.common

import androidx.lifecycle.ViewModel
import com.lingolessons.app.domain.DomainState

abstract class DomainStateViewModel(
    private val domainState: DomainState
) : ViewModel() {
    val domainStateFlow by lazy { domainState.state }

    abstract fun updateStatus(status: ScreenState.Status)
}
