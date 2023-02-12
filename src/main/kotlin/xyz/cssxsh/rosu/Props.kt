package xyz.cssxsh.rosu

internal const val PRETTY_PRINT_KEY: String = "xyz.cssxsh.rosu.debug.pretty"

@Suppress("NOTHING_TO_INLINE")
internal inline fun pretty(): Boolean {
    val value = System.getProperty(PRETTY_PRINT_KEY)
        ?: System.getenv("ROSU_DEBUG_PRETTY")
        ?: "false"

    return value.equals("true", true)
}