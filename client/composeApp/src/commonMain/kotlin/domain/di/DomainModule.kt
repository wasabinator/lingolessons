package domain.di

import domain.auth.GetUserSession
import domain.auth.LoginUser
import domain.auth.LogoutUser
import domain.common.domainDispatcher
import domain.lessons.GetLessons
import kotlinx.coroutines.CoroutineDispatcher
import org.koin.core.qualifier.named
import org.koin.dsl.module

val domainModule = module {
    single<CoroutineDispatcher>(named("domain")) { domainDispatcher }
    single {
        LoginUser(
            dispatcher = get(named("domain")),
            sessionManager = get(),
        )
    }
    single {
        LogoutUser(
            dispatcher = get(named("domain")),
            sessionManager = get(),
        )
    }
    single {
        GetUserSession(
            sessionManager = get(),
        )
    }
    single {
        GetLessons(
            dispatcher = get(named("domain")),
            lessonRepository = get(),
        )
    }
}
