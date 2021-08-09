00007FF6A9AA135B  mov          rcx, qword ptr [00007FF6A9ACA1C8h]
00007FF6A9AA1362  test         rcx, rcx
00007FF6A9AA1365  jne          static void Fabled_Engine::main+BFh (00007FF6A9AA137Fh)
00007FF6A9AA1367  call         GetProcessHeap (00007FF6A9ABF322h)
00007FF6A9AA136C  test         rax, rax
00007FF6A9AA136F  je           static void Fabled_Engine::main+8F7h (00007FF6A9AA1BB7h)
00007FF6A9AA1375  mov          rcx, rax
00007FF6A9AA1378  mov          qword ptr [00007FF6A9ACA1C8h], rax
00007FF6A9AA137F  mov          r8d, 5A0h
00007FF6A9AA1385  xor          edx, edx
00007FF6A9AA1387  call         HeapAlloc (00007FF6A9ABF328h)
00007FF6A9AA138C  test         rax, rax
00007FF6A9AA138F  je           static void Fabled_Engine::main+8F7h (00007FF6A9AA1BB7h)
00007FF6A9AA1395  mov          qword ptr [rsp+28h], rax
00007FF6A9AA139A  movaps       xmm0, xmmword ptr [__xmm@000000000000000000000000000000b4 (00007FF6A9AC2530h)]
00007FF6A9AA13A1  movups       xmmword ptr [rsp+30h], xmm0
00007FF6A9AA13A6  mov          rcx, qword ptr [00007FF6A9ACA1C8h]
00007FF6A9AA13AD  test         rcx, rcx
00007FF6A9AA13B0  jne          static void Fabled_Engine::main+10Ah (00007FF6A9AA13CAh)
00007FF6A9AA13B2  call         GetProcessHeap (00007FF6A9ABF322h)
00007FF6A9AA13B7  test         rax, rax
00007FF6A9AA13BA  je           static void Fabled_Engine::main+908h (00007FF6A9AA1BC8h)
00007FF6A9AA13C0  mov          rcx, rax
00007FF6A9AA13C3  mov          qword ptr [00007FF6A9ACA1C8h], rax
00007FF6A9AA13CA  mov          r8d, 800h
00007FF6A9AA13D0  xor          edx, edx
00007FF6A9AA13D2  call         HeapAlloc (00007FF6A9ABF328h)
00007FF6A9AA13D7  test         rax, rax
00007FF6A9AA13DA  je           static void Fabled_Engine::main+908h (00007FF6A9AA1BC8h)
00007FF6A9AA13E0  mov          qword ptr [rsp+40h], rax
00007FF6A9AA13E5  mov          rcx, 4000000000000000h
00007FF6A9AA13EF  mov          qword ptr [rax], rcx
00007FF6A9AA13F2  mov          dword ptr [rax+8h], 0h
00007FF6A9AA13F9  movaps       xmm0, xmmword ptr [__xmm@3f800000000000003f80000000000000 (00007FF6A9AC2540h)]
00007FF6A9AA1400  movups       xmmword ptr [rax+Ch], xmm0
00007FF6A9AA1404  movaps       xmm0, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF6A9AC2550h)]
00007FF6A9AA140B  movups       xmmword ptr [rax+1Ch], xmm0
00007FF6A9AA140F  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000000000003f800000 (00007FF6A9AC2560h)]
00007FF6A9AA1416  movups       xmmword ptr [rax+2Ch], xmm0
00007FF6A9AA141A  mov          dword ptr [rax+3Ch], 3F800000h
00007FF6A9AA1421  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000020 (00007FF6A9AC2570h)]
00007FF6A9AA1428  movups       xmmword ptr [rsp+48h], xmm0
00007FF6A9AA142D  xorps        xmm0, xmm0
00007FF6A9AA1430  movaps       xmmword ptr [rax+40h], xmm0
00007FF6A9AA1434  movaps       xmm0, xmmword ptr [__xmm@00000000bf80000000000000bf800000 (00007FF6A9AC2580h)]
00007FF6A9AA143B  movaps       xmmword ptr [rax+50h], xmm0
00007FF6A9AA143F  movaps       xmm0, xmmword ptr [__xmm@3f8000000000000000000000bf800000 (00007FF6A9AC2590h)]
00007FF6A9AA1446  movaps       xmmword ptr [rax+60h], xmm0
00007FF6A9AA144A  movaps       xmm0, xmmword ptr [__xmm@3f800000bf8000000000000000000000 (00007FF6A9AC25A0h)]
00007FF6A9AA1451  movaps       xmmword ptr [rax+70h], xmm0
00007FF6A9AA1455  mov          qword ptr [rsp+50h], 2h
00007FF6A9AA145E  mov          r13d, 2h
00007FF6A9AA1464  xor          ecx, ecx
00007FF6A9AA1466  movss        xmm10, dword ptr [__real@3f800000 (00007FF6A9AC25B4h)]
00007FF6A9AA146F  movaps       xmm12, xmmword ptr [__xmm@40a0000040a0000040a0000040a00000 (00007FF6A9AC25D0h)]
00007FF6A9AA1477  movss        xmm13, dword ptr [__real@3ebe26eb (00007FF6A9AC25E0h)]
00007FF6A9AA1480  movss        xmm14, dword ptr [__real@3f6db0a6 (00007FF6A9AC25E4h)]
00007FF6A9AA1489  movaps       xmm15, xmmword ptr [__xmm@00000000bf8000008000000080000000 (00007FF6A9AC25F0h)]
00007FF6A9AA1491  movss        xmm8, dword ptr [__real@41f00000 (00007FF6A9AC2600h)]
00007FF6A9AA149A  xor          eax, eax
00007FF6A9AA149C  nop          dword ptr [rax], eax
00007FF6A9AA14A0  cmp          rcx, 1Eh
00007FF6A9AA14A4  adc          rax, 0h
00007FF6A9AA14A8  test         rcx, rcx
00007FF6A9AA14AB  mov          qword ptr [rsp+98h], rcx
00007FF6A9AA14B3  mov          qword ptr [rsp+90h], rax
00007FF6A9AA14BB  js           static void Fabled_Engine::main+210h (00007FF6A9AA14D0h)
00007FF6A9AA14BD  xorps        xmm9, xmm9
00007FF6A9AA14C1  cvtsi2ss     xmm9, rcx
00007FF6A9AA14C6  jmp          static void Fabled_Engine::main+226h (00007FF6A9AA14E6h)
00007FF6A9AA14C8  nop          dword ptr [rax+rax*1], eax
00007FF6A9AA14D0  mov          rax, rcx
00007FF6A9AA14D3  shr          rax, 1h
00007FF6A9AA14D6  and          ecx, 1h
00007FF6A9AA14D9  or           rcx, rax
00007FF6A9AA14DC  cvtsi2ss     xmm9, rcx
00007FF6A9AA14E1  addss        xmm9, xmm9
00007FF6A9AA14E6  movaps       xmm6, xmm9
00007FF6A9AA14EA  mulss        xmm6, dword ptr [__real@3e567750 (00007FF6A9AC25B0h)]
00007FF6A9AA14F2  movaps       xmm0, xmm6
00007FF6A9AA14F5  call         sinf (00007FF6A9AC0449h)
00007FF6A9AA14FA  movaps       xmm11, xmm0
00007FF6A9AA14FE  movaps       xmm0, xmm6
00007FF6A9AA1501  call         cosf (00007FF6A9AC044Fh)
00007FF6A9AA1506  movaps       xmm6, xmm0
00007FF6A9AA1509  shufps       xmm0, xmm0, 0h
00007FF6A9AA150D  mulps        xmm0, xmm10
00007FF6A9AA1511  movaps       xmm7, xmm11
00007FF6A9AA1515  shufps       xmm7, xmm11, 0h
00007FF6A9AA151A  mulps        xmm7, xmmword ptr [__xmm@3f8000003f8000000000000000000000 (00007FF6A9AC25C0h)]
00007FF6A9AA1521  addps        xmm7, xmm0
00007FF6A9AA1524  mulps        xmm7, xmm12
00007FF6A9AA1528  addps        xmm7, xmmword ptr [__xmm@00000000000000000000000000000000 (00007FF6A9AC2620h)]
00007FF6A9AA152F  mulss        xmm6, xmm13
00007FF6A9AA1534  unpcklps     xmm6, xmm14
00007FF6A9AA1538  mulss        xmm11, xmm13
00007FF6A9AA153D  movaps       xmm0, xmm6
00007FF6A9AA1540  movlhps      xmm0, xmm11
00007FF6A9AA1544  shufps       xmm6, xmm11, 4h
00007FF6A9AA1549  mulps        xmm0, xmm0
00007FF6A9AA154C  movaps       xmm1, xmm0
00007FF6A9AA154F  shufps       xmm1, xmm0, 55h
00007FF6A9AA1553  addss        xmm1, xmm0
00007FF6A9AA1557  movhlps      xmm0, xmm0
00007FF6A9AA155A  addss        xmm0, xmm1
00007FF6A9AA155E  shufps       xmm0, xmm0, 0h
00007FF6A9AA1562  sqrtps       xmm0, xmm0
00007FF6A9AA1565  divps        xmm6, xmm0
00007FF6A9AA1568  movaps       xmm0, xmm7
00007FF6A9AA156B  mulps        xmm0, xmm7
00007FF6A9AA156E  movaps       xmm1, xmm0
00007FF6A9AA1571  shufps       xmm1, xmm0, 55h
00007FF6A9AA1575  addss        xmm1, xmm0
00007FF6A9AA1579  movhlps      xmm0, xmm0
00007FF6A9AA157C  addss        xmm0, xmm1
00007FF6A9AA1580  shufps       xmm0, xmm0, 0h
00007FF6A9AA1584  sqrtps       xmm0, xmm0
00007FF6A9AA1587  movaps       xmm1, xmm7
00007FF6A9AA158A  divps        xmm1, xmm0
00007FF6A9AA158D  movaps       xmm0, xmm1
00007FF6A9AA1590  mulps        xmm0, xmm10
00007FF6A9AA1594  shufps       xmm0, xmm0, D2h
00007FF6A9AA1598  mulps        xmm1, xmm15
00007FF6A9AA159C  addps        xmm1, xmm0
00007FF6A9AA159F  movaps       xmm2, xmm1
00007FF6A9AA15A2  shufps       xmm2, xmm1, 52h
00007FF6A9AA15A6  movaps       xmm0, xmm6
00007FF6A9AA15A9  shufps       xmm0, xmm6, D2h
00007FF6A9AA15AD  mulps        xmm0, xmm2
00007FF6A9AA15B0  movaps       xmm2, xmm1
00007FF6A9AA15B3  shufps       xmm2, xmm1, 49h
00007FF6A9AA15B7  mulps        xmm2, xmm6
00007FF6A9AA15BA  subps        xmm0, xmm2
00007FF6A9AA15BD  movd         esi, xmm7
00007FF6A9AA15C1  movaps       xmm2, xmm7
00007FF6A9AA15C4  shufps       xmm2, xmm7, 55h
00007FF6A9AA15C8  movd         eax, xmm2
00007FF6A9AA15CC  shl          rax, 20h
00007FF6A9AA15D0  or           rsi, rax
00007FF6A9AA15D3  divss        xmm9, xmm8
00007FF6A9AA15D8  movd         ebp, xmm6
00007FF6A9AA15DC  movaps       xmm2, xmm6
00007FF6A9AA15DF  shufps       xmm2, xmm6, 55h
00007FF6A9AA15E3  movd         eax, xmm2
00007FF6A9AA15E7  shl          rax, 20h
00007FF6A9AA15EB  or           rbp, rax
00007FF6A9AA15EE  movaps       xmm2, xmm1
00007FF6A9AA15F1  unpckhpd     xmm2, xmm1
00007FF6A9AA15F5  movd         ebx, xmm2
00007FF6A9AA15F9  movd         eax, xmm1
00007FF6A9AA15FD  shufps       xmm1, xmm1, 55h
00007FF6A9AA1601  movd         r14d, xmm1
00007FF6A9AA1606  shl          rax, 20h
00007FF6A9AA160A  or           rbx, rax
00007FF6A9AA160D  mov          rcx, 3F80000000000000h
00007FF6A9AA1617  or           r14, rcx
00007FF6A9AA161A  movaps       xmm1, xmm0
00007FF6A9AA161D  unpckhpd     xmm1, xmm0
00007FF6A9AA1621  movd         r15d, xmm1
00007FF6A9AA1626  movd         eax, xmm0
00007FF6A9AA162A  shufps       xmm0, xmm0, 55h
00007FF6A9AA162E  movd         r12d, xmm0
00007FF6A9AA1633  shl          rax, 20h
00007FF6A9AA1637  or           r15, rax
00007FF6A9AA163A  or           r12, rcx
00007FF6A9AA163D  cmp          r13, qword ptr [rsp+48h]
00007FF6A9AA1642  je           static void Fabled_Engine::main+408h (00007FF6A9AA16C8h)
00007FF6A9AA1648  punpckhqdq   xmm7, xmm7
00007FF6A9AA164C  movd         eax, xmm7
00007FF6A9AA1650  punpckhqdq   xmm6, xmm6
00007FF6A9AA1654  movd         ecx, xmm6
00007FF6A9AA1658  mov          rdx, qword ptr [rsp+40h]
00007FF6A9AA165D  mov          r13, qword ptr [rsp+50h]
00007FF6A9AA1662  mov          rdi, r13
00007FF6A9AA1665  shl          rdi, 6h
00007FF6A9AA1669  add          r13, 1h
00007FF6A9AA166D  mov          qword ptr [rdx+rdi*1], rsi
00007FF6A9AA1671  mov          dword ptr [rdx+rdi*1+8h], eax
00007FF6A9AA1675  movss        dword ptr [rdx+rdi*1+Ch], xmm9
00007FF6A9AA167C  mov          dword ptr [rdx+rdi*1+10h], 0h
00007FF6A9AA1684  mov          qword ptr [rdx+rdi*1+14h], rbp
00007FF6A9AA1689  mov          dword ptr [rdx+rdi*1+1Ch], ecx
00007FF6A9AA168D  mov          qword ptr [rdx+rdi*1+28h], r14
00007FF6A9AA1692  mov          qword ptr [rdx+rdi*1+20h], rbx
00007FF6A9AA1697  mov          qword ptr [rdx+rdi*1+38h], r12
00007FF6A9AA169C  mov          qword ptr [rdx+rdi*1+30h], r15
00007FF6A9AA16A1  mov          qword ptr [rsp+50h], r13
00007FF6A9AA16A6  cmp          qword ptr [rsp+98h], 1Dh
00007FF6A9AA16AF  mov          rax, qword ptr [rsp+90h]
00007FF6A9AA16B7  ja           static void Fabled_Engine::main+41Ah (00007FF6A9AA16DAh)
00007FF6A9AA16B9  mov          rcx, rax
00007FF6A9AA16BC  cmp          rax, 1Fh
00007FF6A9AA16C0  jb           static void Fabled_Engine::main+1E0h (00007FF6A9AA14A0h)
00007FF6A9AA16C6  jmp          static void Fabled_Engine::main+41Ah (00007FF6A9AA16DAh)
00007FF6A9AA16C8  lea          rcx, [rsp+40h]
00007FF6A9AA16CD  mov          rdx, r13
00007FF6A9AA16D0  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF6A9AC0F80h)
00007FF6A9AA16D5  jmp          static void Fabled_Engine::main+388h (00007FF6A9AA1648h)
00007FF6A9AA16DA  mov          esi, 3h
00007FF6A9AA16DF  mov          rdx, qword ptr [rsp+38h]
00007FF6A9AA16E4  lea          rdi, [rsp+28h]
00007FF6A9AA16E9  jmp          static void Fabled_Engine::main+44Bh (00007FF6A9AA170Bh)
00007FF6A9AA16EB  nop          dword ptr [rax+rax*1], eax
00007FF6A9AA16F0  mov          qword ptr [rcx+rdx*8], rsi
00007FF6A9AA16F4  add          rdx, 1h
00007FF6A9AA16F8  mov          qword ptr [rsp+38h], rdx
00007FF6A9AA16FD  add          rsi, 1h
00007FF6A9AA1701  cmp          rsi, 21h
00007FF6A9AA1705  je           static void Fabled_Engine::main+582h (00007FF6A9AA1842h)
00007FF6A9AA170B  mov          rax, qword ptr [rsp+30h]
00007FF6A9AA1710  cmp          rdx, rax
00007FF6A9AA1713  je           static void Fabled_Engine::main+4FCh (00007FF6A9AA17BCh)
00007FF6A9AA1719  mov          rcx, qword ptr [rsp+28h]
00007FF6A9AA171E  mov          qword ptr [rcx+rdx*8], 0h
00007FF6A9AA1726  add          rdx, 1h
00007FF6A9AA172A  mov          qword ptr [rsp+38h], rdx
00007FF6A9AA172F  cmp          rdx, rax
00007FF6A9AA1732  je           static void Fabled_Engine::main+513h (00007FF6A9AA17D3h)
00007FF6A9AA1738  lea          rbx, [rsi-1h]
00007FF6A9AA173C  mov          qword ptr [rcx+rdx*8], rsi
00007FF6A9AA1740  add          rdx, 1h
00007FF6A9AA1744  mov          qword ptr [rsp+38h], rdx
00007FF6A9AA1749  cmp          rdx, rax
00007FF6A9AA174C  je           static void Fabled_Engine::main+532h (00007FF6A9AA17F2h)
00007FF6A9AA1752  mov          qword ptr [rcx+rdx*8], rbx
00007FF6A9AA1756  add          rdx, 1h
00007FF6A9AA175A  mov          qword ptr [rsp+38h], rdx
00007FF6A9AA175F  mov          rax, qword ptr [rsp+30h]
00007FF6A9AA1764  cmp          rdx, rax
00007FF6A9AA1767  je           static void Fabled_Engine::main+54Ch (00007FF6A9AA180Ch)
00007FF6A9AA176D  mov          rcx, qword ptr [rsp+28h]
00007FF6A9AA1772  mov          qword ptr [rcx+rdx*8], 1h
00007FF6A9AA177A  add          rdx, 1h
00007FF6A9AA177E  mov          qword ptr [rsp+38h], rdx
00007FF6A9AA1783  cmp          rdx, rax
00007FF6A9AA1786  je           static void Fabled_Engine::main+563h (00007FF6A9AA1823h)
00007FF6A9AA178C  mov          qword ptr [rcx+rdx*8], rbx
00007FF6A9AA1790  add          rdx, 1h
00007FF6A9AA1794  mov          qword ptr [rsp+38h], rdx
00007FF6A9AA1799  cmp          rdx, rax
00007FF6A9AA179C  jne          static void Fabled_Engine::main+430h (00007FF6A9AA16F0h)
00007FF6A9AA17A2  mov          rcx, rdi
00007FF6A9AA17A5  mov          rdx, rax
00007FF6A9AA17A8  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A9AC0EC0h)
00007FF6A9AA17AD  mov          rcx, qword ptr [rsp+28h]
00007FF6A9AA17B2  mov          rdx, qword ptr [rsp+38h]
00007FF6A9AA17B7  jmp          static void Fabled_Engine::main+430h (00007FF6A9AA16F0h)
00007FF6A9AA17BC  mov          rcx, rdi
00007FF6A9AA17BF  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A9AC0EC0h)
00007FF6A9AA17C4  mov          rax, qword ptr [rsp+30h]
00007FF6A9AA17C9  mov          rdx, qword ptr [rsp+38h]
00007FF6A9AA17CE  jmp          static void Fabled_Engine::main+459h (00007FF6A9AA1719h)
00007FF6A9AA17D3  mov          rcx, rdi
00007FF6A9AA17D6  mov          rdx, rax
00007FF6A9AA17D9  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A9AC0EC0h)
00007FF6A9AA17DE  mov          rcx, qword ptr [rsp+28h]
00007FF6A9AA17E3  mov          rax, qword ptr [rsp+30h]
00007FF6A9AA17E8  mov          rdx, qword ptr [rsp+38h]
00007FF6A9AA17ED  jmp          static void Fabled_Engine::main+478h (00007FF6A9AA1738h)
00007FF6A9AA17F2  mov          rcx, rdi
00007FF6A9AA17F5  mov          rdx, rax
00007FF6A9AA17F8  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A9AC0EC0h)
00007FF6A9AA17FD  mov          rcx, qword ptr [rsp+28h]
00007FF6A9AA1802  mov          rdx, qword ptr [rsp+38h]
00007FF6A9AA1807  jmp          static void Fabled_Engine::main+492h (00007FF6A9AA1752h)
00007FF6A9AA180C  mov          rcx, rdi
00007FF6A9AA180F  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A9AC0EC0h)
00007FF6A9AA1814  mov          rax, qword ptr [rsp+30h]
00007FF6A9AA1819  mov          rdx, qword ptr [rsp+38h]
00007FF6A9AA181E  jmp          static void Fabled_Engine::main+4ADh (00007FF6A9AA176Dh)
00007FF6A9AA1823  mov          rcx, rdi
00007FF6A9AA1826  mov          rdx, rax
00007FF6A9AA1829  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A9AC0EC0h)
00007FF6A9AA182E  mov          rcx, qword ptr [rsp+28h]
00007FF6A9AA1833  mov          rax, qword ptr [rsp+30h]
00007FF6A9AA1838  mov          rdx, qword ptr [rsp+38h]
00007FF6A9AA183D  jmp          static void Fabled_Engine::main+4CCh (00007FF6A9AA178Ch)
00007FF6A9AA1842  mov          rax, qword ptr [rsp+38h]
00007FF6A9AA1847  mov          qword ptr [rsp+74h], rax
00007FF6A9AA184C  movups       xmm0, xmmword ptr [rsp+28h]
00007FF6A9AA1851  movups       xmmword ptr [rsp+64h], xmm0
00007FF6A9AA1856  mov          rcx, qword ptr [00007FF6A9ACA1C8h]
00007FF6A9AA185D  test         rcx, rcx
00007FF6A9AA1860  jne          static void Fabled_Engine::main+5BAh (00007FF6A9AA187Ah)
00007FF6A9AA1862  call         GetProcessHeap (00007FF6A9ABF322h)
00007FF6A9AA1867  test         rax, rax
00007FF6A9AA186A  je           static void Fabled_Engine::main+90Fh (00007FF6A9AA1BCFh)
00007FF6A9AA1870  mov          rcx, rax
00007FF6A9AA1873  mov          qword ptr [00007FF6A9ACA1C8h], rax
00007FF6A9AA187A  mov          r8d, 40h
00007FF6A9AA1880  xor          edx, edx
00007FF6A9AA1882  call         HeapAlloc (00007FF6A9ABF328h)
00007FF6A9AA1887  test         rax, rax
00007FF6A9AA188A  je           static void Fabled_Engine::main+90Fh (00007FF6A9AA1BCFh)
00007FF6A9AA1890  mov          rcx, qword ptr [rsp+50h]
00007FF6A9AA1895  mov          qword ptr [rax+10h], rcx
00007FF6A9AA1899  movups       xmm0, xmmword ptr [rsp+40h]
00007FF6A9AA189E  movups       xmmword ptr [rax], xmm0
00007FF6A9AA18A1  movups       xmm0, xmmword ptr [rsp+60h]
00007FF6A9AA18A6  movups       xmm1, xmmword ptr [rsp+6Ch]
00007FF6A9AA18AB  movups       xmmword ptr [rax+1Ch], xmm0
00007FF6A9AA18AF  movups       xmmword ptr [rax+28h], xmm1
00007FF6A9AA18B3  mov          dword ptr [rax+18h], 0h
00007FF6A9AA18BA  mov          qword ptr [rsp+A0h], rax
00007FF6A9AA18C2  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000001 (00007FF6A9AC2610h)]
00007FF6A9AA18C9  movups       xmmword ptr [rsp+A8h], xmm0
00007FF6A9AA18D1  lea          rax, [rsp+A0h]