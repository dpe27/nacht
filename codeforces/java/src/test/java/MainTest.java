import org.junit.jupiter.api.Test;
import tmp.Calculator;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class MainTest {

    @Test
    public void testAdd() {
        assertEquals(3, Calculator.calSum(1, 2));
    }

    @Test
    public void testAdd1() {
        assertEquals(3, Calculator.calSum(1, 2));
    }
    @Test
    public void testAdd2() {
        assertEquals(2, Calculator.calSum(1, 2));
    }
    @Test
    public void testAd3() {
        assertEquals(3, Calculator.calSum(1, 2));
    }
}
