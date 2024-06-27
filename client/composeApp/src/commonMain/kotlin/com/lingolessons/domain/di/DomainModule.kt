package com.lingolessons.domain.di

import com.lingolessons.domain.auth.GetUserSession
import com.lingolessons.domain.auth.LoginUser
import com.lingolessons.domain.auth.LogoutUser
import com.lingolessons.domain.common.domainDispatcher
import com.lingolessons.domain.lessons.GetLessons
import kotlinx.coroutines.CoroutineDispatcher
import org.koin.core.qualifier.named
import org.koin.dsl.module

val domainModule = module {
    single<CoroutineDispatcher>(named("com/lingolessons/domain")) { domainDispatcher }
    single {
        LoginUser(
            dispatcher = get(named("com/lingolessons/domain")),
            sessionManager = get(),
        )
    }
    single {
        LogoutUser(
            dispatcher = get(named("com/lingolessons/domain")),
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
            dispatcher = get(named("com/lingolessons/domain")),
            lessonRepository = get(),
        )
    }
}
