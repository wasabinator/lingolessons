#  Copyright 2003-2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.
from django.template.defaulttags import url
from django.urls import path

from ui.views import home_page, login_page, register_page, logout_page, study_page, lessons_page, lesson_page, \
    new_lesson_page

urlpatterns = [
    path('', home_page, name='home_page'),
    path('login/', login_page, name='login_page'),
    path('logout/', logout_page, name='logout_page'),
    path('register/', register_page, name='register_page'),
    path('study/', study_page, name='study_page'),
    path('lessons/', lessons_page, name='lessons_page'),
    path('lesson/new/', new_lesson_page, name='new_lesson_page'),
    path('lesson/<str:id>/', lesson_page, name='lesson_page'),
]
