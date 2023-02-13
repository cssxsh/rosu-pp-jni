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

                assertEquals(attributes.maxCombo(), 909)

                val difficulty = attributes.difficulty
                assertEquals(expected, difficulty.mode)
                assertNotEquals(0, difficulty.ptr)
                assertEquals(2.8693628443424104, difficulty.aim)
                assertEquals(2.533869745015772, difficulty.speed)
                assertEquals(2.288770487900865, difficulty.flashlight)
                assertEquals(0.9803052946037858, difficulty.sliderFactor)
                assertEquals(210.36373973116545, difficulty.speedNoteCount)
                assertEquals(9.300000190734863, difficulty.ar)
                assertEquals(8.800000190734863, difficulty.od)
                assertEquals(5.0, difficulty.hp)
                assertEquals(307, difficulty.circles)
                assertEquals(293, difficulty.sliders)
                assertEquals(1, difficulty.spinners)
                assertEquals(5.669858729379628, difficulty.stars)
                assertEquals(909, difficulty.maxCombo)
            }
            GameMode.Taiko -> {
                assertIs<TaikoPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)

                assertEquals(attributes.maxCombo(), 289)

                val difficulty = attributes.difficulty
                assertEquals(expected, difficulty.mode)
                assertNotEquals(0, difficulty.ptr)
                assertEquals(1.4528845068865617, difficulty.stamina)
                assertEquals(0.20130047251681948, difficulty.rhythm)
                assertEquals(1.0487315549761433, difficulty.colour)
                assertEquals(1.8881824429738323, difficulty.peak)
                assertEquals(35.0, difficulty.hitWindow)
                assertEquals(2.9778030386845606, difficulty.stars)
                assertEquals(289, difficulty.maxCombo)
            }
            GameMode.Catch -> {
                assertIs<CatchPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)

                assertEquals(attributes.maxCombo(), 730)

                val difficulty = attributes.difficulty
                assertEquals(expected, difficulty.mode)
                assertNotEquals(0, difficulty.ptr)
                assertEquals(3.2502669316166624, difficulty.stars)
                assertEquals(8.0, difficulty.ar)
                assertEquals(728, difficulty.fruits)
                assertEquals(2, difficulty.droplets)
                assertEquals(291, difficulty.tinyDroplets)
            }
            GameMode.Mania -> {
                assertIs<ManiaPP>(pp)
                assertNotEquals(0, pp.ptr)
                val attributes = pp.calculate()
                assertEquals(expected, attributes.mode)
                assertNotEquals(0, attributes.ptr)

                assertEquals(attributes.maxCombo(), 5064)

                val difficulty = attributes.difficulty
                assertEquals(expected, difficulty.mode)
                assertNotEquals(0, difficulty.ptr)
                assertEquals(4.824631127426499, difficulty.stars)
                assertEquals(40.0, difficulty.hitWindow)
                assertEquals(5064, difficulty.maxCombo)
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