package com.lingolessons.app.domain

import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Session
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

class DomainState(val domain: DomainInterface) {
    private val _state = MutableStateFlow<Session>(Session.None)
    val state = _state.asStateFlow()

    private val scope by lazy { CoroutineScope(Dispatchers.Main) }

    init {
        refresh()
    }

    fun refresh() {
        scope.launch { _state.update { domain.getSession() } }
    }
}
