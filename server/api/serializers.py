#  Copyright 2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.
import time

from django.contrib.auth.models import User
from rest_framework import serializers
from .models import Language, Lesson, Fact


# Serializers define the API representation.
class UserSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = ['id', 'username']
        ref_name = "Username"


class LanguageSerializer(serializers.ModelSerializer):
    class Meta:
        model = Language
        fields = ['code']


class FactSerializer(serializers.ModelSerializer):
    class Meta:
        model = Fact
        fields = ['id', 'element1', 'element2', 'hint']


class LessonSerializer(serializers.ModelSerializer):
    owner = serializers.SlugRelatedField(read_only=True, slug_field='username')

    class Meta:
        model = Lesson
        fields = ['id', 'title', 'owner']

    def to_representation(self, instance):
        data = super().to_representation(instance)
        data['updated_at'] = time.mktime(instance.updated_at.timetuple())  # Add updated at in utc format
        return data


class LessonDetailSerializer(serializers.ModelSerializer):
    facts = FactSerializer(source='fact_set', many=True)
    owner = serializers.SlugRelatedField(read_only=True, slug_field='username')

    class Meta:
        model = Lesson
        fields = ['title', 'owner', 'facts']

    def to_representation(self, instance):
        data = super().to_representation(instance)
        data['updated_at'] = time.mktime(instance.updated_at.timetuple())  # Add updated at in utc format
        return data
