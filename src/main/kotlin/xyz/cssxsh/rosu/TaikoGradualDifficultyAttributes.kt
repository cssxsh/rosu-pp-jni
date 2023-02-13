package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class TaikoGradualDifficultyAttributes
@PublishedApi internal constructor(override val ptr: NativePointer, override val map: Beatmap) :
    GradualDifficultyAttributes(mode = GameMode.Taiko) {

    public constructor(map: Beatmap, mods: Long) : this(ptr = create(map = map.ptr, mods = mods), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    protected override fun clone(): TaikoGradualDifficultyAttributes {
        return TaikoGradualDifficultyAttributes(ptr = clone(ptr = ptr), map = map)
    }

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    override fun next(): TaikoDifficultyAttributes {
        val next = next(ptr = ptr)
        if (next == 0L) throw NotImplementedError("return is null")
        return TaikoDifficultyAttributes(ptr = next)
    }

    override fun hasNext(): Boolean = hasNext(ptr = ptr)

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        internal external fun create(map: NativePointer, mods: Long): NativePointer

        @JvmStatic
        internal external fun clone(ptr: NativePointer): NativePointer

        @JvmStatic
        internal external fun destroy(ptr: NativePointer)

        @JvmStatic
        internal external fun debug(ptr: NativePointer, pretty: Boolean): String

        @JvmStatic
        internal external fun next(ptr: NativePointer): NativePointer

        @JvmStatic
        internal external fun hasNext(ptr: NativePointer): Boolean
    }
}