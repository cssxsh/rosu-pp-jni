package xyz.cssxsh.rosu

public class OsuDifficultyAttributes @PublishedApi internal constructor(override val ptr: NativePointer) :
    DifficultyAttributes(mode = GameMode.Osu) {

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): OsuDifficultyAttributes = OsuDifficultyAttributes(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    override fun stars(): Double = getStars(ptr = ptr)

    override fun maxCombo(): Long = getMaxCombo(ptr = ptr)

    public val aim: Double get() = getAim(ptr = ptr)

    public val speed: Double get() = getSpeed(ptr = ptr)

    public val flashlight: Double get() = getFlashlight(ptr = ptr)

    public val sliderFactor: Double get() = getSliderFactor(ptr = ptr)

    public val speedNoteCount: Double get() = getSpeedNoteCount(ptr = ptr)

    public val ar: Double get() = getAR(ptr = ptr)

    public val od: Double get() = getOD(ptr = ptr)

    public val hp: Double get() = getHP(ptr = ptr)

    public val circles: Long get() = getNCircles(ptr = ptr)

    public val sliders: Long get() = getNSliders(ptr = ptr)

    public val spinners: Long get() = getNSpinners(ptr = ptr)

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
        internal external fun getAim(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getSpeed(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getFlashlight(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getSliderFactor(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getSpeedNoteCount(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getAR(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getOD(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getHP(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getNCircles(ptr: NativePointer): Long

        @JvmStatic
        internal external fun getNSliders(ptr: NativePointer): Long

        @JvmStatic
        internal external fun getNSpinners(ptr: NativePointer): Long

        @JvmStatic
        internal external fun getStars(ptr: NativePointer): Double

        @JvmStatic
        internal external fun getMaxCombo(ptr: NativePointer): Long
    }
}