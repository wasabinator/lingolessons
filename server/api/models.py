#  Copyright 2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.

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
    id = models.CharField(max_length=36, primary_key=True)
    title = models.CharField(max_length=255)
    owner = models.ForeignKey(User, related_name='lesson_owner', on_delete=models.CASCADE)
    language1 = models.ForeignKey(Language, related_name='language1', on_delete=models.CASCADE)
    language2 = models.ForeignKey(Language, related_name='language2', on_delete=models.CASCADE)
    VOCABULARY = 0
    GRAMMAR = 1
    value_types = (
        (VOCABULARY, "Vocabulary"),
        (GRAMMAR, "Grammar"),
    )
    type = models.IntegerField(choices=value_types, default=0)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def get_type_name(self):
        match = [item for item in self.value_types if item[0] == self.type]
        if match:
            return match[0][1]
        else:
            return "Grammar"

    def __str__(self):
        return f"{self.title} [updated: {self.updated_at}]"


class Fact(models.Model):
    id = models.CharField(max_length=36, primary_key=True)
    lesson = models.ForeignKey(Lesson, on_delete=models.CASCADE)
    element1 = models.CharField(max_length=255)
    element2 = models.CharField(max_length=255)
    hint = models.CharField(max_length=255, blank=True, default='')

    def __str__(self):
        return f"{self.element1} = {self.element2}"
