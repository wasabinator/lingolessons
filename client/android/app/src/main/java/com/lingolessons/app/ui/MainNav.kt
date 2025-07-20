package com.lingolessons.app.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavBackStackEntry
import androidx.navigation.NavDestination.Companion.hasRoute
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.navigation
import androidx.navigation.toRoute
import com.lingolessons.app.ui.common.AuthenticatedScreen
import com.lingolessons.app.ui.lessons.LessonScreen
import com.lingolessons.app.ui.lessons.LessonViewModel
import com.lingolessons.app.ui.lessons.LessonsScreen
import com.lingolessons.app.ui.lessons.LessonsViewModel
import com.lingolessons.app.ui.profile.ProfileScreen
import com.lingolessons.app.ui.profile.ProfileViewModel
import kotlinx.serialization.Serializable
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.parameter.parametersOf

@Serializable
sealed class AppScreen {
    @Serializable
    data object Profile: AppScreen()
    @Serializable
    data object Study: AppScreen()
    @Serializable
    data object Lessons: AppScreen()
    @Serializable
    data object LessonList: AppScreen()
    @Serializable
    data class LessonDetail(val id: String): AppScreen()

    companion object {
        fun getRoute(entry: NavBackStackEntry?): AppScreen = entry?.let {
            when {
                entry.destination.hasRoute<Profile>() -> entry.toRoute<Profile>()
                entry.destination.hasRoute<Study>() -> entry.toRoute<Study>()
                entry.destination.hasRoute<Lessons>() -> entry.toRoute<Lessons>()
                entry.destination.hasRoute<LessonList>() -> entry.toRoute<LessonList>()
                entry.destination.hasRoute<LessonDetail>() -> entry.toRoute<LessonDetail>()
                else -> throw IllegalArgumentException("Invalid route. Did you forget to implement getRouter()?")
            }
        } ?: Profile
    }
}

@Composable
fun MainNav(
    navController: NavHostController
) {
    NavHost(
        navController = navController,
        startDestination = AppScreen.Profile,
        modifier = Modifier.fillMaxSize().background(MaterialTheme.colorScheme.background),
    ) {
        composable<AppScreen.Profile> {
            val viewModel: ProfileViewModel = koinViewModel()
            AuthenticatedScreen(viewModel = viewModel) {
                ProfileScreen(
                    viewModel = viewModel,
                )
            }
        }
        composable<AppScreen.Study> {
            // TODO: Add study screen
        }
        navigation<AppScreen.Lessons>(
            startDestination = AppScreen.LessonList,
        ) {
            composable<AppScreen.LessonList> {
                val viewModel: LessonsViewModel = koinViewModel()
                AuthenticatedScreen(viewModel = viewModel) {
                    LessonsScreen(
                        viewModel = viewModel
                    ) {
                        lesson -> navController.navigate(
                            AppScreen.LessonDetail(lesson.id)
                        )
                    }
                }
            }
            composable<AppScreen.LessonDetail> { backStackEntry ->
                val lessonId: String = backStackEntry.toRoute<AppScreen.LessonDetail>().id
                val viewModel: LessonViewModel = koinViewModel() {
                    parametersOf(lessonId)
                }
                LessonScreen(
                    viewModel = viewModel,
                ) {
                    navController.navigateUp()
                }
            }
        }
    }
}
