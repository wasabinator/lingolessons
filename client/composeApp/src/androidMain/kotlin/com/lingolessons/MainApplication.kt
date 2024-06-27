package com.lingolessons

import android.app.Application
import com.lingolessons.data.di.dataModule
import com.lingolessons.di.appModule
import com.lingolessons.domain.di.domainModule
import org.koin.android.ext.koin.androidContext
import org.koin.android.ext.koin.androidLogger
import org.koin.core.context.GlobalContext.startKoin
import org.koin.core.logger.Level
import platformModule

class MainApplication : Application() {
    override fun onCreate() {
        super.onCreate()

        startKoin {
            androidContext(this@MainApplication)
            androidLogger(Level.ERROR)
            modules(appModule, platformModule, domainModule, dataModule)
        }
    }
}
