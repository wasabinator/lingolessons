package domain.di

import domain.auth.LoginUser
import domain.common.domainDispatcher
import kotlinx.coroutines.CoroutineDispatcher
import org.koin.core.qualifier.named
import org.koin.dsl.module

val domainModule = module {
    single<CoroutineDispatcher>(named("domain")) { domainDispatcher }
    single {
        LoginUser(
            dispatcher = get(named("domain")),
            userRepository = get()
        )
    }
}
