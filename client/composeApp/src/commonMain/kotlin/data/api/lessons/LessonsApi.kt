package data.api.lessons

import de.jensklingenberg.ktorfit.http.GET
import de.jensklingenberg.ktorfit.http.Query

internal interface LessonsApi {
    @GET("lessons")
    suspend fun getLessons(
        @Query("owner") owner: String? = null,
        @Query("title") title: String? = null,
        @Query("since") since: Int? = null,
        @Query("page") page: Int = 1,
    ): LessonsResponse
}