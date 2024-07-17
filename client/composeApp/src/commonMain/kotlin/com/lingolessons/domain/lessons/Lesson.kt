package com.lingolessons.domain.lessons

data class Lesson(
    val id: String,
    val title: String,
    val owner: String,
    val updated: Int,
)
