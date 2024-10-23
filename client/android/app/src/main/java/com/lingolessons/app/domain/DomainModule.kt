package com.lingolessons.app.domain

import com.lingolessons.shared.DomainBuilder
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

val domainModule = module {
    single {
        val path = androidContext().getDatabasePath("app.db")
        DomainState(
            domain = DomainBuilder()
                .baseUrl("http://10.0.2.2:8000/api/v1/")
                .dataPath(path.absolutePath)
                .build()
        )
    }
}
