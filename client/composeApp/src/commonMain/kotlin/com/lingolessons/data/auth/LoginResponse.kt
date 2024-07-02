package com.lingolessons.data.auth

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
internal data class LoginResponse(
    @SerialName("auth_token") val authToken: String,
)