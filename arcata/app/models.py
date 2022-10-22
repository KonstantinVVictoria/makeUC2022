from django.db import models


# Create your models here.
class User(models.Model):
    name = models.CharField(max_length=35)
    uid = models.UUIDField(primary_key=True, editable=False)
    email = models.CharField(max_length=40)
    

class Plant(models.Model):
    name = models.CharField(max_length=35)
    uid = models.UUIDField(primary_key=True, editable=False)
    watering_freq = models.IntegerField(default=0)
    