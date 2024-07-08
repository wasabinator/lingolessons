package com.lingolessons.data.di

import androidx.compose.ui.text.intl.Locale
import com.lingolessons.data.auth.SessionManagerImpl
import com.lingolessons.data.auth.TokenApi
import com.lingolessons.data.auth.TokenRepository
import com.lingolessons.data.auth.TokenRepositoryImpl
import com.lingolessons.data.auth.createTokenApi
import com.lingolessons.data.db.AppDatabase
import com.lingolessons.data.db.DbUtils
import com.lingolessons.data.db.TokenQueries
import com.lingolessons.data.db.dao
import com.lingolessons.data.lessons.LessonRepositoryImpl
import com.lingolessons.data.lessons.LessonApi
import com.lingolessons.data.lessons.createLessonApi
import com.lingolessons.domain.auth.SessionManager
import com.lingolessons.domain.lessons.LessonRepository
import de.jensklingenberg.ktorfit.Ktorfit
import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.DefaultRequest
import io.ktor.client.plugins.auth.Auth
import io.ktor.client.plugins.auth.providers.BearerTokens
import io.ktor.client.plugins.auth.providers.bearer
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
    single<TokenRepository> {
        TokenRepositoryImpl(
            TokenQueries(get()).dao,
            get(named("domain"))
        )
    }

    single<HttpClient> {
        val repository = get<TokenRepository>()
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
                bearer {
                    loadTokens {
                        repository.get().value?.let {
                            println("*** TOKENS: $it ***")
                            BearerTokens(
                                accessToken = it.authToken,
                                refreshToken = it.refreshToken
                            )
                        }
                    }
                    refreshTokens {
                        // TODO
                        repository.get().value?.let {
                            BearerTokens(
                                accessToken = it.authToken,
                                refreshToken = it.refreshToken
                            )
                        }
                    }
//                    sendWithoutRequest { request ->
//                        request.url.encodedPath.contains("login")
//                    }
                }
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

    single<LessonApi> {
        get<Ktorfit>().createLessonApi()
    }

    single<AppDatabase> { DbUtils.createDatabase(get()) }

    single { TokenQueries(get()) }

    single<SessionManager> {
        SessionManagerImpl(
            get(),
            get(),
            get(named("domain"))
        )
    }

    single<LessonRepository> { LessonRepositoryImpl(get()) }
}
