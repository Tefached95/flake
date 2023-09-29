0x200: LD I, $21E ;; Load address $21E into the special register I
0x202: RND V2, 01 ;; Set V2 to a random number with a mask of 010x204: SE V2, 01 ;; Skip the next instruction if the value in register V2 is equal to 01
0x206: LD I, $21A ;; Load address $21A into the special register I
0x208: DRW V0, V1, 04 ;; Draw a sprite at position V0, V1 with V4 bytes of sprite data starting at the address stored in I. Set VF to 01 if any set pixels are changed to unset, and 00 otherwise0x20A: ADD V0, 04 ;; Add 04 to the value in register V0
0x20C: SE V0, 40 ;; Skip the next instruction if the value in register V0 is equal to 40
0x20E: JP $200 ;; Jumps to address $200
0x210: LD V0, 00 ;; Set the value of register V0 to 00
0x212: ADD V1, 04 ;; Add 04 to the value in register V1
0x214: SE V1, 20 ;; Skip the next instruction if the value in register V1 is equal to 20
0x216: JP $200 ;; Jumps to address $200
0x218: JP $218 ;; Jumps to address $218
0x21A: LD V0, V4 ;; Store the value of register V0 to the value of register V4
0x21C: CALL $010 ;; Jumps to subroutine at $010
0x21E: CALL $040 ;; Jumps to subroutine at $040
0x220: LD V0, V1 ;; Store the value of register V0 to the value of register V1
