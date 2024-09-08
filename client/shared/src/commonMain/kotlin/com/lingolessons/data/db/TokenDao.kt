package com.lingolessons.data.db

/**
 * Mockable wrapper around the TokenQueries class
 */
interface TokenDao {
    fun get(): Token?
    fun save(username: String, authToken: String, refreshToken: String)
    fun delete()
}

val TokenQueries.dao: TokenDao
    get() = object : TokenDao {
        override fun get() = this@dao.get().executeAsOneOrNull()

        override fun save(username: String, authToken: String, refreshToken: String) =
            this@dao.save(username, authToken, refreshToken)

        override fun delete() = this@dao.delete()
    }
