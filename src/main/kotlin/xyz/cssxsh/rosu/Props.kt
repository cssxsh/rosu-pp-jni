package xyz.cssxsh.rosu


@PublishedApi
internal const val ROSU_LIBRARY_PATH_PROPERTY: String = "rosu.library.path"

@PublishedApi
internal const val PRETTY_PRINT_KEY: String = "rosu.debug.pretty"

@Suppress("NOTHING_TO_INLINE")
internal inline fun pretty(): Boolean {
    val value = System.getProperty(PRETTY_PRINT_KEY)
        ?: System.getenv("ROSU_DEBUG_PRETTY")
        ?: "false"

    return value.equals("true", true)
}