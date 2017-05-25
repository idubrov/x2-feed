Hardware:

 * IM483 stepper driver
 * DPP100-24 24V power supply
 * TOBSUN EA15-5V 24V to 5V power supply
 *

Electronics:

 * Hall sensor: AH3363Q (high sensitivity)
 * Capacitors: 0.1uF x 1005 (X5R)
 * Capacitors: 10uF x 2012 (X5R)
 * Pinhead 2X10
 * LDO: RT9193 (1028-1014-1-ND)
 * Crystal resonator: 887-1263-1-ND ( TXC CORPORATION 9C-8.000MEEJ-T) (CL = 18pF)
 * Crystal resonator (through-hole): 887-1233-ND (TXC CORPORATION 9B-8.000MEEJ-B) (CL = 18pF)
 * CL1, CL2 = 16pF-26pF (high quality!)
 * Schmitt trigger: 74LCX14MTC


 * TODO:
 * Power supply...
 * Check all pull-ups/pull-downs
 * Sensor: try 	AH3391Q as well (27.5mT Trip, 25mT Release)
 * Sensor: try 	AH3390Q as well (21mT Trip, 18.5mT Release)


BOM:

Capacitors:
 0.1uF      1608            15              490-1524-1-ND
 1uF        1608            1               490-5307-1-ND
 10uF       2012            3               490-5523-1-ND
 0.022uF    1608            1               490-1517-1-ND
 24pF       1608            2               490-12537-1-ND
 18pF       1608            2               490-11449-1-ND
 22pF       1608            2               490-11451-1-ND

Resistors:
 10K        1608            10              311-10.0KHRCT-ND
 5.6K       1608            10              311-5.60KHRCT-ND
 0          1608            10              311-0.0GRCT-ND
 220        1608            2               311-220HRCT-ND
 510        1608            2               311-510HRCT-ND

Other:
 8Mhz Quartz                1               887-1233-ND
 STM32F103C8T6              1               497-6063-ND
 STM32F103CBT6              1               497-6288-ND
 10K adj pot                1               A105870-ND
 RT9193                     1               1028-1014-1-ND
 CONNECTOR (HALL)           1               A35068-ND
 Schmitt Trigger            1               74LCX14MTC-ND
 LED (green)                2               L62505CT-ND


 CPC1106NTR <- solid state relay?