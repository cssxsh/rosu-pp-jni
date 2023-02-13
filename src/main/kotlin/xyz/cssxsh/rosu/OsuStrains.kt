package xyz.cssxsh.rosu

public class OsuStrains @PublishedApi internal constructor(override val ptr: NativePointer) :
    Strains(mode = GameMode.Osu) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): OsuStrains = OsuStrains(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    public fun length(): Long = len(ptr = ptr)

    public val aim: DoubleArray get() = getAim(ptr = ptr)

    public val aimNoSliders: DoubleArray get() = getAimNoSliders(ptr = ptr)

    public val speed: DoubleArray get()  = getSpeed(ptr = ptr)

    public val flashlight: DoubleArray get() = getFlashlight(ptr = ptr)

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
        internal external fun getAim(ptr: NativePointer): DoubleArray

        @JvmStatic
        internal external fun getAimNoSliders(ptr: NativePointer): DoubleArray

        @JvmStatic
        internal external fun getSpeed(ptr: NativePointer): DoubleArray

        @JvmStatic
        internal external fun getFlashlight(ptr: NativePointer): DoubleArray
    }
}