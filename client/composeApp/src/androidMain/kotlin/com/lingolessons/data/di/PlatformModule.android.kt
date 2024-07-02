package com.lingolessons.data.di

import app.cash.sqldelight.driver.android.AndroidSqliteDriver
import com.lingolessons.data.db.AppDatabase
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

actual val platformModule = module {
    single { AndroidSqliteDriver(AppDatabase.Schema, androidContext(), "database.db") }
}
