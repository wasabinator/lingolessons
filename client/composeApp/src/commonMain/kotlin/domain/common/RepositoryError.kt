package domain.common

data class RepositoryError(
    val userFacingError: String?,
    override val cause: Throwable
) : Throwable(
    cause = cause
)
