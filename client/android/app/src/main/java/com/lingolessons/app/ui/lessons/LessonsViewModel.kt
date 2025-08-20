package com.lingolessons.app.ui.lessons

import androidx.lifecycle.viewModelScope
import androidx.paging.Pager
import androidx.paging.PagingConfig
import androidx.paging.PagingData
import androidx.paging.PagingSourceFactory
import androidx.paging.cachedIn
import com.lingolessons.app.domain.DomainState
import com.lingolessons.app.domain.lessons.LessonsPagingSource
import com.lingolessons.app.ui.common.DomainStateViewModel
import com.lingolessons.app.ui.lessons.LessonsViewModel.ScreenData
import com.lingolessons.shared.Lesson
import kotlinx.coroutines.flow.Flow

class LessonsViewModel(
    domainState: DomainState,
    private val pagingSourceFactory: (String) -> PagingSourceFactory<Int, Lesson> = { text ->
        PagingSourceFactory { LessonsPagingSource(domain = domainState.domain, searchText = text) }
    }
) :
    DomainStateViewModel<ScreenData>(
        domainState = domainState,
        initData = ScreenData(),
    ) {
    init {
        updateData { it.copy(lessons = initPager()) }
    }

    private fun initPager(text: String = ""): Flow<PagingData<Lesson>> =
        Pager(
                config =
                    PagingConfig(
                        pageSize = -1, // Not used
                        initialLoadSize = 1, // Not used
                        prefetchDistance = 0, // Only when requested
                        maxSize = 50,
                        enablePlaceholders = true,
                    ),
                initialKey = 0,
                pagingSourceFactory = pagingSourceFactory(text),
            )
            .flow
            .cachedIn(viewModelScope) // This will de-cache when detached from observation

    fun refresh() {
        updateData { it.copy(lessons = initPager(it.filterText)) }
    }

    fun updateFilterText(text: String) {
        updateData { it.copy(filterText = text, lessons = initPager(text)) }
    }

    data class ScreenData(
        val filterText: String = "",
        val lessons: Flow<PagingData<Lesson>>? = null,
    )
}
