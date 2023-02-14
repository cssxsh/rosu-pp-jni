package xyz.cssxsh.rosu

public class TaikoStrains @PublishedApi internal constructor(override val ptr: NativePointer) :
    Strains(mode = GameMode.Taiko) {

    protected fun finalize(): Unit = OsuStrains.destroy(ptr = ptr)

    override fun clone(): OsuStrains = OsuStrains(ptr = OsuStrains.clone(ptr = ptr))

    override fun toString(): String = OsuStrains.debug(ptr = ptr, pretty = pretty())

    public fun length(): Long = len(ptr = ptr)

    public val sectionLength: Double get() = getSectionLength(ptr = ptr)

    public val color: DoubleArray get() = getColor(ptr = ptr)

    public val rhythm: DoubleArray get() = getRhythm(ptr = ptr)

    public val stamina: DoubleArray get() = getStamina(ptr = ptr)

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
        internal external fun getColor(ptr: NativePointer): DoubleArray

        @JvmStatic
        internal external fun getRhythm(ptr: NativePointer): DoubleArray

        @JvmStatic
        internal external fun getStamina(ptr: NativePointer): DoubleArray
    }
}