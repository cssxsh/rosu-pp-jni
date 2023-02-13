package xyz.cssxsh.rosu


public class ManiaStrains @PublishedApi internal constructor(override val ptr: Long) :
    Strains(mode = GameMode.Mania) {

    protected fun finalize(): Unit = CatchStrains.destroy(ptr = ptr)

    override fun clone(): CatchStrains = CatchStrains(ptr = CatchStrains.clone(ptr = ptr))

    override fun toString(): String = CatchStrains.debug(ptr = ptr, pretty = pretty())

    public fun length(): Long = len(ptr = ptr)

    public val sectionLength: Double get() = getSectionLength(ptr = ptr)

    public val strains: DoubleArray get() = getStrains(ptr = ptr)

    public companion object Native {

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
        internal external fun getStrains(ptr: NativePointer): DoubleArray
    }
}