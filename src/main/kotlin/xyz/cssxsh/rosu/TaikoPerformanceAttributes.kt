package xyz.cssxsh.rosu

public class TaikoPerformanceAttributes internal constructor(@PublishedApi override val ptr: Long) :
    PerformanceAttributes(mode = GameMode.Taiko) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): TaikoPerformanceAttributes = TaikoPerformanceAttributes(ptr = clone(ptr))

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