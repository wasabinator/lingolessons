package com.lingolessons.app.common

import junit.framework.TestCase.assertEquals
import junit.framework.TestCase.assertTrue

interface MockMethod {
    fun call(vararg args: Any)

    fun expect(vararg args: Any)

    fun verify(times: Int? = null)
}

fun mockMethod() = object : MockMethod {
    private val calls = mutableListOf<List<Any>>()

    override fun call(vararg args: Any) {
        calls.add(args.toList())
    }

    override fun expect(vararg args: Any) {
        assertTrue(calls.isNotEmpty())
        assertEquals(args.toList(), calls.removeFirst())
    }

    override fun verify(times: Int?) = if (times == null) {
        assertTrue(calls.isNotEmpty())
    } else {
        assertEquals(times, calls.size)
    }
}
