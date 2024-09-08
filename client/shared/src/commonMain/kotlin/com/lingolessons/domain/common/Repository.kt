package com.lingolessons.domain.common

import kotlinx.coroutines.flow.StateFlow

/**
 * Repository that manages a single item at a time
 */
interface Repository<Item> {
    fun get(): StateFlow<Item?>
    fun put(item: Item)
    fun delete()
}
