package xyz.cssxsh.rosu

public class CatchDifficultyAttributes @PublishedApi internal constructor(override val ptr: NativePointer) :
    DifficultyAttributes(mode = GameMode.Catch) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): CatchDifficultyAttributes = CatchDifficultyAttributes(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    override fun stars(): Double = getStars(ptr = ptr)

    override fun maxCombo(): Long = maxCombo(ptr = ptr)

    public val stars: Double get() = getStars(ptr = ptr)

    public val ar: Double get() = getAR(ptr = ptr)

    public val fruits: Long get() = getNFruits(ptr = ptr)

    public val droplets: Long get() = getNDroplets(ptr = ptr)

    public val tinyDroplets: Long get() = getNTinyDroplets(ptr = ptr)

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
        internal external fun getStars(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getAR(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getNFruits(ptr: NativePointer): Long

        @JvmStatic
        internal external fun getNDroplets(ptr: NativePointer): Long

        @JvmStatic
        internal external fun getNTinyDroplets(ptr: NativePointer): Long

        @JvmStatic
        internal external fun maxCombo(ptr: NativePointer): Long
    }
}