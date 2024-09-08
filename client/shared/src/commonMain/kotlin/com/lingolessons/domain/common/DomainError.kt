package com.lingolessons.domain.common

data class DomainError(
    val userFacingError: String?,
    override val cause: Throwable
) : Throwable(
    cause = cause
)
