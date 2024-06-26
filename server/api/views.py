#  Copyright 2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.
import datetime

import pytz
from django.contrib.auth.models import User
from django.db.models.query_utils import Q
from rest_framework import viewsets

from .models import Lesson, Fact
from .serializers import UserSerializer, LessonSerializer, LessonDetailSerializer, FactSerializer


# ViewSets define the view behavior.
class UserViewSet(viewsets.ModelViewSet):
    queryset = User.objects.all()
    serializer_class = UserSerializer


class FactViewSet(viewsets.ModelViewSet):
    serializer_class = FactSerializer

    def get_queryset(self):
        if getattr(self, 'swagger_fake_view', False):  # Swagger inspection won't pass a lesson_id
            return Fact.objects.none()
        return Fact.objects.filter(lesson__id=int(self.kwargs['lesson_id'])).order_by('id')


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
        qs = Q()

        since = self.request.query_params.get('since', None)
        if since is not None:
            qs.add(Q(updated_at__gte=datetime.datetime.fromtimestamp(int(since), tz=pytz.UTC)), Q.AND)
        owner = self.request.query_params.get('owner', None)
        if owner is not None:
            qs.add(Q(owner__username=owner), Q.AND)

        if len(qs) == 0:
            return Lesson.objects.all().order_by('-updated_at')
        else:
            return Lesson.objects.filter(qs).order_by('-updated_at')

    def get_serializer_class(self):
        return self.serializer_classes.get(self.action, self.serializer_class)
