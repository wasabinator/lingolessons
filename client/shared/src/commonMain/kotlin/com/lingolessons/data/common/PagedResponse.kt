package com.lingolessons.data.common

interface PagedResponse<T> {
    val count: Int
    val next: String?
    val results: List<T>
}
