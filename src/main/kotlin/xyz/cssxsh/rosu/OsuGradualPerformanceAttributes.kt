package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class OsuGradualPerformanceAttributes
@PublishedApi internal constructor(override val ptr: NativePointer, override val map: Beatmap) :
    GradualPerformanceAttributes(mode = GameMode.Osu) {

    public constructor(map: Beatmap, mods: Long) : this(ptr = create(map = map.ptr, mods = mods), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    @ROsuPP
    override fun processNext(state: ScoreState, number: Int): OsuPerformanceAttributes? {
        val next = next(ptr = ptr, state = state.ptr, number = number)
        if (next == 0L) return null
        return OsuPerformanceAttributes(ptr = next)
    }

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        internal external fun create(map: NativePointer, mods: Long): NativePointer

        @JvmStatic
        internal external fun destroy(ptr: NativePointer)

        @JvmStatic
        internal external fun debug(ptr: NativePointer, pretty: Boolean): String

        @JvmStatic
        internal external fun next(ptr: NativePointer, state: NativePointer, number: Int): NativePointer
    }
}