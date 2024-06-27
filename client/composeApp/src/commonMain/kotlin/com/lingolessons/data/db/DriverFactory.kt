package com.lingolessons.data.db

import app.cash.sqldelight.db.SqlDriver

expect class DriverFactory {
    fun createDriver(): SqlDriver
}

fun createDatabase(driver: SqlDriver): AppDatabase {
    val database = AppDatabase(driver)
    // TODO: Migrations etc
    return database
}
