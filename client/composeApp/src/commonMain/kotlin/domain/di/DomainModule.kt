package domain.di

import domain.auth.LoginUser
import domain.domainScope
import kotlinx.coroutines.CoroutineScope
import org.koin.core.qualifier.named
import org.koin.dsl.module

val domainModule = module {
    single<CoroutineScope>(named("domain")) { domainScope }
    single {
        LoginUser(
            domainScope = get(named("domain")),
            userRepository = get()
        )
    }
}
