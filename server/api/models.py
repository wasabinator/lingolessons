#  Copyright 2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.

import uuid
from django.contrib.auth.models import User
from django.db import models


class Language(models.Model):
    code = models.CharField(max_length=5)
    flag = models.CharField(max_length=4)

    class Meta:
        indexes = [
            models.Index(fields=['code'])
        ]

    def __str__(self):
        return self.flag


class Lesson(models.Model):
    id = models.UUIDField(primary_key=True, default=uuid.uuid4, editable=False)
    title = models.CharField(max_length=255)
    owner = models.ForeignKey(User, related_name='lesson_owner', on_delete=models.CASCADE)
    language1 = models.ForeignKey(Language, related_name='language1', on_delete=models.CASCADE)
    language2 = models.ForeignKey(Language, related_name='language2', on_delete=models.CASCADE)

    VOCABULARY = 0
    GRAMMAR = 1
    types = (
        (VOCABULARY, "Vocabulary"),
        (GRAMMAR, "Grammar"),
    )
    type = models.IntegerField(choices=types, default=0)

    PRIVATE = 0
    PUBLIC = 1
    statuses = (
        (PRIVATE, "Private"),
        (PUBLIC, "Public"),
    )
    status = models.IntegerField(choices=statuses, default=PRIVATE)

    is_deleted = models.BooleanField(default=False, editable=False)    
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def get_type_name(self):
        match = [item for item in self.types if item[0] == self.type]
        if match:
            return match[0][1]
        else:
            return "Grammar"

    def get_status_name(self):
        match = [item for item in self.statuses if item[0] == self.type]
        if match:
            return match[0][1]
        else:
            return "Private"
        
    def __str__(self):
        return f"{self.title} [updated: {self.updated_at}]"


class Fact(models.Model):
    id = models.UUIDField(primary_key=True, default=uuid.uuid4)
    lesson = models.ForeignKey(Lesson, on_delete=models.CASCADE)
    element1 = models.CharField(max_length=255)
    element2 = models.CharField(max_length=255)
    hint = models.CharField(max_length=255, blank=True, default='')

    def __str__(self):
        return f"{self.element1} = {self.element2}"
