package com.lingolessons.common

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestCoroutineScheduler
import kotlinx.coroutines.test.TestDispatcher
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import kotlin.test.AfterTest
import kotlin.test.BeforeTest

abstract class BaseTest {
    protected lateinit var scheduler: TestCoroutineScheduler
    protected lateinit var dispatcher: TestDispatcher

    @OptIn(ExperimentalCoroutinesApi::class)
    @BeforeTest
    fun before() {
        scheduler = TestCoroutineScheduler()
        dispatcher = StandardTestDispatcher(scheduler)
        Dispatchers.setMain(dispatcher)
        setup()
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    @AfterTest
    fun after() {
        Dispatchers.resetMain()
    }

    abstract fun setup()

    fun advanceUntilIdle() = scheduler.advanceUntilIdle()

    fun runTest(block: suspend () -> Unit) = runTest(context = dispatcher) { block() }
}
