package com.lingolessons.app.common

import junit.framework.TestCase.assertEquals
import junit.framework.TestCase.assertTrue

interface MockMethod {
    fun call(vararg args: Any)
    fun expect(vararg args: Any)
    fun verify(times: Int? = null)
}

fun mockMethod() = object : MockMethod {
    private val calls = ArrayDeque<List<Any>>()
    override fun call(vararg args: Any) {
        calls.addLast(args.toList())
    }

    override fun expect(vararg args: Any) {
        assertEquals(args.toList(), calls.removeFirst())
    }

    override fun verify(times: Int?) =
        if (times == null) assertTrue(calls.size > 0)
        else assertEquals(times, calls.size)
}
