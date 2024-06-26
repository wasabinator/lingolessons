import com.lingolessons.data.db.AppDatabase
import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import me.sujanpoudel.utils.paths.appDataDirectory
import java.io.File

actual class DriverFactory {
    actual fun createDriver(): SqlDriver {
        val databasePath = File(
            appDataDirectory("com.lingolessons").toString(), "database.db"
        )
        val driver: SqlDriver = JdbcSqliteDriver(url = "jdbc:sqlite:${databasePath.path}")
        AppDatabase.Schema.create(driver)
        return driver
    }
}
