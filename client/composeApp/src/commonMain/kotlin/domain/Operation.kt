package domain

import kotlinx.coroutines.flow.Flow

interface Operation<T, U> {
    suspend fun perform(param: T): Result<U>
}

interface GetOperation<T, U> {
    fun perform(param: T): Flow<U?>
}
