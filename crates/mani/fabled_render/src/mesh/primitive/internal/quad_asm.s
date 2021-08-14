# --------------- Quad Dissassembly -------------------
00007FF6259212DF  movaps       xmm0, xmmword ptr [__xmm@bf00000000000000bf0000003f000000 (00007FF62593C510h)]
00007FF6259212E6  movaps       xmmword ptr [rsp+40h], xmm0
00007FF6259212EB  movaps       xmm0, xmmword ptr [__xmm@bf000000bf000000000000003f000000 (00007FF62593C520h)]
00007FF6259212F2  movaps       xmmword ptr [rsp+50h], xmm0
00007FF6259212F7  movaps       xmm0, xmmword ptr [__xmm@000000003f0000003f00000000000000 (00007FF62593C530h)]
00007FF6259212FE  movaps       xmmword ptr [rsp+60h], xmm0
00007FF625921303  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000c3fa000043fa0000 (00007FF62593C540h)]
00007FF62592130A  movaps       xmmword ptr [rsp+70h], xmm0
00007FF62592130F  mov          dword ptr [rsp+80h], 0h
00007FF62592131A  mov          eax, dword ptr [00007FF62593DAD8h]
00007FF625921320  mov          dword ptr [rsp+8Ch], eax
00007FF625921327  mov          rcx, qword ptr [00007FF62593DAD0h]
00007FF62592132E  mov          qword ptr [rsp+84h], rcx
00007FF625921336  movaps       xmm0, xmmword ptr [00007FF62593DA90h]
00007FF62592133D  movaps       xmmword ptr [rsp+90h], xmm0
00007FF625921345  movaps       xmm1, xmmword ptr [00007FF62593DAE0h]
00007FF62592134C  movaps       xmmword ptr [rsp+A0h], xmm1
00007FF625921354  movss        xmm4, dword ptr [rsp+4Ch]
00007FF62592135A  movss        xmm5, dword ptr [rsp+50h]
00007FF625921360  movss        xmm2, dword ptr [__real@3f000000 (00007FF62593C550h)]
00007FF625921368  movaps       xmm6, xmm5
00007FF62592136B  addss        xmm6, xmm2
00007FF62592136F  movaps       xmm7, xmm4
00007FF625921372  addss        xmm7, xmm2
00007FF625921376  movss        xmm3, dword ptr [__real@447a0000 (00007FF62593C554h)]
00007FF62592137E  mulss        xmm5, xmm3
00007FF625921382  mulss        xmm4, xmm3
00007FF625921386  movss        dword ptr [rsp+B0h], xmm4
00007FF62592138F  movss        dword ptr [rsp+B4h], xmm5
00007FF625921398  mov          dword ptr [rsp+B8h], 0h
00007FF6259213A3  movss        dword ptr [rsp+BCh], xmm7
00007FF6259213AC  movss        dword ptr [rsp+C0h], xmm6
00007FF6259213B5  mov          dword ptr [rsp+CCh], eax
00007FF6259213BC  mov          qword ptr [rsp+C4h], rcx
00007FF6259213C4  movaps       xmmword ptr [rsp+D0h], xmm0
00007FF6259213CC  movaps       xmmword ptr [rsp+E0h], xmm1
00007FF6259213D4  movss        xmm4, dword ptr [rsp+58h]
00007FF6259213DA  movss        xmm5, dword ptr [rsp+5Ch]
00007FF6259213E0  movaps       xmm6, xmm5
00007FF6259213E3  addss        xmm6, xmm2
00007FF6259213E7  movaps       xmm7, xmm4
00007FF6259213EA  addss        xmm7, xmm2
00007FF6259213EE  mulss        xmm5, xmm3
00007FF6259213F2  mulss        xmm4, xmm3
00007FF6259213F6  movss        dword ptr [rsp+F0h], xmm4
00007FF6259213FF  movss        dword ptr [rsp+F4h], xmm5
00007FF625921408  mov          dword ptr [rsp+F8h], 0h
00007FF625921413  movss        dword ptr [rsp+FCh], xmm7
00007FF62592141C  movss        dword ptr [rsp+100h], xmm6
00007FF625921425  mov          dword ptr [rsp+10Ch], eax
00007FF62592142C  mov          qword ptr [rsp+104h], rcx
00007FF625921434  movaps       xmmword ptr [rsp+110h], xmm0
00007FF62592143C  movaps       xmmword ptr [rsp+120h], xmm1
00007FF625921444  movss        xmm4, dword ptr [rsp+64h]
00007FF62592144A  movss        xmm5, dword ptr [rsp+68h]
00007FF625921450  movaps       xmm6, xmm5
00007FF625921453  addss        xmm6, xmm2
00007FF625921457  addss        xmm2, xmm4
00007FF62592145B  mulss        xmm5, xmm3
00007FF62592145F  mulss        xmm4, xmm3
00007FF625921463  movss        dword ptr [rsp+130h], xmm4
00007FF62592146C  movss        dword ptr [rsp+134h], xmm5
00007FF625921475  mov          dword ptr [rsp+138h], 0h
00007FF625921480  movss        dword ptr [rsp+13Ch], xmm2
00007FF625921489  movss        dword ptr [rsp+140h], xmm6
00007FF625921492  mov          dword ptr [rsp+14Ch], eax
00007FF625921499  mov          qword ptr [rsp+144h], rcx
00007FF6259214A1  movaps       xmmword ptr [rsp+150h], xmm0
00007FF6259214A9  movaps       xmmword ptr [rsp+160h], xmm1
00007FF6259214B1  mov          rcx, qword ptr [00007FF6259431C8h]
00007FF6259214B8  test         rcx, rcx
00007FF6259214BB  jne          static void Fabled_Engine::main+2C5h (00007FF6259214D5h)
00007FF6259214BD  call         GetProcessHeap (00007FF625938E4Ch)
00007FF6259214C2  test         rax, rax
00007FF6259214C5  je           static void Fabled_Engine::main+613h (00007FF625921823h)
00007FF6259214CB  mov          rcx, rax
00007FF6259214CE  mov          qword ptr [00007FF6259431C8h], rax
00007FF6259214D5  mov          r8d, 100h
00007FF6259214DB  xor          edx, edx
00007FF6259214DD  call         HeapAlloc (00007FF625938E52h)
00007FF6259214E2  test         rax, rax
00007FF6259214E5  je           static void Fabled_Engine::main+613h (00007FF625921823h)
00007FF6259214EB  mov          rdi, rax
00007FF6259214EE  lea          rdx, [rsp+70h]
00007FF6259214F3  mov          r8d, 100h
00007FF6259214F9  mov          rcx, rax
00007FF6259214FC  call         memcpy (00007FF625939F37h)
00007FF625921501  mov          rcx, qword ptr [00007FF6259431C8h]
00007FF625921508  test         rcx, rcx
00007FF62592150B  jne          static void Fabled_Engine::main+315h (00007FF625921525h)
00007FF62592150D  call         GetProcessHeap (00007FF625938E4Ch)
00007FF625921512  test         rax, rax
00007FF625921515  je           static void Fabled_Engine::main+61Ah (00007FF62592182Ah)
00007FF62592151B  mov          rcx, rax
00007FF62592151E  mov          qword ptr [00007FF6259431C8h], rax
00007FF625921525  mov          r8d, 30h
00007FF62592152B  xor          edx, edx
00007FF62592152D  call         HeapAlloc (00007FF625938E52h)
00007FF625921532  test         rax, rax
00007FF625921535  je           static void Fabled_Engine::main+61Ah (00007FF62592182Ah)
00007FF62592153B  mov          rbx, rax
00007FF62592153E  movups       xmm0, xmmword ptr [00007FF62593DAC0h]
00007FF625921545  movups       xmmword ptr [rax+20h], xmm0
00007FF625921549  movups       xmm0, xmmword ptr [00007FF62593DAB0h]
00007FF625921550  movups       xmmword ptr [rax+10h], xmm0
00007FF625921554  movups       xmm0, xmmword ptr [00007FF62593DAA0h]
00007FF62592155B  movups       xmmword ptr [rax], xmm0
00007FF62592155E  mov          rcx, qword ptr [00007FF6259431C8h]
00007FF625921565  test         rcx, rcx
00007FF625921568  jne          static void Fabled_Engine::main+372h (00007FF625921582h)
00007FF62592156A  call         GetProcessHeap (00007FF625938E4Ch)
00007FF62592156F  test         rax, rax
00007FF625921572  je           static void Fabled_Engine::main+62Bh (00007FF62592183Bh)
00007FF625921578  mov          rcx, rax
00007FF62592157B  mov          qword ptr [00007FF6259431C8h], rax
00007FF625921582  mov          r8d, 40h
00007FF625921588  xor          edx, edx
00007FF62592158A  call         HeapAlloc (00007FF625938E52h)
00007FF62592158F  test         rax, rax
00007FF625921592  je           static void Fabled_Engine::main+62Bh (00007FF62592183Bh)
00007FF625921598  mov          rsi, rax
00007FF62592159B  mov          qword ptr [rax], rdi
00007FF62592159E  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF62593C560h)]
00007FF6259215A5  movups       xmmword ptr [rax+8h], xmm0
00007FF6259215A9  add          rax, 18h
00007FF6259215AD  mov          dword ptr [rsi+18h], 0h
00007FF6259215B4  mov          qword ptr [rsi+20h], rbx
00007FF6259215B8  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF62593C570h)]
00007FF6259215BF  movups       xmmword ptr [rsi+28h], xmm0
