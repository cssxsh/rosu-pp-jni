package xyz.cssxsh.rosu

public class CatchPerformanceAttributes internal constructor(@PublishedApi override val ptr: Long) :
    PerformanceAttributes(mode = GameMode.Catch) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): CatchPerformanceAttributes = CatchPerformanceAttributes(ptr = clone(ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    public companion object {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        internal external fun clone(ptr: Long): Long

        @JvmStatic
        internal external fun destroy(ptr: Long)

        @JvmStatic
        internal external fun debug(ptr: Long, pretty: Boolean): String
    }
}