package com.lingolessons.ui.main

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.material.Scaffold
import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationBarItemDefaults
import androidx.compose.material3.NavigationDrawerItem
import androidx.compose.material3.NavigationDrawerItemDefaults
import androidx.compose.material3.PermanentDrawerSheet
import androidx.compose.material3.PermanentNavigationDrawer
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.NavHostController
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import getPlatform
import lingolessons.composeapp.generated.resources.Res
import lingolessons.composeapp.generated.resources.account_circle_48px
import lingolessons.composeapp.generated.resources.library_books_48px
import lingolessons.composeapp.generated.resources.school_48px
import org.jetbrains.compose.resources.vectorResource


@Composable
fun MainScreen(
    navController: NavHostController = rememberNavController()
) {
    val backStackEntry by navController.currentBackStackEntryAsState()
    val largeScreen = getPlatform().isLargeScreen()

    val currentRoute =
        derivedStateOf { backStackEntry?.destination?.route?.let { AppScreen.valueOf(it) } }

    if (largeScreen) {
        Scaffold {
            PermanentNavigationDrawer(
                drawerContent = {
                    DrawerNavigation(
                        currentScreen = currentRoute.value ?: AppScreen.Profile,
                        onClick = {
                            navController.navigate(it.name) {
                                launchSingleTop = true
                            }
                        }
                    )
                }
            ) {
                MainNav(navController = navController)
            }
        }
    } else {
        Scaffold(
            bottomBar = {
                BottomNavigationBar(
                    currentScreen = currentRoute.value
                        ?: AppScreen.Profile,
                    onClick = {
                        navController.navigate(it.name) {
                            launchSingleTop = true
                        }
                    }
                )
            }
        ) {
            MainNav(navController = navController)
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
                    imageVector = vectorResource(Res.drawable.account_circle_48px),
                    contentDescription = "",
                    tint = drawerIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text("Profile") },
            selected = currentScreen == AppScreen.Profile,
            onClick = { onClick(AppScreen.Profile) },
            modifier = Modifier.padding(horizontal = 12.dp)
        )
        NavigationDrawerItem(
            icon = {
                Icon(
                    imageVector = vectorResource(Res.drawable.school_48px),
                    contentDescription = "",
                    tint = drawerIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text("Study") },
            selected = currentScreen == AppScreen.Study,
            onClick = { onClick(AppScreen.Study) },
            modifier = Modifier.padding(horizontal = 12.dp)
        )
        NavigationDrawerItem(
            icon = {
                Icon(
                    imageVector = vectorResource(Res.drawable.library_books_48px),
                    contentDescription = "",
                    tint = drawerIconColor(currentScreen == AppScreen.Study)
                )
            }, label = { Text("Lessons") },
            selected = currentScreen == AppScreen.Lessons,
            onClick = { onClick(AppScreen.Lessons) },
            modifier = Modifier.padding(horizontal = 12.dp)
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
                    imageVector = vectorResource(Res.drawable.account_circle_48px),
                    modifier = Modifier.size(32.dp),
                    contentDescription = "",
                    tint = navBarIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text("Profile") }
        )
        NavigationBarItem(
            selected = currentScreen == AppScreen.Study,
            onClick = { onClick(AppScreen.Study) },
            icon = {
                Icon(
                    imageVector = vectorResource(Res.drawable.school_48px),
                    modifier = Modifier.size(32.dp),
                    contentDescription = "",
                    tint = navBarIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text("Study") }
        )
        NavigationBarItem(
            selected = currentScreen == AppScreen.Lessons,
            onClick = { onClick(AppScreen.Lessons) },
            icon = {
                Icon(
                    imageVector = vectorResource(Res.drawable.library_books_48px),
                    modifier = Modifier.size(32.dp),
                    contentDescription = "",
                    tint = navBarIconColor(currentScreen == AppScreen.Study)
                )
            },
            label = { Text("Lessons") }
        )
    }
}

@Composable
fun navBarIconColor(isSelected: Boolean) = if (isSelected) {
    NavigationBarItemDefaults.colors().selectedIconColor
} else {
    NavigationBarItemDefaults.colors().unselectedIconColor
}
