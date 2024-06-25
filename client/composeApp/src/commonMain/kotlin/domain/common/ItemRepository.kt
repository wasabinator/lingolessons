package domain.common

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

/**
 * Repository that manages a single item at a time
 */
abstract class ItemRepository<in ItemKey, Item>(
    initialState: Item
) {
    protected val _state = MutableStateFlow(initialState)
    val state = _state.asStateFlow()

    open suspend fun start() {}
    open suspend fun stop() {}

    open suspend fun put(value: ItemKey) {}
    abstract suspend fun delete()
}
