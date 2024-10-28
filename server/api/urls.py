#  Copyright 2024 Anthony Miceli and contributors. This file is part of LingoLessons.
#  LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
#  GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
#  or (at your option) any later version.
#  LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
#  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
#  You should have received a copy of the GNU General Public License along with LingoLessons.
#  If not, see <https://www.gnu.org/licenses/>.

from django.urls import include, path
from django.views.generic import RedirectView
from rest_framework import routers
from .views import UserViewSet, LessonViewSet, FactViewSet
from rest_framework import permissions
from drf_yasg.views import get_schema_view
from drf_yasg import openapi

schema_view = get_schema_view(
    openapi.Info(
        title="LingoLessons API",
        default_version='v1',
        description="LingoLessons Server API",
        terms_of_service="https://www.google.com/policies/terms/",
        contact=openapi.Contact(email="contact@lingolessons.com"),
        license=openapi.License(name="AGPL License"),
    ),
    public=True,
    permission_classes=[permissions.AllowAny, ],
)

router = routers.DefaultRouter()
router.register(r'users', UserViewSet)
router.register(r'lessons', LessonViewSet, basename='LessonView')
router.register(r'lesson/(?P<id>[\w-]+)/facts', FactViewSet, basename='LessonFactView')

urlpatterns = [
    path(r'', RedirectView.as_view(url='v1', permanent=False), name='index'),
    path(r'v1/', include(router.urls)),
    # path(r'v1/', include('djoser.urls')),
    path(r'v1/', include('djoser.urls.jwt')),
    path(r'v1/swagger<format>/', schema_view.without_ui(cache_timeout=0), name='schema-json'),
    path(r'v1/swagger/', schema_view.with_ui('swagger', cache_timeout=0), name='schema-swagger-ui'),
    path(r'v1/redoc/', schema_view.with_ui('redoc', cache_timeout=0), name='schema-redoc'),
]
