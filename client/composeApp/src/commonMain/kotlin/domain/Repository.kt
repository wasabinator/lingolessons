package domain

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

abstract class Repository<in T, U> {
    protected val _state = MutableStateFlow<U?>(null)
    val state = _state.asStateFlow()

    open suspend fun start() {}
    open suspend fun stop() {}
    abstract suspend fun update(value: T?)
}
