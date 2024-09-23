package com.lingolessons.app.ui.profile

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.unit.dp

@Composable
fun ProfileScreen(
    viewModel: ProfileViewModel,
) {
    ProfileScreen(
        logout = viewModel::logout
    )
}

@Composable
fun ProfileScreen(
    logout: () -> Unit
) = Column {
    Text("profile")

    Button(
        modifier = Modifier.testTag("logout"),
        onClick = logout,
    ) {
        Box(modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp)) {
            Text("Logout")
        }
    }
}
