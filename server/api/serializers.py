#  Copyright 2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.

from django.contrib.auth.models import User
from rest_framework import serializers
from .models import Language, Lesson, LessonFact, Fact, Element


# Serializers define the API representation.
class UserSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = ['id', 'username']
        ref_name = "Username"


# class UserCreateSerializer(BaseUserCreateSerializer):
#     class Meta(BaseUserCreateSerializer.Meta):
#         fields = ['id', 'username', 'first_name', 'last_name', 'email', 'password']


class LanguageSerializer(serializers.ModelSerializer):
    class Meta:
        model = Language
        fields = ['code']


class ElementSerializer(serializers.ModelSerializer):
    language = LanguageSerializer(many=False)

    class Meta:
        model = Element
        fields = ['id', 'value', 'language']


class FactSerializer(serializers.ModelSerializer):
    elements = ElementSerializer(many=True)

    class Meta:
        model = Fact
        fields = ['id', 'elements']


class LessonFactSerializer(serializers.ModelSerializer):
    fact = FactSerializer(many=False)

    class Meta:
        model = LessonFact
        fields = ['id', 'fact', 'hint']


class LessonSerializer(serializers.ModelSerializer):
    owner = serializers.SlugRelatedField(read_only=True, slug_field='username')

    class Meta:
        model = Lesson
        fields = ['id', 'title', 'owner']


class LessonDetailSerializer(serializers.ModelSerializer):
    facts = LessonFactSerializer(many=True)
    owner = UserSerializer(many=False)

    class Meta:
        model = Lesson
        fields = ['title', 'owner', 'facts']
        depth = 1

# class UserSerializer(BaseUserSerializer):
#     class Meta(BaseUserSerializer.Meta):
#         fields = ['id', 'username', 'first_name', 'last_name', 'email']
