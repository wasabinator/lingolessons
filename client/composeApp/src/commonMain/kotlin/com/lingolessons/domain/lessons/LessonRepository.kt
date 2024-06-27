package com.lingolessons.domain.lessons

import com.lingolessons.domain.common.ItemList
import com.lingolessons.domain.common.ListRepository

typealias LessonList = ItemList<Lesson>

abstract class LessonRepository : ListRepository<LessonRequest, Lesson>()
