# --------------- Quad Dissassembly -------------------
00007FF7A4B6125F  mov          rcx, qword ptr [00007FF7A4B831C8h]
00007FF7A4B61266  test         rcx, rcx
00007FF7A4B61269  jne          static void Fabled_Engine::main+73h (00007FF7A4B61283h)
00007FF7A4B6126B  call         GetProcessHeap (00007FF7A4B78E9Ch)
00007FF7A4B61270  test         rax, rax
00007FF7A4B61273  je           static void Fabled_Engine::main+4D4h (00007FF7A4B616E4h)
00007FF7A4B61279  mov          rcx, rax
00007FF7A4B6127C  mov          qword ptr [00007FF7A4B831C8h], rax
00007FF7A4B61283  mov          r8d, 100h
00007FF7A4B61289  xor          edx, edx
00007FF7A4B6128B  call         HeapAlloc (00007FF7A4B78EA2h)
00007FF7A4B61290  test         rax, rax
00007FF7A4B61293  je           static void Fabled_Engine::main+4D4h (00007FF7A4B616E4h)
00007FF7A4B61299  mov          qword ptr [rsp+30h], rax
00007FF7A4B6129E  mov          qword ptr [rsp+38h], 4h
00007FF7A4B612A7  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000c3fa000043fa0000 (00007FF7A4B7C510h)]
00007FF7A4B612AE  movaps       xmmword ptr [rax], xmm0
00007FF7A4B612B1  mov          dword ptr [rax+10h], 0h
00007FF7A4B612B8  mov          esi, dword ptr [00007FF7A4B7DA98h]
00007FF7A4B612BE  mov          dword ptr [rax+1Ch], esi
00007FF7A4B612C1  mov          rdi, qword ptr [00007FF7A4B7DA90h]
00007FF7A4B612C8  mov          qword ptr [rax+14h], rdi
00007FF7A4B612CC  movaps       xmm6, xmmword ptr [00007FF7A4B7DA80h]
00007FF7A4B612D3  movaps       xmmword ptr [rax+20h], xmm6
00007FF7A4B612D7  movaps       xmm7, xmmword ptr [00007FF7A4B7DAA0h]
00007FF7A4B612DE  movaps       xmmword ptr [rax+30h], xmm7
00007FF7A4B612E2  mov          qword ptr [rsp+40h], 1h
00007FF7A4B612EB  movaps       xmm0, xmmword ptr [__xmm@000000000000000043fa0000c3fa0000 (00007FF7A4B7C520h)]
00007FF7A4B612F2  movaps       xmmword ptr [rax+40h], xmm0
00007FF7A4B612F6  mov          dword ptr [rax+50h], 3F800000h
00007FF7A4B612FD  mov          dword ptr [rax+5Ch], esi
00007FF7A4B61300  mov          qword ptr [rax+54h], rdi
00007FF7A4B61304  movaps       xmmword ptr [rax+60h], xmm6
00007FF7A4B61308  movaps       xmmword ptr [rax+70h], xmm7
00007FF7A4B6130C  mov          qword ptr [rsp+40h], 2h
00007FF7A4B61315  movaps       xmm0, xmmword ptr [__xmm@0000000000000000c3fa0000c3fa0000 (00007FF7A4B7C530h)]
00007FF7A4B6131C  movaps       xmmword ptr [rax+80h], xmm0
00007FF7A4B61323  mov          dword ptr [rax+90h], 0h
00007FF7A4B6132D  mov          dword ptr [rax+9Ch], esi
00007FF7A4B61333  mov          qword ptr [rax+94h], rdi
00007FF7A4B6133A  movaps       xmmword ptr [rax+A0h], xmm6
00007FF7A4B61341  movaps       xmmword ptr [rax+B0h], xmm7
00007FF7A4B61348  mov          rdx, qword ptr [rsp+40h]
00007FF7A4B6134D  add          rdx, 1h
00007FF7A4B61351  mov          qword ptr [rsp+40h], rdx
00007FF7A4B61356  cmp          rdx, qword ptr [rsp+38h]
00007FF7A4B6135B  je           static void Fabled_Engine::main+48Dh (00007FF7A4B6169Dh)
00007FF7A4B61361  mov          rax, qword ptr [rsp+30h]
00007FF7A4B61366  mov          rcx, rdx
00007FF7A4B61369  shl          rcx, 6h
00007FF7A4B6136D  movaps       xmm0, xmmword ptr [__xmm@3f8000000000000043fa000043fa0000 (00007FF7A4B7C540h)]
00007FF7A4B61374  movaps       xmmword ptr [rax+rcx*1], xmm0
00007FF7A4B61378  mov          dword ptr [rax+rcx*1+10h], 3F800000h
00007FF7A4B61380  mov          dword ptr [rax+rcx*1+1Ch], esi
00007FF7A4B61384  mov          qword ptr [rax+rcx*1+14h], rdi
00007FF7A4B61389  movaps       xmmword ptr [rax+rcx*1+20h], xmm6
00007FF7A4B6138E  movaps       xmmword ptr [rax+rcx*1+30h], xmm7
00007FF7A4B61393  add          rdx, 1h
00007FF7A4B61397  mov          qword ptr [rsp+40h], rdx
00007FF7A4B6139C  mov          qword ptr [rsp+70h], rdx
00007FF7A4B613A1  movups       xmm0, xmmword ptr [rsp+30h]
00007FF7A4B613A6  movaps       xmmword ptr [rsp+60h], xmm0
00007FF7A4B613AB  mov          rcx, qword ptr [00007FF7A4B831C8h]
00007FF7A4B613B2  test         rcx, rcx
00007FF7A4B613B5  jne          static void Fabled_Engine::main+1BFh (00007FF7A4B613CFh)
00007FF7A4B613B7  call         GetProcessHeap (00007FF7A4B78E9Ch)
00007FF7A4B613BC  test         rax, rax
00007FF7A4B613BF  je           static void Fabled_Engine::main+4DBh (00007FF7A4B616EBh)
00007FF7A4B613C5  mov          rcx, rax
00007FF7A4B613C8  mov          qword ptr [00007FF7A4B831C8h], rax
00007FF7A4B613CF  mov          r8d, 30h
00007FF7A4B613D5  xor          edx, edx
00007FF7A4B613D7  call         HeapAlloc (00007FF7A4B78EA2h)
00007FF7A4B613DC  test         rax, rax
00007FF7A4B613DF  je           static void Fabled_Engine::main+4DBh (00007FF7A4B616EBh)
00007FF7A4B613E5  mov          rdi, rax
00007FF7A4B613E8  movups       xmm0, xmmword ptr [00007FF7A4B7DAD0h]
00007FF7A4B613EF  movups       xmmword ptr [rax+20h], xmm0
00007FF7A4B613F3  movups       xmm0, xmmword ptr [00007FF7A4B7DAC0h]
00007FF7A4B613FA  movups       xmmword ptr [rax+10h], xmm0
00007FF7A4B613FE  movups       xmm0, xmmword ptr [00007FF7A4B7DAB0h]
00007FF7A4B61405  movups       xmmword ptr [rax], xmm0
00007FF7A4B61408  mov          rcx, qword ptr [00007FF7A4B831C8h]
00007FF7A4B6140F  test         rcx, rcx
00007FF7A4B61412  jne          static void Fabled_Engine::main+21Ch (00007FF7A4B6142Ch)
00007FF7A4B61414  call         GetProcessHeap (00007FF7A4B78E9Ch)
00007FF7A4B61419  test         rax, rax
00007FF7A4B6141C  je           static void Fabled_Engine::main+4ECh (00007FF7A4B616FCh)
00007FF7A4B61422  mov          rcx, rax
00007FF7A4B61425  mov          qword ptr [00007FF7A4B831C8h], rax
00007FF7A4B6142C  mov          r8d, 40h
00007FF7A4B61432  xor          edx, edx
00007FF7A4B61434  call         HeapAlloc (00007FF7A4B78EA2h)
00007FF7A4B61439  test         rax, rax
00007FF7A4B6143C  je           static void Fabled_Engine::main+4ECh (00007FF7A4B616FCh)
00007FF7A4B61442  mov          rsi, rax
00007FF7A4B61445  mov          rax, qword ptr [rsp+70h]
00007FF7A4B6144A  mov          qword ptr [rsi+10h], rax
00007FF7A4B6144E  movaps       xmm0, xmmword ptr [rsp+60h]
00007FF7A4B61453  movaps       xmmword ptr [rsi], xmm0
00007FF7A4B61456  mov          rax, rsi
00007FF7A4B61459  add          rax, 18h
00007FF7A4B6145D  mov          dword ptr [rsi+18h], 0h
00007FF7A4B61464  mov          qword ptr [rsi+20h], rdi
00007FF7A4B61468  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF7A4B7C550h)]
00007FF7A4B6146F  movups       xmmword ptr [rsi+28h], xmm0
