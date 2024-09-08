package com.lingolessons.data.auth

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
internal data class LoginResponse(
    @SerialName("access") val authToken: String,
    @SerialName("refresh") val refreshToken: String,
)
