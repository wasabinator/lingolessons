package domain.lessons

import domain.common.ItemList
import domain.common.ListRepository

typealias LessonList = ItemList<Lesson>

abstract class LessonRepository : ListRepository<LessonRequest, Lesson>()
