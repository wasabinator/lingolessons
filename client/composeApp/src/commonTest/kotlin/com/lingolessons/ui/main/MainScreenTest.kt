package com.lingolessons.ui.main

import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.assertIsSelected
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.runComposeUiTest
import com.lingolessons.common.BaseUiTest
import com.lingolessons.common.MockMethod
import com.lingolessons.common.mockMethod
import kotlin.test.Test

@OptIn(ExperimentalTestApi::class)
class MainScreenTest : BaseUiTest() {
    private lateinit var mockClickMethod: MockMethod

    override fun setup() {
        mockClickMethod = mockMethod()
    }

    @OptIn(ExperimentalTestApi::class)
    private fun ComposeUiTest.setContent(
        largeScreen: Boolean,
        currentRoute: AppScreen?,
        mockClickMethod: MockMethod,
    ) {
        setContent {
            MainScreen(
                largeScreen = largeScreen,
                mainNav = {},
                currentRoute = currentRoute,
                onNavItemClick = { mockClickMethod.call(it) },
            )
        }
    }

    @Test
    fun `should perform correct navigation on small screens`() = runComposeUiTest {
        setContent(
            largeScreen = false,
            currentRoute = null,
            mockClickMethod = mockClickMethod
        )

        onNodeWithTag("smallScreen").assertExists()
        onNodeWithTag("largeScreen").assertDoesNotExist()

        verifyNavigation()
    }

    @Test
    fun `should perform correct navigation on large screens`() = runComposeUiTest {
        setContent(
            largeScreen = true,
            currentRoute = null,
            mockClickMethod = mockClickMethod
        )

        onNodeWithTag("largeScreen").assertExists()
        onNodeWithTag("smallScreen").assertDoesNotExist()

        verifyNavigation()
    }

    private fun ComposeUiTest.verifyNavigation() {
        onNodeWithTag("profile").assertIsSelected()
        onNodeWithTag("study").performClick()
        mockClickMethod.expect(AppScreen.Study)
        onNodeWithTag("lessons").performClick()
        mockClickMethod.expect(AppScreen.Lessons)
    }

    @Test
    fun `should select correct item selected for each route`() = runComposeUiTest {
        verifyCurrentNavItem(true, "profile", AppScreen.Profile)
        verifyCurrentNavItem(false, "profile", AppScreen.Profile)

        verifyCurrentNavItem(true, "study", AppScreen.Study)
        verifyCurrentNavItem(false, "study", AppScreen.Study)

        verifyCurrentNavItem(true, "lessons", AppScreen.Lessons)
        verifyCurrentNavItem(false, "lessons", AppScreen.Lessons)
    }

    private fun ComposeUiTest.verifyCurrentNavItem(
        largeScreen: Boolean,
        tag: String,
        screen: AppScreen
    ) {
        setContent(
            largeScreen = largeScreen,
            currentRoute = screen,
            mockClickMethod = mockClickMethod
        )
        onNodeWithTag(tag).assertIsSelected()
    }
}

