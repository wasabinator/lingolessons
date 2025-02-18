package com.lingolessons.app.ui

import android.annotation.SuppressLint
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationBarItemDefaults
import androidx.compose.material3.NavigationDrawerItem
import androidx.compose.material3.NavigationDrawerItemDefaults
import androidx.compose.material3.PermanentDrawerSheet
import androidx.compose.material3.PermanentNavigationDrawer
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.res.vectorResource
import androidx.compose.ui.unit.dp
import androidx.navigation.NavBackStackEntry
import androidx.navigation.NavHostController
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.lingolessons.app.R

@Composable
fun MainScreen() {
    val navController: NavHostController = rememberNavController()
    val backStackEntry: NavBackStackEntry? by navController.currentBackStackEntryAsState()
    val currentRoute = AppScreen.getRoute(backStackEntry)

    MainScreen(
        largeScreen = false, // TODO
        mainNav = { MainNav(navController = navController) },
        currentRoute = currentRoute,
        onNavItemClick = { screen: AppScreen ->
            navController.navigate(screen) {
                popUpTo(0)
            }
        }
    )
}

@SuppressLint("UnusedMaterial3ScaffoldPaddingParameter")
@Composable
fun MainScreen(
    largeScreen: Boolean,
    mainNav: @Composable () -> Unit,
    currentRoute: AppScreen?,
    onNavItemClick: (AppScreen) -> Unit,
) {
    if (largeScreen) {
        Scaffold(modifier = Modifier.testTag("largeScreen")) { _ ->
            PermanentNavigationDrawer(
                drawerContent = {
                    DrawerNavigation(
                        currentScreen = currentRoute ?: AppScreen.Profile,
                        onClick = onNavItemClick
                    )
                }
            ) {
                mainNav()
            }
        }
    } else {
        Scaffold(
            modifier = Modifier.testTag("smallScreen"),
            bottomBar = {
                BottomNavigationBar(
                    currentScreen = currentRoute ?: AppScreen.Profile,
                    onClick = onNavItemClick
                )
            }
        ) { _ -> // Inner Scaffolds will provide this padding
            Box(modifier = Modifier
                .fillMaxSize()
            ) {
                mainNav()
            }
        }
    }
}

@Composable
fun DrawerNavigation(
    currentScreen: AppScreen,
    onClick: (AppScreen) -> Unit
) {
    PermanentDrawerSheet(modifier = Modifier.width(240.dp)) {
        Spacer(Modifier.height(24.dp))
        NavigationDrawerItem(
            icon = {
                Icon(
                    imageVector = ImageVector.vectorResource(R.drawable.account_circle_48px),
                    contentDescription = "",
                    tint = drawerIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text(stringResource(R.string.feature_profile)) },
            selected = currentScreen == AppScreen.Profile,
            onClick = { onClick(AppScreen.Profile) },
            modifier = Modifier
                .padding(horizontal = 12.dp)
                .testTag("profile")
        )
        NavigationDrawerItem(
            icon = {
                Icon(
                    imageVector = ImageVector.vectorResource(R.drawable.school_48px),
                    contentDescription = "",
                    tint = drawerIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text(stringResource(R.string.feature_study)) },
            selected = currentScreen == AppScreen.Study,
            onClick = { onClick(AppScreen.Study) },
            modifier = Modifier
                .padding(horizontal = 12.dp)
                .testTag("study")
        )
        NavigationDrawerItem(
            icon = {
                Icon(
                    imageVector = ImageVector.vectorResource(R.drawable.library_books_48px),
                    contentDescription = "",
                    tint = drawerIconColor(currentScreen == AppScreen.Study)
                )
            }, label = { Text(stringResource(R.string.feature_lessons)) },
            selected = currentScreen == AppScreen.Lessons,
            onClick = { onClick(AppScreen.Lessons) },
            modifier = Modifier
                .padding(horizontal = 12.dp)
                .testTag("lessons")
        )
    }
}

@Composable
fun drawerIconColor(isSelected: Boolean) =
    NavigationDrawerItemDefaults.colors().iconColor(isSelected).value


@Composable
fun BottomNavigationBar(
    currentScreen: AppScreen,
    onClick: (AppScreen) -> Unit
) {
    NavigationBar {
        NavigationBarItem(
            selected = currentScreen == AppScreen.Profile,
            onClick = { onClick(AppScreen.Profile) },
            icon = {
                Icon(
                    imageVector = ImageVector.vectorResource(R.drawable.account_circle_48px),
                    modifier = Modifier.size(32.dp),
                    contentDescription = "",
                    tint = navBarIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text(stringResource(R.string.feature_profile)) },
            modifier = Modifier.testTag("profile"),
        )
        NavigationBarItem(
            selected = currentScreen == AppScreen.Study,
            onClick = { onClick(AppScreen.Study) },
            icon = {
                Icon(
                    imageVector = ImageVector.vectorResource(R.drawable.school_48px),
                    modifier = Modifier.size(32.dp),
                    contentDescription = "",
                    tint = navBarIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text(stringResource(R.string.feature_study)) },
            modifier = Modifier.testTag("study"),
        )
        NavigationBarItem(
            selected = currentScreen == AppScreen.Lessons,
            onClick = { onClick(AppScreen.Lessons) },
            icon = {
                Icon(
                    imageVector = ImageVector.vectorResource(R.drawable.library_books_48px),
                    modifier = Modifier.size(32.dp),
                    contentDescription = "",
                    tint = navBarIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text(stringResource(R.string.feature_lessons)) },
            modifier = Modifier.testTag("lessons"),
        )
    }
}

@Composable
fun navBarIconColor(isSelected: Boolean) = if (isSelected) {
    NavigationBarItemDefaults.colors().selectedIconColor
} else {
    NavigationBarItemDefaults.colors().unselectedIconColor
}
