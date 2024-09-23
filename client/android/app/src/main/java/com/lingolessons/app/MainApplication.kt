package com.lingolessons.app

import android.app.Application
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class MainApplication : Application() {
    override fun onCreate() {
        super.onCreate()

//        startKoin {
//            androidContext(this@MainApplication)
//            androidLogger(Level.ERROR)
//            modules(uiModule, platformModule, domainModule, dataModule)
//        }
    }
}