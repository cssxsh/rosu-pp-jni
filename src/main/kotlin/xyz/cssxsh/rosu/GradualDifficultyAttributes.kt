package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public sealed class GradualDifficultyAttributes(public val mode: GameMode) : Cloneable,
    Iterator<DifficultyAttributes> {
    internal abstract val ptr: Long
    public abstract val map: Beatmap

    public companion object Native {
        @JvmStatic
        @JvmName("create")
        public operator fun invoke(map: Beatmap, mods: Long): GradualDifficultyAttributes {
            return when (map.mode) {
                GameMode.Osu -> OsuGradualDifficultyAttributes(map = map, mods = mods)
                GameMode.Taiko -> TaikoGradualDifficultyAttributes(map = map, mods = mods)
                GameMode.Catch -> CatchGradualDifficultyAttributes(map = map, mods = mods)
                GameMode.Mania -> ManiaGradualDifficultyAttributes(map = map, mods = mods)
            }
        }
    }
}