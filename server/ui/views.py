#  Copyright 2003-2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.
from django.contrib.auth import authenticate, login, logout
from django.core.exceptions import ValidationError
from django.forms import modelformset_factory
from django.http import HttpResponseRedirect
from django.shortcuts import render, redirect
from django.utils.translation import gettext as _

from api.models import Lesson, Fact
from ui.forms import LoginForm, RegisterForm, FactForm, BaseFactFormSet, LessonForm


def home_page(request):
    if request.user.is_authenticated:
        return render(request, 'home.html', {
            'username': request.user.username
        })
    else:
        return render(request, 'index.html')


def login_page(request):
    form = LoginForm(request.POST or None)
    if request.method == "POST":
        if form.is_valid():
            username = form.cleaned_data['username']
            password = form.cleaned_data['password']
            user = authenticate(request, username=username, password=password)
            if user:
                login(request, user)
                return redirect('/')
            else:
                form.add_error(None, ValidationError(_("Username or password incorrect. Please try again.")))

    return render(request, 'login.html', {
        'form': form
    })


def register_page(request):
    if request.method == "POST":
        form = RegisterForm(request.POST)
        if form.is_valid():
            form.save()
            return redirect('/')
    else:
        form = RegisterForm()

    return render(request, 'register.html', {
        'form': form,
    })


def logout_page(request):
    logout(request)
    return redirect('/')


def study_page(request):
    return render(request, "study.html", {
    })


def lessons_page(request):
    return render(request, "lessons.html", {
        "lessons": Lesson.objects.filter(owner=request.user.id)
    })


def new_lesson_page(request):
    lesson = Lesson(owner=request.user)
    form = LessonForm(request.POST or None, instance=lesson)
    if request.method == "POST":
        if form.is_valid():
            lesson = form.save()
            return HttpResponseRedirect(f"/lesson/{lesson.id}")
    return render(request, "lesson_new.html", {
        "lesson": lesson,
        "form": form,
        "languages": _("Languages"),
    })


def lesson_page(request, id):
    lesson = Lesson.objects.get(id=id)
    form = LessonForm(request.POST or None, instance=lesson)
    FactFormSet = modelformset_factory(Fact, formset=BaseFactFormSet, form=FactForm, extra=1, can_delete=True)

    if request.method == "POST":
        formset = FactFormSet(request.POST, form_kwargs={'lesson': lesson})
        is_form_valid = form.is_valid()
        is_formset_valid = formset.is_valid()
        if is_form_valid and is_formset_valid:
            form.save()
            formset.save()
            return HttpResponseRedirect(f"/lesson/{id}")
    else:
        formset = FactFormSet(queryset=Fact.objects.filter(lesson_id=id), form_kwargs={'lesson': lesson})

    return render(request, "lesson.html", {
        "lesson": lesson,
        "form": form,
        "languages": _("Languages"),
        "formset": formset,
    })
