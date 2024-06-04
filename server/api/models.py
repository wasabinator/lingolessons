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


class Element(models.Model):
    language = models.ForeignKey(Language, related_name='language', on_delete=models.CASCADE)
    value = models.CharField(max_length=255)
    value_types = (
        (1, "Vocabulary"),
        (2, "Grammar"),
    )
    type = models.IntegerField(choices=value_types, default=1)

    def __str__(self):
        return f"{self.value}({self.language})"


class Fact(models.Model):
    elements = models.ManyToManyField(Element)

    def __str__(self):
        return " = ".join(str(e) for e in self.elements.all())


class LessonFact(models.Model):
    fact = models.OneToOneField(Fact, on_delete=models.CASCADE)
    hint = models.CharField(max_length=255, blank=True, default='')

    # owner = models.ForeignKey(User, related_name='fact_owner', on_delete=models.CASCADE)

    def __str__(self):
        return f"{self.fact}. Hint = {self.hint}"


class Lesson(models.Model):
    title = models.CharField(max_length=255)
    owner = models.ForeignKey(User, related_name='lesson_owner', on_delete=models.CASCADE)
    facts = models.ManyToManyField(LessonFact, related_name='lesson_facts')

    # def get_facts(self):
    #     return LessonFact.objects.filter(lesson=self)

    def __str__(self):
        return self.title
