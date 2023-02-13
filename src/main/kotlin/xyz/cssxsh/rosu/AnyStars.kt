package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public sealed interface AnyStars : Cloneable {

    public val mode: GameMode

    public val map: Beatmap

    @ROsuPP
    public fun calculate(): DifficultyAttributes

    @ROsuPP
    public fun strains(): Strains

    @ROsuPP
    public fun mode(target: GameMode): AnyStars = this

    @ROsuPP
    public fun mods(flag: Long): AnyStars

    @ROsuPP
    public fun passedObjects(number: Long): AnyStars

    @ROsuPP
    public fun clockRate(value: Double): AnyStars

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        @JvmName("create")
        public operator fun invoke(map: Beatmap): AnyStars {
            return when (map.mode) {
                GameMode.Osu -> OsuStars(map = map)
                GameMode.Taiko -> TaikoStars(map = map)
                GameMode.Catch -> CatchStars(map = map)
                GameMode.Mania -> ManiaStars(map = map)
            }
        }

        @JvmStatic
        @JvmName("map")
        public fun map(bytes: ByteArray): AnyStars {
            val map = Beatmap(bytes = bytes)
            return when (map.mode) {
                GameMode.Osu -> OsuStars(map = map)
                GameMode.Taiko -> TaikoStars(map = map)
                GameMode.Catch -> CatchStars(map = map)
                GameMode.Mania -> ManiaStars(map = map)
            }
        }

        @JvmStatic
        @JvmName("map")
        public fun map(path: String): AnyStars {
            val map = Beatmap(path = path)
            return when (map.mode) {
                GameMode.Osu -> OsuStars(map = map)
                GameMode.Taiko -> TaikoStars(map = map)
                GameMode.Catch -> CatchStars(map = map)
                GameMode.Mania -> ManiaStars(map = map)
            }
        }
    }
}