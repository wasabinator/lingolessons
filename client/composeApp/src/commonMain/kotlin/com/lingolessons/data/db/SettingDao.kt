package com.lingolessons.data.db

/**
 * Mockable wrapper around the SettingQueries class
 */
interface SettingDao {
    fun get(key: String): String?
    fun save(key: String, value: String?)
}

val SettingQueries.dao: SettingDao
    get() = object : SettingDao {
        override fun get(key: String) = this@dao.get(key).executeAsOneOrNull()?.value_

        override fun save(key: String, value: String?) =
            this@dao.save(key, value)
    }
