# --------------- Cone Dissassembly -------------------
00007FF716BF1357  mov          rcx, qword ptr [00007FF716C131C8h]
00007FF716BF135E  test         rcx, rcx
00007FF716BF1361  jne          static void Fabled_Engine::main+BBh (00007FF716BF137Bh)
00007FF716BF1363  call         GetProcessHeap (00007FF716C08FD2h)
00007FF716BF1368  test         rax, rax
00007FF716BF136B  je           static void Fabled_Engine::main+806h (00007FF716BF1AC6h)
00007FF716BF1371  mov          rcx, rax
00007FF716BF1374  mov          qword ptr [00007FF716C131C8h], rax
00007FF716BF137B  mov          r8d, 300h
00007FF716BF1381  xor          edx, edx
00007FF716BF1383  call         HeapAlloc (00007FF716C08FD8h)
00007FF716BF1388  test         rax, rax
00007FF716BF138B  je           static void Fabled_Engine::main+806h (00007FF716BF1AC6h)
00007FF716BF1391  mov          qword ptr [rsp+28h], rax
00007FF716BF1396  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000060 (00007FF716C0C530h)]
00007FF716BF139D  movups       xmmword ptr [rsp+30h], xmm0
00007FF716BF13A2  mov          rcx, qword ptr [00007FF716C131C8h]
00007FF716BF13A9  test         rcx, rcx
00007FF716BF13AC  jne          static void Fabled_Engine::main+106h (00007FF716BF13C6h)
00007FF716BF13AE  call         GetProcessHeap (00007FF716C08FD2h)
00007FF716BF13B3  test         rax, rax
00007FF716BF13B6  je           static void Fabled_Engine::main+817h (00007FF716BF1AD7h)
00007FF716BF13BC  mov          rcx, rax
00007FF716BF13BF  mov          qword ptr [00007FF716C131C8h], rax
00007FF716BF13C6  mov          r8d, 480h
00007FF716BF13CC  xor          edx, edx
00007FF716BF13CE  call         HeapAlloc (00007FF716C08FD8h)
00007FF716BF13D3  test         rax, rax
00007FF716BF13D6  je           static void Fabled_Engine::main+817h (00007FF716BF1AD7h)
00007FF716BF13DC  mov          qword ptr [rsp+40h], rax
00007FF716BF13E1  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000012 (00007FF716C0C540h)]
00007FF716BF13E8  movups       xmmword ptr [rsp+48h], xmm0
00007FF716BF13ED  mov          rcx, 4000000000000000h
00007FF716BF13F7  mov          qword ptr [rax], rcx
00007FF716BF13FA  mov          dword ptr [rax+8h], 0h
00007FF716BF1401  movaps       xmm0, xmmword ptr [__xmm@3f800000000000003f80000000000000 (00007FF716C0C550h)]
00007FF716BF1408  movups       xmmword ptr [rax+Ch], xmm0
00007FF716BF140C  movaps       xmm0, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF716C0C560h)]
00007FF716BF1413  movups       xmmword ptr [rax+1Ch], xmm0
00007FF716BF1417  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000000000003f800000 (00007FF716C0C570h)]
00007FF716BF141E  movups       xmmword ptr [rax+2Ch], xmm0
00007FF716BF1422  mov          dword ptr [rax+3Ch], 3F800000h
00007FF716BF1429  mov          qword ptr [rsp+50h], 1h
00007FF716BF1432  mov          rbx, qword ptr [rsp+40h]
00007FF716BF1437  xorps        xmm8, xmm8
00007FF716BF143B  movaps       xmmword ptr [rbx+40h], xmm8
00007FF716BF1440  movaps       xmm0, xmmword ptr [__xmm@00000000bf80000000000000bf800000 (00007FF716C0C580h)]
00007FF716BF1447  movaps       xmmword ptr [rbx+50h], xmm0
00007FF716BF144B  movaps       xmm0, xmmword ptr [__xmm@3f8000000000000000000000bf800000 (00007FF716C0C590h)]
00007FF716BF1452  movaps       xmmword ptr [rbx+60h], xmm0
00007FF716BF1456  movaps       xmm0, xmmword ptr [__xmm@3f800000bf8000000000000000000000 (00007FF716C0C5A0h)]
00007FF716BF145D  movaps       xmmword ptr [rbx+70h], xmm0
00007FF716BF1461  mov          qword ptr [rsp+50h], 2h
00007FF716BF146A  mov          edi, 2h
00007FF716BF146F  xor          ebp, ebp
00007FF716BF1471  movss        xmm10, dword ptr [__real@3f800000 (00007FF716C0C5B4h)]
00007FF716BF147A  movaps       xmm11, xmmword ptr [__xmm@3f8000003f8000000000000000000000 (00007FF716C0C5C0h)]
00007FF716BF1482  movaps       xmm12, xmmword ptr [__xmm@40a0000040a0000040a0000040a00000 (00007FF716C0C5D0h)]
00007FF716BF148A  movaps       xmm13, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF716C0C5E0h)]
00007FF716BF1492  movaps       xmm14, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF716C0C5F0h)]
00007FF716BF149A  movss        xmm15, dword ptr [__real@3d800000 (00007FF716C0C600h)]
00007FF716BF14A3  lea          r14, [rsp+40h]
00007FF716BF14A8  nop          dword ptr [rax+rax*1], eax
00007FF716BF14B0  test         rbp, rbp
00007FF716BF14B3  js           static void Fabled_Engine::main+200h (00007FF716BF14C0h)
00007FF716BF14B5  xorps        xmm9, xmm9
00007FF716BF14B9  cvtsi2ss     xmm9, rbp
00007FF716BF14BE  jmp          static void Fabled_Engine::main+21Ch (00007FF716BF14DCh)
00007FF716BF14C0  mov          rax, rbp
00007FF716BF14C3  shr          rax, 1h
00007FF716BF14C6  mov          ecx, ebp
00007FF716BF14C8  and          ecx, 1h
00007FF716BF14CB  or           rcx, rax
00007FF716BF14CE  xorps        xmm9, xmm9
00007FF716BF14D2  cvtsi2ss     xmm9, rcx
00007FF716BF14D7  addss        xmm9, xmm9
00007FF716BF14DC  movaps       xmm7, xmm9
00007FF716BF14E0  mulss        xmm7, dword ptr [__real@3ec90fdb (00007FF716C0C5B0h)]
00007FF716BF14E8  movaps       xmm0, xmm7
00007FF716BF14EB  call         sinf (00007FF716C0A0F9h)
00007FF716BF14F0  movaps       xmm6, xmm0
00007FF716BF14F3  movaps       xmm0, xmm7
00007FF716BF14F6  call         cosf (00007FF716C0A0FFh)
00007FF716BF14FB  cmp          rbp, 10h
00007FF716BF14FF  shufps       xmm0, xmm0, 0h
00007FF716BF1503  mulps        xmm0, xmm10
00007FF716BF1507  shufps       xmm6, xmm6, 0h
00007FF716BF150B  mulps        xmm6, xmm11
00007FF716BF150F  addps        xmm6, xmm0
00007FF716BF1512  mulps        xmm6, xmm12
00007FF716BF1516  addps        xmm6, xmm8
00007FF716BF151A  movaps       xmm0, xmm13
00007FF716BF151E  subps        xmm0, xmm6
00007FF716BF1521  movaps       xmm1, xmm6
00007FF716BF1524  mulps        xmm1, xmm6
00007FF716BF1527  movaps       xmm2, xmm1
00007FF716BF152A  shufps       xmm2, xmm1, 55h
00007FF716BF152E  addss        xmm2, xmm1
00007FF716BF1532  movhlps      xmm1, xmm1
00007FF716BF1535  addss        xmm1, xmm2
00007FF716BF1539  shufps       xmm1, xmm1, 0h
00007FF716BF153D  sqrtps       xmm1, xmm1
00007FF716BF1540  movaps       xmm2, xmm6
00007FF716BF1543  divps        xmm2, xmm1
00007FF716BF1546  movaps       xmm1, xmm2
00007FF716BF1549  unpckhpd     xmm1, xmm2
00007FF716BF154D  xorps        xmm1, xmm14
00007FF716BF1551  xorps        xmm3, xmm3
00007FF716BF1554  movss        xmm3, xmm1
00007FF716BF1558  shufps       xmm3, xmm2, 4h
00007FF716BF155C  movaps       xmm1, xmm0
00007FF716BF155F  shufps       xmm1, xmm0, D2h
00007FF716BF1563  mulps        xmm1, xmm3
00007FF716BF1566  shufps       xmm3, xmm3, D2h
00007FF716BF156A  mulps        xmm3, xmm0
00007FF716BF156D  subps        xmm1, xmm3
00007FF716BF1570  movaps       xmm7, xmm1
00007FF716BF1573  shufps       xmm7, xmm1, D2h
00007FF716BF1577  mulps        xmm1, xmm1
00007FF716BF157A  movaps       xmm0, xmm1
00007FF716BF157D  unpckhpd     xmm0, xmm1
00007FF716BF1581  addss        xmm0, xmm1
00007FF716BF1585  shufps       xmm1, xmm1, 55h
00007FF716BF1589  addss        xmm1, xmm0
00007FF716BF158D  shufps       xmm1, xmm1, 0h
00007FF716BF1591  sqrtps       xmm0, xmm1
00007FF716BF1594  divps        xmm7, xmm0
00007FF716BF1597  mulss        xmm9, xmm15
00007FF716BF159C  movaps       xmmword ptr [rsp+60h], xmm8
00007FF716BF15A2  movaps       xmmword ptr [rsp+90h], xmm8
00007FF716BF15AB  mov          rsi, rbp
00007FF716BF15AE  adc          rsi, 0h
00007FF716BF15B2  cmp          rdi, qword ptr [rsp+48h]
00007FF716BF15B7  je           static void Fabled_Engine::main+389h (00007FF716BF1649h)
00007FF716BF15BD  movaps       xmm0, xmm6
00007FF716BF15C0  shufps       xmm0, xmm6, 55h
00007FF716BF15C4  movaps       xmm1, xmm7
00007FF716BF15C7  shufps       xmm1, xmm7, 55h
00007FF716BF15CB  movaps       xmm2, xmm6
00007FF716BF15CE  unpckhpd     xmm2, xmm6
00007FF716BF15D2  movaps       xmm3, xmm7
00007FF716BF15D5  unpckhpd     xmm3, xmm7
00007FF716BF15D9  shl          rdi, 6h
00007FF716BF15DD  movss        dword ptr [rbx+rdi*1], xmm6
00007FF716BF15E2  movss        dword ptr [rbx+rdi*1+4h], xmm0
00007FF716BF15E8  movss        dword ptr [rbx+rdi*1+8h], xmm2
00007FF716BF15EE  movss        dword ptr [rbx+rdi*1+Ch], xmm9
00007FF716BF15F5  mov          dword ptr [rbx+rdi*1+10h], 0h
00007FF716BF15FD  movss        dword ptr [rbx+rdi*1+14h], xmm7
00007FF716BF1603  movss        dword ptr [rbx+rdi*1+18h], xmm1
00007FF716BF1609  movss        dword ptr [rbx+rdi*1+1Ch], xmm3
00007FF716BF160F  movaps       xmm0, xmmword ptr [rsp+60h]
00007FF716BF1614  movaps       xmmword ptr [rbx+rdi*1+20h], xmm0
00007FF716BF1619  movaps       xmm0, xmmword ptr [rsp+90h]
00007FF716BF1621  movaps       xmmword ptr [rbx+rdi*1+30h], xmm0
00007FF716BF1626  mov          rdi, qword ptr [rsp+50h]
00007FF716BF162B  add          rdi, 1h
00007FF716BF162F  mov          qword ptr [rsp+50h], rdi
00007FF716BF1634  cmp          rbp, Fh
00007FF716BF1638  ja           static void Fabled_Engine::main+3A3h (00007FF716BF1663h)
00007FF716BF163A  mov          rbp, rsi
00007FF716BF163D  cmp          rsi, 11h
00007FF716BF1641  jb           static void Fabled_Engine::main+1F0h (00007FF716BF14B0h)
00007FF716BF1647  jmp          static void Fabled_Engine::main+3A3h (00007FF716BF1663h)
00007FF716BF1649  mov          rcx, r14
00007FF716BF164C  mov          rdx, rdi
00007FF716BF164F  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF716C0AB10h)
00007FF716BF1654  mov          rbx, qword ptr [rsp+40h]
00007FF716BF1659  mov          rdi, qword ptr [rsp+50h]
00007FF716BF165E  jmp          static void Fabled_Engine::main+2FDh (00007FF716BF15BDh)
00007FF716BF1663  mov          esi, 3h
00007FF716BF1668  mov          rdx, qword ptr [rsp+38h]
00007FF716BF166D  lea          r12, [rsp+28h]
00007FF716BF1672  jmp          static void Fabled_Engine::main+3DBh (00007FF716BF169Bh)
00007FF716BF1674  nop          word ptr cs:[rax+rax*1], ax
00007FF716BF167E  nop
00007FF716BF1680  mov          qword ptr [rax+rdx*8], rsi
00007FF716BF1684  add          rdx, 1h
00007FF716BF1688  mov          qword ptr [rsp+38h], rdx
00007FF716BF168D  add          rsi, 1h
00007FF716BF1691  cmp          rsi, 13h
00007FF716BF1695  je           static void Fabled_Engine::main+50Dh (00007FF716BF17CDh)
00007FF716BF169B  cmp          rdx, qword ptr [rsp+30h]
00007FF716BF16A0  je           static void Fabled_Engine::main+4A4h (00007FF716BF1764h)
00007FF716BF16A6  mov          rax, qword ptr [rsp+28h]
00007FF716BF16AB  mov          qword ptr [rax+rdx*8], 0h
00007FF716BF16B3  mov          rdx, qword ptr [rsp+38h]
00007FF716BF16B8  add          rdx, 1h
00007FF716BF16BC  mov          qword ptr [rsp+38h], rdx
00007FF716BF16C1  cmp          rdx, qword ptr [rsp+30h]
00007FF716BF16C6  je           static void Fabled_Engine::main+4B6h (00007FF716BF1776h)
00007FF716BF16CC  lea          rbx, [rsi-1h]
00007FF716BF16D0  mov          qword ptr [rax+rdx*8], rsi
00007FF716BF16D4  mov          rdx, qword ptr [rsp+38h]
00007FF716BF16D9  add          rdx, 1h
00007FF716BF16DD  mov          qword ptr [rsp+38h], rdx
00007FF716BF16E2  cmp          rdx, qword ptr [rsp+30h]
00007FF716BF16E7  je           static void Fabled_Engine::main+4CDh (00007FF716BF178Dh)
00007FF716BF16ED  mov          qword ptr [rax+rdx*8], rbx
00007FF716BF16F1  mov          rdx, qword ptr [rsp+38h]
00007FF716BF16F6  add          rdx, 1h
00007FF716BF16FA  mov          qword ptr [rsp+38h], rdx
00007FF716BF16FF  cmp          rdx, qword ptr [rsp+30h]
00007FF716BF1704  je           static void Fabled_Engine::main+4E4h (00007FF716BF17A4h)
00007FF716BF170A  mov          rax, qword ptr [rsp+28h]
00007FF716BF170F  mov          qword ptr [rax+rdx*8], 1h
00007FF716BF1717  mov          rdx, qword ptr [rsp+38h]
00007FF716BF171C  add          rdx, 1h
00007FF716BF1720  mov          qword ptr [rsp+38h], rdx
00007FF716BF1725  cmp          rdx, qword ptr [rsp+30h]
00007FF716BF172A  je           static void Fabled_Engine::main+4F6h (00007FF716BF17B6h)
00007FF716BF1730  mov          qword ptr [rax+rdx*8], rbx
00007FF716BF1734  mov          rdx, qword ptr [rsp+38h]
00007FF716BF1739  add          rdx, 1h
00007FF716BF173D  mov          qword ptr [rsp+38h], rdx
00007FF716BF1742  cmp          rdx, qword ptr [rsp+30h]
00007FF716BF1747  jne          static void Fabled_Engine::main+3C0h (00007FF716BF1680h)
00007FF716BF174D  mov          rcx, r12
00007FF716BF1750  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF716C0ABD0h)
00007FF716BF1755  mov          rax, qword ptr [rsp+28h]
00007FF716BF175A  mov          rdx, qword ptr [rsp+38h]
00007FF716BF175F  jmp          static void Fabled_Engine::main+3C0h (00007FF716BF1680h)
00007FF716BF1764  mov          rcx, r12
00007FF716BF1767  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF716C0ABD0h)
00007FF716BF176C  mov          rdx, qword ptr [rsp+38h]
00007FF716BF1771  jmp          static void Fabled_Engine::main+3E6h (00007FF716BF16A6h)
00007FF716BF1776  mov          rcx, r12
00007FF716BF1779  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF716C0ABD0h)
00007FF716BF177E  mov          rax, qword ptr [rsp+28h]
00007FF716BF1783  mov          rdx, qword ptr [rsp+38h]
00007FF716BF1788  jmp          static void Fabled_Engine::main+40Ch (00007FF716BF16CCh)
00007FF716BF178D  mov          rcx, r12
00007FF716BF1790  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF716C0ABD0h)
00007FF716BF1795  mov          rax, qword ptr [rsp+28h]
00007FF716BF179A  mov          rdx, qword ptr [rsp+38h]
00007FF716BF179F  jmp          static void Fabled_Engine::main+42Dh (00007FF716BF16EDh)
00007FF716BF17A4  mov          rcx, r12
00007FF716BF17A7  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF716C0ABD0h)
00007FF716BF17AC  mov          rdx, qword ptr [rsp+38h]
00007FF716BF17B1  jmp          static void Fabled_Engine::main+44Ah (00007FF716BF170Ah)
00007FF716BF17B6  mov          rcx, r12
00007FF716BF17B9  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF716C0ABD0h)
00007FF716BF17BE  mov          rax, qword ptr [rsp+28h]
00007FF716BF17C3  mov          rdx, qword ptr [rsp+38h]
00007FF716BF17C8  jmp          static void Fabled_Engine::main+470h (00007FF716BF1730h)
00007FF716BF17CD  mov          rax, qword ptr [rsp+38h]
00007FF716BF17D2  mov          qword ptr [rsp+74h], rax
00007FF716BF17D7  movups       xmm0, xmmword ptr [rsp+28h]
00007FF716BF17DC  movups       xmmword ptr [rsp+64h], xmm0
00007FF716BF17E1  mov          rcx, qword ptr [00007FF716C131C8h]
00007FF716BF17E8  test         rcx, rcx
00007FF716BF17EB  jne          static void Fabled_Engine::main+545h (00007FF716BF1805h)
00007FF716BF17ED  call         GetProcessHeap (00007FF716C08FD2h)
00007FF716BF17F2  test         rax, rax
00007FF716BF17F5  je           static void Fabled_Engine::main+81Eh (00007FF716BF1ADEh)
00007FF716BF17FB  mov          rcx, rax
00007FF716BF17FE  mov          qword ptr [00007FF716C131C8h], rax
00007FF716BF1805  mov          r8d, 40h
00007FF716BF180B  xor          edx, edx
00007FF716BF180D  call         HeapAlloc (00007FF716C08FD8h)
00007FF716BF1812  test         rax, rax
00007FF716BF1815  je           static void Fabled_Engine::main+81Eh (00007FF716BF1ADEh)
00007FF716BF181B  mov          rbx, rax
00007FF716BF181E  mov          rax, qword ptr [rsp+50h]
00007FF716BF1823  mov          qword ptr [rbx+10h], rax
00007FF716BF1827  movups       xmm0, xmmword ptr [rsp+40h]
00007FF716BF182C  movups       xmmword ptr [rbx], xmm0
00007FF716BF182F  movups       xmm0, xmmword ptr [rsp+60h]
00007FF716BF1834  movups       xmm1, xmmword ptr [rsp+6Ch]
00007FF716BF1839  movups       xmmword ptr [rbx+1Ch], xmm0
00007FF716BF183D  movups       xmmword ptr [rbx+28h], xmm1
00007FF716BF1841  mov          dword ptr [rbx+18h], 0h
00007FF716BF1848  lea          rax, [00007FF716C0FB40h]
00007FF716BF184F  mov          qword ptr [rsp+90h], rax
00007FF716BF1857  mov          qword ptr [rsp+98h], 6h