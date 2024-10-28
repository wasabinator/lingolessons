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
    language1 = serializers.SlugRelatedField(read_only=True, slug_field='code')
    language2 = serializers.SlugRelatedField(read_only=True, slug_field='code')

    class Meta:
        model = Lesson
        fields = ['id', 'title', 'type', 'language1', 'language2', 'owner']

    def to_representation(self, instance):
        data = super().to_representation(instance)
        data['updated_at'] = int(time.mktime(instance.updated_at.timetuple()))  # Add updated at in utc format
        return data


class LessonDetailSerializer(serializers.ModelSerializer):
    facts = FactSerializer(source='fact_set', many=True)
    owner = serializers.SlugRelatedField(read_only=True, slug_field='username')

    class Meta:
        model = Lesson
        fields = ['title', 'type', 'language1', 'language2', 'owner', 'facts']

    def to_representation(self, instance):
        data = super().to_representation(instance)
        data['updated_at'] = int(time.mktime(instance.updated_at.timetuple()))  # Add updated at in utc format
        return data


# Token serializers needed by drf-yasg

# from drf_yasg.utils import swagger_auto_schema
# from rest_framework import serializers, status
# from rest_framework_simplejwt.views import (
#     TokenBlacklistView,
#     TokenObtainPairView,
#     TokenRefreshView,
#     TokenVerifyView,
# )


# class TokenObtainPairResponseSerializer(serializers.Serializer):
#     access = serializers.CharField()
#     refresh = serializers.CharField()

#     def create(self, validated_data):
#         raise NotImplementedError()

#     def update(self, instance, validated_data):
#         raise NotImplementedError()


# class DecoratedTokenObtainPairView(TokenObtainPairView):
#     @swagger_auto_schema(
#         responses={
#             status.HTTP_200_OK: TokenObtainPairResponseSerializer,
#         }
#     )
#     def post(self, request, *args, **kwargs):
#         return super().post(request, *args, **kwargs)


# class TokenRefreshResponseSerializer(serializers.Serializer):
#     access = serializers.CharField()

#     def create(self, validated_data):
#         raise NotImplementedError()

#     def update(self, instance, validated_data):
#         raise NotImplementedError()


# class DecoratedTokenRefreshView(TokenRefreshView):
#     @swagger_auto_schema(
#         responses={
#             status.HTTP_200_OK: TokenRefreshResponseSerializer,
#         }
#     )
#     def post(self, request, *args, **kwargs):
#         return super().post(request, *args, **kwargs)


# class TokenVerifyResponseSerializer(serializers.Serializer):
#     def create(self, validated_data):
#         raise NotImplementedError()

#     def update(self, instance, validated_data):
#         raise NotImplementedError()


# class DecoratedTokenVerifyView(TokenVerifyView):
#     @swagger_auto_schema(
#         responses={
#             status.HTTP_200_OK: TokenVerifyResponseSerializer,
#         }
#     )
#     def post(self, request, *args, **kwargs):
#         return super().post(request, *args, **kwargs)


# class TokenBlacklistResponseSerializer(serializers.Serializer):
#     def create(self, validated_data):
#         raise NotImplementedError()

#     def update(self, instance, validated_data):
#         raise NotImplementedError()


# class DecoratedTokenBlacklistView(TokenBlacklistView):
#     @swagger_auto_schema(
#         responses={
#             status.HTTP_200_OK: TokenBlacklistResponseSerializer,
#         }
#     )
#     def post(self, request, *args, **kwargs):
#         return super().post(request, *args, **kwargs)