package domain.common

/**
 * Repository that manages lists of items
 */
abstract class ListRepository<ItemKey, ItemType> {
    open suspend fun start() {}
    open suspend fun stop() {}

    abstract suspend fun getList(key: ItemKey): ItemList<ItemType>
    abstract suspend fun add(value: ItemType)
    abstract suspend fun put(value: ItemType)
    abstract suspend fun delete(value: ItemType)
}

data class ItemList<T>(
    val total: Int = 0,
    val lessons: List<T> = emptyList(),
    val hasMore: Boolean = false,
)