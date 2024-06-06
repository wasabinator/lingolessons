#  Copyright 2003-2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.

from django import forms
from django.contrib.auth.forms import UserCreationForm
from django.contrib.auth.models import User


class RegisterForm(UserCreationForm):
    class Meta:
        model = User
        fields = ['username', 'password1', 'password2']


class LoginForm(forms.Form):
    username = forms.CharField(required=True)
    password = forms.CharField(widget=forms.PasswordInput, required=True)
