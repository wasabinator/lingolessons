package data.api.common

import domain.common.RepositoryError
import io.ktor.client.call.body
import io.ktor.client.plugins.ClientRequestException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.IO
import kotlinx.coroutines.withContext

internal interface ApiCallProcessor {
    suspend fun <T, U> processCall(
        call: suspend () -> T,
        mapper: (T?) -> U,
    ) = try {
        mapper(
            withContext(Dispatchers.IO) {
                call()
            }
        )
    } catch (e: ClientRequestException) {
        throw RepositoryError(
            // Just return the first error message if there are multiple
            userFacingError = e.response.body<Map<String, List<String>>?>()
                ?.flatMap { it.value }?.firstOrNull(),
            cause = e,
        )
    }
}
