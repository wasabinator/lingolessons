package com.lingolessons.app.di

import android.content.Context
import com.lingolessons.app.domain.DomainState
import com.lingolessons.shared.DomainBuilder
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object AppModule {
    @Provides
    @Singleton
    fun domainState(@ApplicationContext context: Context): DomainState {
        val path = context.getDatabasePath("app.db")
        return DomainState(
            domain = DomainBuilder()
            .baseUrl("http://10.0.2.2:8000/api/v1/")
            .dataPath(path.absolutePath)
            .build()
        )
    }
}
