package data.di

import DriverFactory
import androidx.compose.ui.text.intl.Locale
import com.lingolessons.data.db.AppDatabase
import com.lingolessons.data.db.SessionQueries
import createDatabase
import data.api.auth.TokenApi
import data.api.auth.createTokenApi
import data.api.lessons.LessonsApi
import data.api.lessons.createLessonsApi
import data.manager.auth.SessionManagerImpl
import data.repository.lessons.LessonRepositoryImpl
import de.jensklingenberg.ktorfit.Ktorfit
import domain.auth.SessionManager
import domain.lessons.LessonRepository
import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.DefaultRequest
import io.ktor.client.plugins.auth.Auth
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.plugins.logging.DEFAULT
import io.ktor.client.plugins.logging.LogLevel
import io.ktor.client.plugins.logging.Logger
import io.ktor.client.plugins.logging.Logging
import io.ktor.client.request.header
import io.ktor.http.HttpHeaders.AcceptLanguage
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json
import org.koin.core.qualifier.named
import org.koin.dsl.module

val dataModule = module {
    single<HttpClient> {
        HttpClient(CIO) {
            expectSuccess = true
            install(Logging) {
                logger = Logger.DEFAULT
                level = LogLevel.ALL
            }
            install(ContentNegotiation) {
                json(Json { isLenient = true; ignoreUnknownKeys = true })
            }
            install(DefaultRequest) {
                header(AcceptLanguage, Locale.current.language)
            }
            install(Auth) {
                // Details will be filled in by the SessionManager
            }
        }
    }

    single<Ktorfit> {
        Ktorfit.Builder().httpClient(
            client = get()
        ).baseUrl("http://127.0.0.1:8000/api/v1/").build()
    }

    single<TokenApi> {
        get<Ktorfit>().createTokenApi()
    }

    single<LessonsApi> {
        get<Ktorfit>().createLessonsApi()
    }

    single {
        get<DriverFactory>().createDriver()
    }

    single<AppDatabase> { createDatabase(get()) }

    single { SessionQueries(get()) }

    single<SessionManager> { SessionManagerImpl(get(), get(), get(), get(named("domain"))) }

    single<LessonRepository> { LessonRepositoryImpl(get()) }
}
