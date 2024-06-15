package domain.auth

import common.suspendCatching
import domain.Operation
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.withContext

class LogoutUser(
    private val domainScope: CoroutineScope,
    private val userRepository: UserRepository,
) : Operation<Unit, Unit> {
    override suspend fun perform(param: Unit): Result<Unit> =
        withContext(domainScope.coroutineContext) {
            suspendCatching {
                userRepository.update(null)
            }
        }
}
