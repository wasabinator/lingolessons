import com.lingolessons.data.db.AppDatabase
import app.cash.sqldelight.db.SqlDriver

expect class DriverFactory {
    fun createDriver(): SqlDriver
}

fun createDatabase(driver: SqlDriver): AppDatabase {
    val database = AppDatabase(driver)
    // TODO: Migrations etc
    return database
}
