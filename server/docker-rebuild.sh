#
# Copyright 2003-2024 Anthony Miceli and contributors. This file is part of LingoLessons.
# LingoLessons is free software: you can redistribute it and/or modify it under the terms of the
# GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
# or (at your option) any later version.
# LingoLessons is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
# warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
# You should have received a copy of the GNU General Public License along with LingoLessons.
# If not, see <https://www.gnu.org/licenses/>.
#

#docker-compose pull
docker-compose up --force-recreate --remove-orphans --build
docker image prune -f