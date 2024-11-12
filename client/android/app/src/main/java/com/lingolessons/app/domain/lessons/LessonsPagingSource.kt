package com.lingolessons.app.domain.lessons

import androidx.compose.ui.text.intl.Locale
import androidx.compose.ui.text.toLowerCase
import androidx.paging.PagingSource
import androidx.paging.PagingState
import com.lingolessons.shared.DomainException
import com.lingolessons.shared.DomainInterface
import com.lingolessons.shared.Lesson

class LessonsPagingSource(
    private val domain: DomainInterface,
    private val searchText: String? = null,
) : PagingSource<Int, Lesson>() {
    override suspend fun load(params: LoadParams<Int>): LoadResult<Int, Lesson> = try {
        val currentPage = params.key ?: 0
        val lessons: List<Lesson> = domain.getLessons(currentPage.toUByte()).let {
            if (searchText.isNullOrBlank()) {
                it
            } else {
                val text = searchText.toLowerCase(Locale.current)
                it.filter { lesson ->
                    lesson.title.toLowerCase(Locale.current).contains(text)
                }
            }
        }

        LoadResult.Page(
            data = lessons,
            prevKey = if (currentPage == 0) null else currentPage - 1,
            nextKey = if (lessons.isNotEmpty()) currentPage + 1 else null
        )
    } catch (e: DomainException) {
        LoadResult.Error(e)
    }

    override fun getRefreshKey(state: PagingState<Int, Lesson>): Int? {
        return null
    }
}
