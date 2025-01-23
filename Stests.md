running 4 tests
[DEBUG give_up_and_restart] [handle(0xc149..ab32)] 0x0100..0000: Game initialization: PebblesInit { difficulty: Easy, pebbles_count: 15, max_pebbles_per_turn: 2 }
[DEBUG give_up_and_restart] [handle(0xc97d..c302)] 0x0100..0000: Received action: GiveUp
[DEBUG difficulty_levels] [handle(0xc149..ab32)] 0x0100..0000: Game initialization: PebblesInit { difficulty: Easy, pebbles_count: 15, max_pebbles_per_turn: 2 }
[DEBUG give_up_and_restart] [handle(0x327f..fd3c)] 0x0100..0000: Received action: Restart { difficulty: Hard, pebbles_count: 20, max_pebbles_per_turn: 3 }
[DEBUG give_up_and_restart] [handle(0x327f..fd3c)] 0x0100..0000: AI removes 1 pebbles
[DEBUG game_flow] [handle(0xc149..ab32)] 0x0100..0000: Game initialization: PebblesInit { difficulty: Easy, pebbles_count: 15, max_pebbles_per_turn: 2 }
test give_up_and_restart ... ok
[DEBUG invalid_inputs] [handle(0xc149..ab32)] 0x0100..0000: Game initialization: PebblesInit { difficulty: Easy, pebbles_count: 0, max_pebbles_per_turn: 0 }
[DEBUG game_flow] [handle(0xc97d..c302)] 0x0100..0000: Received action: Turn(1)
[DEBUG game_flow] [handle(0xc97d..c302)] 0x0100..0000: Human removes 1 pebbles
[DEBUG game_flow] [handle(0xc97d..c302)] 0x0100..0000: AI removes 1 pebbles
[DEBUG difficulty_levels] [handle(0xc97d..c302)] 0x0200..0000: Game initialization: PebblesInit { difficulty: Hard, pebbles_count: 15, max_pebbles_per_turn: 2 }
[DEBUG difficulty_levels] [handle(0xc97d..c302)] 0x0200..0000: AI removes 1 pebbles
[DEBUG invalid_inputs] [handle(0x2b37..584c)] 0x0200..0000: Game initialization: PebblesInit { difficulty: Easy, pebbles_count: 15, max_pebbles_per_turn: 2 }
[DEBUG invalid_inputs] [handle(0x2b37..584c)] 0x0200..0000: AI removes 1 pebbles
test invalid_inputs ... ok
test difficulty_levels ... ok
[DEBUG game_flow] [handle(0x327f..fd3c)] 0x0100..0000: Received action: Turn(2)
[DEBUG game_flow] [handle(0x327f..fd3c)] 0x0100..0000: Human removes 2 pebbles
[DEBUG game_flow] [handle(0x327f..fd3c)] 0x0100..0000: AI removes 1 pebbles
[DEBUG game_flow] [handle(0x249e..d1f4)] 0x0100..0000: Received action: Turn(2)
[DEBUG game_flow] [handle(0x249e..d1f4)] 0x0100..0000: Human removes 2 pebbles
[DEBUG game_flow] [handle(0x249e..d1f4)] 0x0100..0000: AI removes 1 pebbles
[DEBUG game_flow] [handle(0xb3c5..3260)] 0x0100..0000: Received action: Turn(2)
[DEBUG game_flow] [handle(0xb3c5..3260)] 0x0100..0000: Human removes 2 pebbles
[DEBUG game_flow] [handle(0xb3c5..3260)] 0x0100..0000: AI removes 2 pebbles
[DEBUG game_flow] [handle(0x9617..776c)] 0x0100..0000: Received action: Turn(2)
[DEBUG game_flow] [handle(0x9617..776c)] 0x0100..0000: Human removes 2 pebbles
[DEBUG game_flow] [handle(0x9617..776c)] 0x0100..0000: AI removes 1 pebbles
test game_flow ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.65s

   Doc-tests pebbles_game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s