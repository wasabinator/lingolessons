package com.lingolessons.app.di

import com.lingolessons.app.domain.DomainState
import com.lingolessons.shared.DomainBuilder
import com.lingolessons.shared.DomainInterface
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

val domainModule = module {
    single { DomainState(domain = get()) }

    single<DomainInterface> {
        val path = androidContext().getDatabasePath("app.db")
        DomainBuilder().baseUrl("http://10.0.2.2:8000/api/v1/").dataPath(path.absolutePath).build()
    }
}
