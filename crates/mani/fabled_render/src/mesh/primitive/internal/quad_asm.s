# --------------- Quad Dissassembly -------------------
00007FF6B67D12FF  mov          rcx, qword ptr [00007FF6B67F21C8h]
00007FF6B67D1306  test         rcx, rcx
00007FF6B67D1309  jne          static void Fabled_Engine::main+63h (00007FF6B67D1323h)
00007FF6B67D130B  call         GetProcessHeap (00007FF6B67E8A52h)
00007FF6B67D1310  test         rax, rax
00007FF6B67D1313  je           static void Fabled_Engine::main+432h (00007FF6B67D16F2h)
00007FF6B67D1319  mov          rcx, rax
00007FF6B67D131C  mov          qword ptr [00007FF6B67F21C8h], rax
00007FF6B67D1323  mov          r8d, 100h
00007FF6B67D1329  xor          edx, edx
00007FF6B67D132B  call         HeapAlloc (00007FF6B67E8A58h)
00007FF6B67D1330  test         rax, rax
00007FF6B67D1333  je           static void Fabled_Engine::main+432h (00007FF6B67D16F2h)
00007FF6B67D1339  mov          rdi, rax
00007FF6B67D133C  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000bf0000003f000000 (00007FF6B67EB520h)]
00007FF6B67D1343  movaps       xmmword ptr [rax], xmm0
00007FF6B67D1346  mov          dword ptr [rax+10h], 0h
00007FF6B67D134D  mov          eax, dword ptr [00007FF6B67ECA88h]
00007FF6B67D1353  mov          dword ptr [rdi+1Ch], eax
00007FF6B67D1356  mov          rcx, qword ptr [00007FF6B67ECA80h]
00007FF6B67D135D  mov          qword ptr [rdi+14h], rcx
00007FF6B67D1361  movaps       xmm0, xmmword ptr [00007FF6B67ECA70h]
00007FF6B67D1368  movaps       xmmword ptr [rdi+20h], xmm0
00007FF6B67D136C  movaps       xmm1, xmmword ptr [00007FF6B67ECA90h]
00007FF6B67D1373  movaps       xmmword ptr [rdi+30h], xmm1
00007FF6B67D1377  movaps       xmm2, xmmword ptr [__xmm@00000000000000003f000000bf000000 (00007FF6B67EB530h)]
00007FF6B67D137E  movaps       xmmword ptr [rdi+40h], xmm2
00007FF6B67D1382  mov          dword ptr [rdi+50h], 3F800000h
00007FF6B67D1389  mov          dword ptr [rdi+5Ch], eax
00007FF6B67D138C  mov          qword ptr [rdi+54h], rcx
00007FF6B67D1390  movaps       xmmword ptr [rdi+60h], xmm0
00007FF6B67D1394  movaps       xmmword ptr [rdi+70h], xmm1
00007FF6B67D1398  movaps       xmm2, xmmword ptr [__xmm@0000000000000000bf000000bf000000 (00007FF6B67EB540h)]
00007FF6B67D139F  movaps       xmmword ptr [rdi+80h], xmm2
00007FF6B67D13A6  mov          dword ptr [rdi+90h], 0h
00007FF6B67D13B0  mov          dword ptr [rdi+9Ch], eax
00007FF6B67D13B6  mov          qword ptr [rdi+94h], rcx
00007FF6B67D13BD  movaps       xmmword ptr [rdi+A0h], xmm0
00007FF6B67D13C4  movaps       xmmword ptr [rdi+B0h], xmm1
00007FF6B67D13CB  movaps       xmm2, xmmword ptr [__xmm@3f800000000000003f0000003f000000 (00007FF6B67EB550h)]
00007FF6B67D13D2  movaps       xmmword ptr [rdi+C0h], xmm2
00007FF6B67D13D9  mov          dword ptr [rdi+D0h], 3F800000h
00007FF6B67D13E3  mov          dword ptr [rdi+DCh], eax
00007FF6B67D13E9  mov          qword ptr [rdi+D4h], rcx
00007FF6B67D13F0  movaps       xmmword ptr [rdi+E0h], xmm0
00007FF6B67D13F7  movaps       xmmword ptr [rdi+F0h], xmm1
00007FF6B67D13FE  mov          rcx, qword ptr [00007FF6B67F21C8h]
00007FF6B67D1405  test         rcx, rcx
00007FF6B67D1408  jne          static void Fabled_Engine::main+162h (00007FF6B67D1422h)
00007FF6B67D140A  call         GetProcessHeap (00007FF6B67E8A52h)
00007FF6B67D140F  test         rax, rax
00007FF6B67D1412  je           static void Fabled_Engine::main+439h (00007FF6B67D16F9h)
00007FF6B67D1418  mov          rcx, rax
00007FF6B67D141B  mov          qword ptr [00007FF6B67F21C8h], rax
00007FF6B67D1422  mov          r8d, 30h
00007FF6B67D1428  xor          edx, edx
00007FF6B67D142A  call         HeapAlloc (00007FF6B67E8A58h)
00007FF6B67D142F  test         rax, rax
00007FF6B67D1432  je           static void Fabled_Engine::main+439h (00007FF6B67D16F9h)
00007FF6B67D1438  mov          rbx, rax
00007FF6B67D143B  movups       xmm0, xmmword ptr [00007FF6B67ECAC0h]
00007FF6B67D1442  movups       xmmword ptr [rax+20h], xmm0
00007FF6B67D1446  movups       xmm0, xmmword ptr [00007FF6B67ECAB0h]
00007FF6B67D144D  movups       xmmword ptr [rax+10h], xmm0
00007FF6B67D1451  movups       xmm0, xmmword ptr [00007FF6B67ECAA0h]
00007FF6B67D1458  movups       xmmword ptr [rax], xmm0
00007FF6B67D145B  mov          rcx, qword ptr [00007FF6B67F21C8h]
00007FF6B67D1462  test         rcx, rcx
00007FF6B67D1465  jne          static void Fabled_Engine::main+1BFh (00007FF6B67D147Fh)
00007FF6B67D1467  call         GetProcessHeap (00007FF6B67E8A52h)
00007FF6B67D146C  test         rax, rax
00007FF6B67D146F  je           static void Fabled_Engine::main+44Ah (00007FF6B67D170Ah)
00007FF6B67D1475  mov          rcx, rax
00007FF6B67D1478  mov          qword ptr [00007FF6B67F21C8h], rax
00007FF6B67D147F  mov          r8d, 40h
00007FF6B67D1485  xor          edx, edx
00007FF6B67D1487  call         HeapAlloc (00007FF6B67E8A58h)
00007FF6B67D148C  test         rax, rax
00007FF6B67D148F  je           static void Fabled_Engine::main+44Ah (00007FF6B67D170Ah)
00007FF6B67D1495  mov          rsi, rax
00007FF6B67D1498  mov          qword ptr [rax], rdi
00007FF6B67D149B  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF6B67EB510h)]
00007FF6B67D14A2  movups       xmmword ptr [rax+8h], xmm0
00007FF6B67D14A6  mov          dword ptr [rax+18h], 0h
00007FF6B67D14AD  mov          qword ptr [rax+20h], rbx
00007FF6B67D14B1  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF6B67EB560h)]
00007FF6B67D14B8  movups       xmmword ptr [rax+28h], xmm0
00007FF6B67D14BC  lea          rax, [00007FF6B67EEB10h]
00007FF6B67D14C3  mov          qword ptr [rsp+90h], rax
00007FF6B67D14CB  mov          qword ptr [rsp+98h], 6h