package com.lingolessons.data.db

import app.cash.sqldelight.db.SqlDriver

object DbUtils {
    fun createDatabase(driver: SqlDriver): AppDatabase {
        val database = AppDatabase(driver)
        // TODO: Migrations etc
        return database
    }
}
