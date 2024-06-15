package domain

interface Operation<T, U> {
    suspend fun perform(param: T): Result<U>
}
