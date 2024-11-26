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
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.shared.Lesson
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

class LessonsViewModel(
    domainState: DomainState,
    private val pagingSourceFactory: (String) -> PagingSourceFactory<Int, Lesson> = { text ->
        PagingSourceFactory {
            LessonsPagingSource(
                domain = domainState.domain,
                searchText = text
            )
        }
    },
) : DomainStateViewModel(domainState = domainState) {
    private val _state = MutableStateFlow(
        State(
            lessons = initPager()
        )
    )
    val state = _state.asStateFlow()

    fun updateFilterText(text: String) {
        _state.update {
            it.copy(
                filterText = text,
                lessons = initPager(text)
            )
        }
    }

    private fun initPager(text: String = ""): Flow<PagingData<Lesson>> =
        Pager(
            config = PagingConfig(
                pageSize = -1, // Not used
                initialLoadSize = 1, // Not used
                prefetchDistance = 0, // Only when requested
                maxSize = 50,
                enablePlaceholders = true
            ),
            initialKey = 0,
            pagingSourceFactory = pagingSourceFactory(text),
        ).flow.cachedIn(viewModelScope) // This will de-cache when detached from observation

    fun refresh() {
        _state.update {
            it.copy(
                lessons = initPager(it.filterText)
            )
        }
    }

    data class State(
        val filterText: String = "",
        val lessons: Flow<PagingData<Lesson>>,
        override val status: ScreenState.Status = ScreenState.Status.None,
    ) : ScreenState

    override fun updateStatus(status: ScreenState.Status) {
        _state.update {
            it.copy(
                status = status
            )
        }
    }
}
