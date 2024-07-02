package com.lingolessons.data.di

import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.lingolessons.data.db.AppDatabase
import me.sujanpoudel.utils.paths.appDataDirectory
import org.koin.dsl.module
import java.io.File

actual val platformModule = module {
    single<SqlDriver> {
        val databasePath = File(
            appDataDirectory("com.lingolessons").toString(), "database.db"
        )
        JdbcSqliteDriver(url = "jdbc:sqlite:${databasePath.path}").also {
            // Create the schema
            AppDatabase.Schema.create(it)
        }
    }
}
