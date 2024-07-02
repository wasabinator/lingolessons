package com.lingolessons.data.di

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.lingolessons.data.db.AppDatabase
import org.koin.dsl.module

actual val platformModule = module {
    single {
        AppDatabase.Schema.create(
            JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        )
    }
}
