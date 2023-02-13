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

//                assertEquals(attributes.stars(), 5.669858729379628)
                assertEquals(attributes.maxCombo(), 909)
            }
            GameMode.Taiko -> {
                assertIs<TaikoPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)

//                assertEquals(attributes.stars(), 2.9778030386845606)
                assertEquals(attributes.maxCombo(), 289)
            }
            GameMode.Catch -> {
                assertIs<CatchPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)

//                assertEquals(attributes.stars(), 3.2502669316166624)
                assertEquals(attributes.maxCombo(), 730)
            }
            GameMode.Mania -> {
                assertIs<ManiaPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)

//                assertEquals(attributes.stars(), 4.824631127426499)
                assertEquals(attributes.maxCombo(), 5064)
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