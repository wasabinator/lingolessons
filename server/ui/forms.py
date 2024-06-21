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
from django.forms import BaseModelFormSet
from django.forms.widgets import HiddenInput, TextInput, CheckboxInput, ChoiceWidget, Select

from api.models import Fact, Lesson
from ui.styles import formInputStyle, formInputErrorStyle


class RegisterForm(UserCreationForm):
    class Meta:
        model = User
        fields = ['username', 'password1', 'password2']


class LoginForm(forms.Form):
    username = forms.CharField(required=True)
    password = forms.CharField(widget=forms.PasswordInput, required=True)


class LessonForm(forms.ModelForm):
    class Meta:
        model = Lesson
        fields = ['title', 'language1', 'language2', 'type']
        widgets = {
            'title': TextInput(attrs={'class': formInputStyle}),
            'language1': Select(attrs={'class': formInputStyle}),
            'language2': Select(attrs={'class': formInputStyle}),
            'type': Select(attrs={'class': formInputStyle}),
        }

    def is_valid(self) -> bool:
        for item in self.errors.as_data().items():
            if item[0] in self.fields:
                self.fields[item[0]].widget.attrs['class'] = formInputErrorStyle
        return super().is_valid()


class FactForm(forms.ModelForm):
    class Meta:
        model = Fact
        fields = ['id', 'element1', 'element2', 'hint']
        widgets = {
            'id': HiddenInput(),
            'element1': TextInput(attrs={'class': f'mandatory {formInputStyle}'}),
            'element2': TextInput(attrs={'class': f'mandatory {formInputStyle}'}),
            'hint': TextInput(attrs={'class': formInputStyle}),
        }

    def __init__(self, data=None, *args, lesson, **kwargs):
        self.lesson = lesson
        super(FactForm, self).__init__(data, *args, **kwargs)

    def clean(self):
        cleaned_data = super().clean()
        if cleaned_data.get('element1') or cleaned_data.get('element2'):
            # Ensure we set the lesson
            cleaned_data['lesson'] = self.lesson
            self.instance.lesson = self.lesson
        return cleaned_data

    def is_valid(self) -> bool:
        for item in self.errors.as_data().items():
            if item[0] in self.fields:
                self.fields[item[0]].widget.attrs['class'] = formInputErrorStyle
        return super().is_valid()


class BaseFactFormSet(BaseModelFormSet):
    deletion_widget = CheckboxInput(attrs={'class': formInputStyle})
