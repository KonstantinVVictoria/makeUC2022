from django.contrib import admin
from django.urls import path, include
from .views import ListView


urlpatterns = [
    path('admin/', admin.site.urls),
    path("", Listvies.as_view()),
]
