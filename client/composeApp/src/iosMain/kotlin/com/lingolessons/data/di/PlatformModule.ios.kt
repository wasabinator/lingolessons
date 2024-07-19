package com.lingolessons.data.di

import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.native.NativeSqliteDriver
import com.lingolessons.data.db.AppDatabase
import org.koin.dsl.module

actual val platformModule = module {
    single<SqlDriver> {
        NativeSqliteDriver(AppDatabase.Schema, "database.db")
    }
}
