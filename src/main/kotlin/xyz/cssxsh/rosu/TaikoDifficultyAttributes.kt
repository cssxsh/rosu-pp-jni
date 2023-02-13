package xyz.cssxsh.rosu

public class TaikoDifficultyAttributes @PublishedApi internal constructor(override val ptr: NativePointer) :
    DifficultyAttributes(mode = GameMode.Taiko) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): TaikoDifficultyAttributes = TaikoDifficultyAttributes(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    override fun stars(): Double = getStars(ptr = ptr)

    override fun maxCombo(): Long = getMaxCombo(ptr = ptr)

    public val stamina: Double get() = getStamina(ptr = ptr)

    public val rhythm: Double get() = getRhythm(ptr = ptr)

    public val colour: Double get() = getColour(ptr = ptr)

    public val peak: Double get() = getPeak(ptr = ptr)

    public val hitWindow: Double get() = getHitWindow(ptr = ptr)

    public val stars: Double get() = getStars(ptr = ptr)

    public val maxCombo: Long get() = getMaxCombo(ptr = ptr)

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
        internal external fun getStamina(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getRhythm(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getColour(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getPeak(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getHitWindow(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getStars(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getMaxCombo(ptr: NativePointer): Long
    }
}