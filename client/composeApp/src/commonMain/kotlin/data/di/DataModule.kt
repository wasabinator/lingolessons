package data.di

import androidx.compose.ui.text.intl.Locale
import data.api.auth.TokenApi
import data.api.auth.createTokenApi
import data.repository.UserRepositoryImpl
import de.jensklingenberg.ktorfit.Ktorfit
import domain.auth.UserRepository
import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.DefaultRequest
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.plugins.logging.DEFAULT
import io.ktor.client.plugins.logging.LogLevel
import io.ktor.client.plugins.logging.Logger
import io.ktor.client.plugins.logging.Logging
import io.ktor.client.request.header
import io.ktor.http.HttpHeaders.AcceptLanguage
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json
import org.koin.dsl.module

val dataModule = module {
    single<UserRepository> { UserRepositoryImpl(get()) }

    single<Ktorfit> {
        Ktorfit.Builder().httpClient(
            client = HttpClient(CIO) {
                expectSuccess = true
                install(Logging) {
                    logger = Logger.DEFAULT
                    level = LogLevel.HEADERS
                }
                install(ContentNegotiation) {
                    json(Json { isLenient = true; ignoreUnknownKeys = true })
                }
                install(DefaultRequest) {
                    header(AcceptLanguage, Locale.current.language)
                }
            }).baseUrl("http://127.0.0.1:8000/api/v1/").build()
    }

    single<TokenApi> {
        get<Ktorfit>().createTokenApi()
    }
}
