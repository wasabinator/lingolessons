package com.lingolessons.app.common

import com.lingolessons.shared.DateTime
import java.time.Instant
import java.time.LocalDateTime
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import java.time.format.FormatStyle

private val longDateFormatter by lazy { DateTimeFormatter.ofLocalizedDate(FormatStyle.LONG) }

fun DateTime.toLocalDateTime(): LocalDateTime =
    Instant.ofEpochMilli(toEpochMilli()).atZone(ZoneId.systemDefault()).toLocalDateTime()

fun DateTime.toLocalDateString(): String = longDateFormatter.format(toLocalDateTime())
