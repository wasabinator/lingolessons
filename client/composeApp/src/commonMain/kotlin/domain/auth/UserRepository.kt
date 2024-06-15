package domain.auth

import domain.Repository

abstract class UserRepository : Repository<LoginDetails, UserSession>()

data class LoginDetails(
    val username: String,
    val password: String,
)

data class UserSession(
    val username: String,
    val authToken: String,
)
