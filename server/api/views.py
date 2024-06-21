#  Copyright 2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.

from django.shortcuts import render
from rest_framework import viewsets, generics
from django.contrib.auth.models import User
from .serializers import UserSerializer, LessonSerializer, LessonDetailSerializer, FactSerializer
from .models import Lesson, Fact


# ViewSets define the view behavior.
class UserViewSet(viewsets.ModelViewSet):
    queryset = User.objects.all()
    serializer_class = UserSerializer


class LessonFactViewSet(viewsets.ModelViewSet):
    queryset = Fact.objects.all()
    serializer_class = FactSerializer


class LessonViewSet(viewsets.ModelViewSet):
    queryset = Lesson.objects.all()
    serializer_class = LessonSerializer


class LessonViewSet(viewsets.ModelViewSet):
    serializer_classes = {
        'list': LessonSerializer,
        'retrieve': LessonDetailSerializer,
        # ... other actions
    }
    serializer_class = LessonSerializer

    def get_queryset(self):
        owner = self.request.query_params.get('owner', None)
        if owner is None:
            return Lesson.objects.all()
        else:
            return Lesson.objects.filter(owner__username=owner)

    def get_serializer_class(self):
        return self.serializer_classes.get(self.action, self.serializer_class)
