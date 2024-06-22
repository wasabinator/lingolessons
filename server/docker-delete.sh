#!/bin/sh
sudo docker rmi -f $(sudo docker images -aq)
