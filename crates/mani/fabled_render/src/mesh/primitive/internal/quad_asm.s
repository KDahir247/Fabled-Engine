# --------------- Quad Dissassembly -------------------
00007FF7E1B8137F  movaps       xmm1, xmmword ptr [__xmm@bf00000000000000bf000000bf000000 (00007FF7E1B9C520h)]
00007FF7E1B81386  movaps       xmmword ptr [rsp+40h], xmm1
00007FF7E1B8138B  movaps       xmm1, xmmword ptr [__xmm@3f0000003f000000000000003f000000 (00007FF7E1B9C530h)]
00007FF7E1B81392  movaps       xmmword ptr [rsp+50h], xmm1
00007FF7E1B81397  movaps       xmm1, xmmword ptr [__xmm@00000000bf0000003f00000000000000 (00007FF7E1B9C540h)]
00007FF7E1B8139E  movaps       xmmword ptr [rsp+60h], xmm1
00007FF7E1B813A3  movss        xmm2, dword ptr [rsp+44h]
00007FF7E1B813A9  movss        xmm3, dword ptr [rsp+4Ch]
00007FF7E1B813AF  movss        xmm1, dword ptr [__real@3f000000 (00007FF7E1B9C550h)]
00007FF7E1B813B7  mov          dword ptr [rsp+70h], BF000000h
00007FF7E1B813BF  movss        dword ptr [rsp+74h], xmm2
00007FF7E1B813C5  addss        xmm2, xmm1
00007FF7E1B813C9  mov          qword ptr [rsp+78h], 0h
00007FF7E1B813D2  movss        dword ptr [rsp+80h], xmm2
00007FF7E1B813DB  mov          eax, dword ptr [00007FF7E1B9DAA8h]
00007FF7E1B813E1  mov          dword ptr [rsp+8Ch], eax
00007FF7E1B813E8  mov          rcx, qword ptr [00007FF7E1B9DAA0h]
00007FF7E1B813EF  mov          qword ptr [rsp+84h], rcx
00007FF7E1B813F7  movaps       xmmword ptr [rsp+A0h], xmm0
00007FF7E1B813FF  movaps       xmmword ptr [rsp+90h], xmm0
00007FF7E1B81407  movss        xmm2, dword ptr [rsp+50h]
00007FF7E1B8140D  movss        dword ptr [rsp+B0h], xmm3
00007FF7E1B81416  addss        xmm3, xmm1
00007FF7E1B8141A  movss        dword ptr [rsp+B4h], xmm2
00007FF7E1B81423  addss        xmm2, xmm1
00007FF7E1B81427  mov          dword ptr [rsp+B8h], 0h
00007FF7E1B81432  movss        dword ptr [rsp+BCh], xmm3
00007FF7E1B8143B  movss        dword ptr [rsp+C0h], xmm2
00007FF7E1B81444  mov          dword ptr [rsp+CCh], eax
00007FF7E1B8144B  mov          qword ptr [rsp+C4h], rcx
00007FF7E1B81453  movaps       xmmword ptr [rsp+E0h], xmm0
00007FF7E1B8145B  movaps       xmmword ptr [rsp+D0h], xmm0
00007FF7E1B81463  movss        xmm2, dword ptr [rsp+58h]
00007FF7E1B81469  movss        xmm3, dword ptr [rsp+5Ch]
00007FF7E1B8146F  movss        dword ptr [rsp+F0h], xmm2
00007FF7E1B81478  addss        xmm2, xmm1
00007FF7E1B8147C  movss        dword ptr [rsp+F4h], xmm3
00007FF7E1B81485  addss        xmm3, xmm1
00007FF7E1B81489  mov          dword ptr [rsp+F8h], 0h
00007FF7E1B81494  movss        dword ptr [rsp+FCh], xmm2
00007FF7E1B8149D  movss        dword ptr [rsp+100h], xmm3
00007FF7E1B814A6  mov          dword ptr [rsp+10Ch], eax
00007FF7E1B814AD  mov          qword ptr [rsp+104h], rcx
00007FF7E1B814B5  movaps       xmmword ptr [rsp+120h], xmm0
00007FF7E1B814BD  movaps       xmmword ptr [rsp+110h], xmm0
00007FF7E1B814C5  movss        xmm2, dword ptr [rsp+64h]
00007FF7E1B814CB  movss        xmm3, dword ptr [rsp+68h]
00007FF7E1B814D1  movss        dword ptr [rsp+130h], xmm2
00007FF7E1B814DA  movss        dword ptr [rsp+134h], xmm3
00007FF7E1B814E3  addss        xmm3, xmm1
00007FF7E1B814E7  addss        xmm1, xmm2
00007FF7E1B814EB  mov          dword ptr [rsp+138h], 0h
00007FF7E1B814F6  movss        dword ptr [rsp+13Ch], xmm1
00007FF7E1B814FF  movss        dword ptr [rsp+140h], xmm3
00007FF7E1B81508  mov          dword ptr [rsp+14Ch], eax
00007FF7E1B8150F  mov          qword ptr [rsp+144h], rcx
00007FF7E1B81517  movaps       xmmword ptr [rsp+160h], xmm0
00007FF7E1B8151F  movaps       xmmword ptr [rsp+150h], xmm0
00007FF7E1B81527  mov          rcx, qword ptr [00007FF7E1BA31C8h]
00007FF7E1B8152E  test         rcx, rcx
00007FF7E1B81531  jne          static void Fabled_Engine::main+28Bh (00007FF7E1B8154Bh)
00007FF7E1B81533  call         GetProcessHeap (00007FF7E1B98BD2h)
00007FF7E1B81538  test         rax, rax
00007FF7E1B8153B  je           static void Fabled_Engine::main+5AEh (00007FF7E1B8186Eh)
00007FF7E1B81541  mov          rcx, rax
00007FF7E1B81544  mov          qword ptr [00007FF7E1BA31C8h], rax
00007FF7E1B8154B  mov          r8d, 30h
00007FF7E1B81551  xor          edx, edx
00007FF7E1B81553  call         HeapAlloc (00007FF7E1B98BD8h)
00007FF7E1B81558  test         rax, rax
00007FF7E1B8155B  je           static void Fabled_Engine::main+5AEh (00007FF7E1B8186Eh)
00007FF7E1B81561  mov          rdi, rax
00007FF7E1B81564  movups       xmm0, xmmword ptr [00007FF7E1B9DA90h]
00007FF7E1B8156B  movups       xmmword ptr [rax+20h], xmm0
00007FF7E1B8156F  movups       xmm0, xmmword ptr [00007FF7E1B9DA80h]
00007FF7E1B81576  movups       xmmword ptr [rax+10h], xmm0
00007FF7E1B8157A  movups       xmm0, xmmword ptr [00007FF7E1B9DA70h]
00007FF7E1B81581  movups       xmmword ptr [rax], xmm0
00007FF7E1B81584  mov          rcx, qword ptr [00007FF7E1BA31C8h]
00007FF7E1B8158B  test         rcx, rcx
00007FF7E1B8158E  jne          static void Fabled_Engine::main+2E8h (00007FF7E1B815A8h)
00007FF7E1B81590  call         GetProcessHeap (00007FF7E1B98BD2h)
00007FF7E1B81595  test         rax, rax
00007FF7E1B81598  je           static void Fabled_Engine::main+5BFh (00007FF7E1B8187Fh)
00007FF7E1B8159E  mov          rcx, rax
00007FF7E1B815A1  mov          qword ptr [00007FF7E1BA31C8h], rax
00007FF7E1B815A8  mov          r8d, 100h
00007FF7E1B815AE  xor          edx, edx
00007FF7E1B815B0  call         HeapAlloc (00007FF7E1B98BD8h)
00007FF7E1B815B5  test         rax, rax
00007FF7E1B815B8  je           static void Fabled_Engine::main+5BFh (00007FF7E1B8187Fh)
00007FF7E1B815BE  mov          rbx, rax
00007FF7E1B815C1  lea          rdx, [rsp+70h]
00007FF7E1B815C6  mov          r8d, 100h
00007FF7E1B815CC  mov          rcx, rax
00007FF7E1B815CF  call         memcpy (00007FF7E1B99CB7h)
00007FF7E1B815D4  mov          rcx, qword ptr [00007FF7E1BA31C8h]
00007FF7E1B815DB  test         rcx, rcx
00007FF7E1B815DE  jne          static void Fabled_Engine::main+338h (00007FF7E1B815F8h)
00007FF7E1B815E0  call         GetProcessHeap (00007FF7E1B98BD2h)
00007FF7E1B815E5  test         rax, rax
00007FF7E1B815E8  je           static void Fabled_Engine::main+5C6h (00007FF7E1B81886h)
00007FF7E1B815EE  mov          rcx, rax
00007FF7E1B815F1  mov          qword ptr [00007FF7E1BA31C8h], rax
00007FF7E1B815F8  mov          r8d, 40h
00007FF7E1B815FE  xor          edx, edx
00007FF7E1B81600  call         HeapAlloc (00007FF7E1B98BD8h)
00007FF7E1B81605  test         rax, rax
00007FF7E1B81608  je           static void Fabled_Engine::main+5C6h (00007FF7E1B81886h)
00007FF7E1B8160E  mov          rsi, rax
00007FF7E1B81611  mov          qword ptr [rax+20h], rdi
00007FF7E1B81615  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF7E1B9C560h)]
00007FF7E1B8161C  movups       xmmword ptr [rax+28h], xmm0
00007FF7E1B81620  mov          qword ptr [rax], rbx
00007FF7E1B81623  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF7E1B9C510h)]
00007FF7E1B8162A  movups       xmmword ptr [rax+8h], xmm0
00007FF7E1B8162E  mov          dword ptr [rax+18h], 0h
00007FF7E1B81635  lea          rax, [00007FF7E1B9FAF0h]
00007FF7E1B8163C  mov          qword ptr [rsp+170h], rax
00007FF7E1B81644  mov          qword ptr [rsp+178h], 6h