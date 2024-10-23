package com.lingolessons.app.ui.main

import androidx.compose.runtime.collectAsState
import androidx.compose.ui.test.ComposeUiTest
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.assertIsSelected
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.runComposeUiTest
import com.lingolessons.app.common.BaseUiTest
import com.lingolessons.app.common.MockMethod
import com.lingolessons.app.common.mockMethod
import com.lingolessons.app.ui.AppScreen
import com.lingolessons.app.ui.MainScreen
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.update
import org.junit.Test

class MainScreenTest : BaseUiTest() {
    private lateinit var largeScreen: MutableStateFlow<Boolean>
    private lateinit var currentRoute: MutableStateFlow<AppScreen?>
    private lateinit var mockClickMethod: MockMethod

    override fun setup() {
        largeScreen = MutableStateFlow(false)
        currentRoute = MutableStateFlow(null)
        mockClickMethod = mockMethod()
    }

    @OptIn(ExperimentalTestApi::class)
    private fun ComposeUiTest.setContent() {
        setContent {
            MainScreen(
                largeScreen = largeScreen.collectAsState().value,
                mainNav = {},
                currentRoute = currentRoute.collectAsState().value,
                onNavItemClick = { mockClickMethod.call(it) },
            )
        }
    }

    @OptIn(ExperimentalTestApi::class)
    @Test
    fun `should perform correct navigation on small screens`() = runComposeUiTest {
        setContent()
        onNodeWithTag("smallScreen").assertExists()
        onNodeWithTag("largeScreen").assertDoesNotExist()

        verifyNavigation()
    }

    @OptIn(ExperimentalTestApi::class)
    @Test
    fun `should perform correct navigation on large screens`() = runComposeUiTest {
        largeScreen.update { true }
        setContent()
        onNodeWithTag("largeScreen").assertExists()
        onNodeWithTag("smallScreen").assertDoesNotExist()

        verifyNavigation()
    }

    @OptIn(ExperimentalTestApi::class)
    private fun ComposeUiTest.verifyNavigation() {
        onNodeWithTag("profile").assertIsSelected()
        onNodeWithTag("study").performClick()
        mockClickMethod.expect(AppScreen.Study)
        onNodeWithTag("lessons").performClick()
        mockClickMethod.expect(AppScreen.Lessons)
    }

    @OptIn(ExperimentalTestApi::class)
    @Test
    fun `should select correct item selected for each route`() = runComposeUiTest {
        with(this) {
            setContent()
            verifyCurrentNavItem(true, AppScreen.Profile, "profile")
            verifyCurrentNavItem(false, AppScreen.Profile, "profile")

            verifyCurrentNavItem(true, AppScreen.Study, "study")
            verifyCurrentNavItem(false, AppScreen.Study, "study")

            verifyCurrentNavItem(true, AppScreen.Lessons, "lessons")
            verifyCurrentNavItem(false, AppScreen.Lessons, "lessons")
        }
    }

    context(ComposeUiTest)
    @OptIn(ExperimentalTestApi::class)
    private fun verifyCurrentNavItem(
        largeScreen: Boolean,
        screen: AppScreen,
        tag: String,
    ) {
        this.largeScreen.update { largeScreen }
        this.currentRoute.update { screen }

        onNodeWithTag(tag).assertIsSelected()
    }
}
