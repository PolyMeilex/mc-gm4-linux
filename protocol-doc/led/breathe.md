# Breathe 
`0090   80 80 80 80 80 80 80 80 80 80 00 00 00 22 00 00`

led_mode = 0x22

led_arg1 = 0

led_arg2 = 0

## Speed:
```c
00a0   12 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
00a0   22 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
00a0   32 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
00a0   42 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
00a0   52 04 00 00 ff 00 00 00 00 ff 00 ff 00 ff 00 ff
```

led_arg3 -> 1,2,3,4,5 (4s,5s,6s,7s,8s) [* 0x10 + 0x02]

```rs
enum BreatheSpeed {
    S4 = 1,
    S5 = 2,
    S6 = 3,
    S7 = 4,
    S8 = 5,
}
```