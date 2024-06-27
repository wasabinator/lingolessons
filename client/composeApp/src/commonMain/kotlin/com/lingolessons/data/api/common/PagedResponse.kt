package com.lingolessons.data.api.common

interface PagedResponse<T> {
    val count: Int
    val next: String?
    val results: List<T>
}
