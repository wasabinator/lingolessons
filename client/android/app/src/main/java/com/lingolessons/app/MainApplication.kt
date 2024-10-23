package com.lingolessons.app

import android.app.Application
import com.lingolessons.app.di.uiModule
import com.lingolessons.app.domain.domainModule
import org.koin.android.ext.koin.androidContext
import org.koin.android.ext.koin.androidLogger
import org.koin.core.context.GlobalContext.startKoin
import org.koin.core.logger.Level

class MainApplication : Application() {
    override fun onCreate() {
        super.onCreate()

        startKoin {
            androidContext(this@MainApplication)
            androidLogger(Level.ERROR)
            modules(uiModule, domainModule)
        }
    }
}