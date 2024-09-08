package com.lingolessons.domain.common

import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers

val domainDispatcher: CoroutineDispatcher by lazy {
    Dispatchers.Default.limitedParallelism(
        parallelism = 1,
        name = "Domain Dispatcher"
    )
}
