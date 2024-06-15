package data.repository

import data.api.auth.TokenApi
import data.api.common.ApiCallProcessor
import domain.auth.LoginDetails
import domain.auth.UserRepository
import domain.auth.UserSession
import kotlinx.coroutines.flow.update

internal class UserRepositoryImpl(
    private val tokenApi: TokenApi,
) : UserRepository(), ApiCallProcessor {
    override suspend fun update(value: LoginDetails?) {
        if (value != null) {
            processCall(
                { tokenApi.login(value.username, value.password) }
            ) { result ->
                _state.update {
                    result?.let {
                        UserSession(
                            username = value.username,
                            authToken = result.authToken,
                        )
                    }
                }
            }
        } else {
            processCall(
                { tokenApi.logout() }
            ) {
                _state.update { null }
            }
        }
    }
}
