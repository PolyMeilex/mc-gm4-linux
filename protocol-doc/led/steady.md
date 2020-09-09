# Steady 
led_mode = 0x28

## Brigtness
```c
// 1 (5%)
00a0   12 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
// 2 (15%)
00a0   22 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
// 3 (25%)
00a0   32 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
// 9 (85%);
00a0   92 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
// 10 (100%)
00a0   a2 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
```

led_arg1 -> 0

led_arg2 -> 0

led_arg3 -> 1,2,3,4,5,6,7,8,9,10 [* 0x10 + 0x02]

```rs
enum SteadyBrightnes {
    P5 = 1,
    P15 = 2,
    P25 = 3,
    P35 = 4,
    P45 = 5,
    P55 = 6,
    P65 = 7,
    P75 = 8,
    P85 = 9,
    P100 = 10,
}
```