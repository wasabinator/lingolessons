#  Copyright 2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.

from django.test import TestCase
from django.contrib.auth.models import User
from rest_framework.test import APIRequestFactory, force_authenticate
from .views import LessonViewSet

#from django.urls import reverse

# Create your tests here.
class LessonsTestCase(TestCase):
    fixtures = ["languages", "users", "lessons"]

    def setUp(self):
        self.factory = APIRequestFactory()
        self.view = LessonViewSet.as_view({'get': 'list'})
        self.bluto = User.objects.get(username='bluto')
        self.melt = User.objects.get(username='melt')

    def test_expect_one_public_lesson_when_not_authenticated(self):
        request = self.factory.get("/api/v1/lessons")
        response = self.view(request)
        self.assertEqual(1, response.data['count'])
        self.assertEqual(
            '447b7087-6761-49a9-a0fb-742dace1a5bb',
            response.data['results'][0]['id']
        )

    def test_bluto_sees_bluto_all_lesson(self):
        request = self.factory.get("/api/v1/lessons")
        force_authenticate(request, user=self.melt)
        response = self.view(request)
        self.assertEqual(3, response.data['count'])

    def test_anon_user_sees_bluto_one_public_lesson(self):
        request = self.factory.get("/api/v1/lessons")
        force_authenticate(request, user=self.bluto)
        response = self.view(request)
        self.assertEqual(1, response.data['count'])
        self.assertEqual(
            '447b7087-6761-49a9-a0fb-742dace1a5bb',
            response.data['results'][0]['id']
        )
