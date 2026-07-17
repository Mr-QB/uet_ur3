from setuptools import find_packages
from setuptools import setup

setup(
    name='ur3_moveit_control',
    version='0.0.0',
    packages=find_packages(
        include=('ur3_moveit_control', 'ur3_moveit_control.*')),
)
