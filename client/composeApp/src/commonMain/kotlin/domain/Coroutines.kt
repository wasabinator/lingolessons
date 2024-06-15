package domain

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob

val domainScope: CoroutineScope by lazy {
    CoroutineScope(
        context = Dispatchers.Default.limitedParallelism(
            parallelism = 1,
            name = "Domain Context"
        ) + SupervisorJob()
    )
}
