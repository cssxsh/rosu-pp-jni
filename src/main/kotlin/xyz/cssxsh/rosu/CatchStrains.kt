package xyz.cssxsh.rosu


public class CatchStrains @PublishedApi internal constructor(override val ptr: Long) :
    Strains(mode = GameMode.Catch) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): CatchStrains = CatchStrains(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    public fun length(): Long = len(ptr = ptr)

    public val sectionLength: Double get() = getSectionLength(ptr = ptr)

    public val movement: DoubleArray get() = getMovement(ptr = ptr)

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        internal external fun clone(ptr: NativePointer): NativePointer

        @JvmStatic
        internal external fun destroy(ptr: NativePointer)

        @JvmStatic
        internal external fun debug(ptr: NativePointer, pretty: Boolean): String

        @JvmStatic
        internal external fun len(ptr: NativePointer): Long

        @JvmStatic
        internal external fun getSectionLength(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getMovement(ptr: NativePointer): DoubleArray
    }
}