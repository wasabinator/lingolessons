# Generated by Django 5.0.6 on 2024-07-15 10:07

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('api', '0001_initial'),
    ]

    operations = [
        migrations.AlterField(
            model_name='fact',
            name='id',
            field=models.CharField(max_length=36, primary_key=True, serialize=False),
        ),
        migrations.AlterField(
            model_name='lesson',
            name='id',
            field=models.CharField(max_length=36, primary_key=True, serialize=False),
        ),
    ]
