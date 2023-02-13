package xyz.cssxsh.rosu

public class ManiaPerformanceAttributes @PublishedApi internal constructor(override val ptr: Long) :
    PerformanceAttributes(mode = GameMode.Mania) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): ManiaPerformanceAttributes = ManiaPerformanceAttributes(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    override fun pp(): Double = pp(ptr = ptr)

    override fun stars(): Double = stars(ptr = ptr)

    override fun maxCombo(): Long = maxCombo(ptr = ptr)

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        internal external fun clone(ptr: Long): Long

        @JvmStatic
        internal external fun destroy(ptr: Long)

        @JvmStatic
        internal external fun debug(ptr: Long, pretty: Boolean): String

        @JvmStatic
        internal external fun pp(ptr: Long): Double

        @JvmStatic
        internal external fun stars(ptr: Long): Double

        @JvmStatic
        internal external fun maxCombo(ptr: Long): Long
    }
}