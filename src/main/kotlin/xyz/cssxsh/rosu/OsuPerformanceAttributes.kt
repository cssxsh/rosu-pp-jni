package xyz.cssxsh.rosu

public class OsuPerformanceAttributes @PublishedApi internal constructor(override val ptr: NativePointer) :
    PerformanceAttributes(mode = GameMode.Osu) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): OsuPerformanceAttributes = OsuPerformanceAttributes(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    override fun pp(): Double = pp(ptr = ptr)

    override fun stars(): Double = stars(ptr = ptr)

    override fun maxCombo(): Long = maxCombo(ptr = ptr)

    override val difficulty: OsuDifficultyAttributes get() = OsuDifficultyAttributes(ptr = getDifficulty(ptr = ptr))

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
        internal external fun pp(ptr: NativePointer): Double

        @JvmStatic
        internal external fun stars(ptr: NativePointer): Double

        @JvmStatic
        internal external fun maxCombo(ptr: NativePointer): Long

        @JvmStatic
        internal external fun getDifficulty(ptr: NativePointer): NativePointer
    }
}