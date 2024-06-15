package data.api.auth

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
internal data class LoginResult(
    @SerialName("auth_token") val authToken: String,
)
