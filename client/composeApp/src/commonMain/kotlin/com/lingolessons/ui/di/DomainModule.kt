package com.lingolessons.ui.di

import com.lingolessons.domain.auth.GetUserSession
import com.lingolessons.domain.auth.GetUserSessionImpl
import com.lingolessons.domain.auth.LoginUser
import com.lingolessons.domain.auth.LoginUserImpl
import com.lingolessons.domain.auth.LogoutUser
import com.lingolessons.domain.auth.LogoutUserImpl
import com.lingolessons.domain.common.domainDispatcher
import com.lingolessons.domain.lessons.GetLessons
import com.lingolessons.domain.lessons.GetLessonsImpl
import kotlinx.coroutines.CoroutineDispatcher
import org.koin.core.qualifier.named
import org.koin.dsl.module

val domainModule = module {
    single<CoroutineDispatcher>(named("domain")) { domainDispatcher }
    single<LoginUser> {
        LoginUserImpl(
            dispatcher = get(named("domain")),
            sessionManager = get(),
        )
    }
    single<LogoutUser> {
        LogoutUserImpl(
            dispatcher = get(named("domain")),
            sessionManager = get(),
        )
    }
    single<GetUserSession> {
        GetUserSessionImpl(
            sessionManager = get(),
        )
    }
    single<GetLessons> {
        GetLessonsImpl(
            dispatcher = get(named("domain")),
            lessonRepository = get(),
        )
    }
}
