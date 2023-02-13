package xyz.cssxsh.rosu

import kotlin.test.*

internal class AnyPPTest {

    private fun assertAnyPP(expected: GameMode, pp: AnyPP) {
        assertEquals(expected, pp.mode)
        when (expected) {
            GameMode.Osu -> {
                assertIs<OsuPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)
            }
            GameMode.Taiko -> {
                assertIs<TaikoPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)
            }
            GameMode.Catch -> {
                assertIs<CatchPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)
            }
            GameMode.Mania -> {
                assertIs<ManiaPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)
            }
        }
    }

    @Test
    fun create() {
        val osu = AnyPP.map(path = "./maps/2785319.osu")
        assertAnyPP(GameMode.Osu, osu)

        val taiko = AnyPP.map(path = "./maps/1028484.osu")
        assertAnyPP(GameMode.Taiko, taiko)

        val catch = AnyPP.map(path = "./maps/2118524.osu")
        assertAnyPP(GameMode.Catch, catch)

        val mania = AnyPP.map(path = "./maps/1974394.osu")
        assertAnyPP(GameMode.Mania, mania)
    }
}