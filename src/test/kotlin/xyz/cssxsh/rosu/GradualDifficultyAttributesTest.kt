package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap
import kotlin.test.*

internal class GradualDifficultyAttributesTest {
    @Test
    fun create() {
        val osu = Beatmap(path = "./maps/2785319.osu")
        for (attributes in GradualDifficultyAttributes(map = osu, mods = 0)) {
            assertEquals(GameMode.Osu, attributes.mode)
            assertNotEquals(0, attributes.ptr)
        }

        val taiko = Beatmap(path = "./maps/1028484.osu")
        for (attributes in GradualDifficultyAttributes(map = taiko, mods = 0)) {
            assertEquals(GameMode.Taiko, attributes.mode)
            assertNotEquals(0, attributes.ptr)
        }

        val catch = Beatmap(path = "./maps/2118524.osu")
        for (attributes in GradualDifficultyAttributes(map = catch, mods = 0)) {
            assertEquals(GameMode.Catch, attributes.mode)
            assertNotEquals(0, attributes.ptr)
        }

        val mania = Beatmap(path = "./maps/1974394.osu")
        for (attributes in GradualDifficultyAttributes(map = mania, mods = 0)) {
            assertEquals(GameMode.Mania, attributes.mode)
            assertNotEquals(0, attributes.ptr)
        }
    }
}