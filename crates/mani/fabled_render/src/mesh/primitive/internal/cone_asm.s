# --------------- Cone Dissassembly -------------------
00007FF724EB12AB  mov          rcx, qword ptr [00007FF724ED31C8h]
00007FF724EB12B2  test         rcx, rcx
00007FF724EB12B5  jne          static void Fabled_Engine::main+BFh (00007FF724EB12CFh)
00007FF724EB12B7  call         GetProcessHeap (00007FF724EC925Ch)
00007FF724EB12BC  test         rax, rax
00007FF724EB12BF  je           static void Fabled_Engine::main+88Ch (00007FF724EB1A9Ch)
00007FF724EB12C5  mov          rcx, rax
00007FF724EB12C8  mov          qword ptr [00007FF724ED31C8h], rax
00007FF724EB12CF  mov          r8d, 90h
00007FF724EB12D5  xor          edx, edx
00007FF724EB12D7  call         HeapAlloc (00007FF724EC9262h)
00007FF724EB12DC  test         rax, rax
00007FF724EB12DF  je           static void Fabled_Engine::main+88Ch (00007FF724EB1A9Ch)
00007FF724EB12E5  mov          qword ptr [rsp+30h], rax
00007FF724EB12EA  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000012 (00007FF724ECC520h)]
00007FF724EB12F1  movups       xmmword ptr [rsp+38h], xmm0
00007FF724EB12F6  mov          rcx, qword ptr [00007FF724ED31C8h]
00007FF724EB12FD  test         rcx, rcx
00007FF724EB1300  jne          static void Fabled_Engine::main+10Ah (00007FF724EB131Ah)
00007FF724EB1302  call         GetProcessHeap (00007FF724EC925Ch)
00007FF724EB1307  test         rax, rax
00007FF724EB130A  je           static void Fabled_Engine::main+89Dh (00007FF724EB1AADh)
00007FF724EB1310  mov          rcx, rax
00007FF724EB1313  mov          qword ptr [00007FF724ED31C8h], rax
00007FF724EB131A  mov          r8d, 140h
00007FF724EB1320  xor          edx, edx
00007FF724EB1322  call         HeapAlloc (00007FF724EC9262h)
00007FF724EB1327  test         rax, rax
00007FF724EB132A  je           static void Fabled_Engine::main+89Dh (00007FF724EB1AADh)
00007FF724EB1330  mov          qword ptr [rsp+48h], rax
00007FF724EB1335  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000005 (00007FF724ECC530h)]
00007FF724EB133C  movups       xmmword ptr [rsp+50h], xmm0
00007FF724EB1341  mov          rcx, 4000000000000000h
00007FF724EB134B  mov          qword ptr [rax], rcx
00007FF724EB134E  mov          dword ptr [rax+8h], 0h
00007FF724EB1355  movaps       xmm0, xmmword ptr [__xmm@3f800000000000003f80000000000000 (00007FF724ECC540h)]
00007FF724EB135C  movups       xmmword ptr [rax+Ch], xmm0
00007FF724EB1360  movaps       xmm0, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF724ECC550h)]
00007FF724EB1367  movups       xmmword ptr [rax+1Ch], xmm0
00007FF724EB136B  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000000000003f800000 (00007FF724ECC560h)]
00007FF724EB1372  movups       xmmword ptr [rax+2Ch], xmm0
00007FF724EB1376  mov          dword ptr [rax+3Ch], 3F800000h
00007FF724EB137D  mov          qword ptr [rsp+58h], 1h
00007FF724EB1386  mov          rbx, qword ptr [rsp+48h]
00007FF724EB138B  xorps        xmm0, xmm0
00007FF724EB138E  movaps       xmmword ptr [rbx+40h], xmm0
00007FF724EB1392  movaps       xmm0, xmmword ptr [__xmm@00000000bf80000000000000bf800000 (00007FF724ECC570h)]
00007FF724EB1399  movaps       xmmword ptr [rbx+50h], xmm0
00007FF724EB139D  movaps       xmm0, xmmword ptr [__xmm@3f8000000000000000000000bf800000 (00007FF724ECC580h)]
00007FF724EB13A4  movaps       xmmword ptr [rbx+60h], xmm0
00007FF724EB13A8  movaps       xmm0, xmmword ptr [__xmm@3f800000bf8000000000000000000000 (00007FF724ECC590h)]
00007FF724EB13AF  movaps       xmmword ptr [rbx+70h], xmm0
00007FF724EB13B3  mov          qword ptr [rsp+58h], 2h
00007FF724EB13BC  mov          edi, 2h
00007FF724EB13C1  xor          r13d, r13d
00007FF724EB13C4  movss        xmm10, dword ptr [__real@3f800000 (00007FF724ECC5A4h)]
00007FF724EB13CD  movaps       xmm11, xmmword ptr [__xmm@3f8000003f8000000000000000000000 (00007FF724ECC5B0h)]
00007FF724EB13D5  movaps       xmm12, xmmword ptr [__xmm@40a0000040a0000040a0000040a00000 (00007FF724ECC5C0h)]
00007FF724EB13DD  movaps       xmm13, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF724ECC5D0h)]
00007FF724EB13E5  movaps       xmm14, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF724ECC5E0h)]
00007FF724EB13ED  movss        xmm15, dword ptr [__real@40400000 (00007FF724ECC5F0h)]
00007FF724EB13F6  xor          eax, eax
00007FF724EB13F8  nop          dword ptr [rax+rax*1], eax
00007FF724EB1400  cmp          r13, 3h
00007FF724EB1404  adc          rax, 0h
00007FF724EB1408  test         r13, r13
00007FF724EB140B  mov          qword ptr [rsp+98h], rax
00007FF724EB1413  js           static void Fabled_Engine::main+210h (00007FF724EB1420h)
00007FF724EB1415  xorps        xmm8, xmm8
00007FF724EB1419  cvtsi2ss     xmm8, r13
00007FF724EB141E  jmp          static void Fabled_Engine::main+22Dh (00007FF724EB143Dh)
00007FF724EB1420  mov          rax, r13
00007FF724EB1423  shr          rax, 1h
00007FF724EB1426  mov          ecx, r13d
00007FF724EB1429  and          ecx, 1h
00007FF724EB142C  or           rcx, rax
00007FF724EB142F  xorps        xmm8, xmm8
00007FF724EB1433  cvtsi2ss     xmm8, rcx
00007FF724EB1438  addss        xmm8, xmm8
00007FF724EB143D  movaps       xmm7, xmm8
00007FF724EB1441  mulss        xmm7, dword ptr [__real@40060a92 (00007FF724ECC5A0h)]
00007FF724EB1449  movaps       xmm0, xmm7
00007FF724EB144C  call         sinf (00007FF724ECA389h)
00007FF724EB1451  movaps       xmm6, xmm0
00007FF724EB1454  movaps       xmm0, xmm7
00007FF724EB1457  call         cosf (00007FF724ECA38Fh)
00007FF724EB145C  shufps       xmm0, xmm0, 0h
00007FF724EB1460  mulps        xmm0, xmm10
00007FF724EB1464  shufps       xmm6, xmm6, 0h
00007FF724EB1468  mulps        xmm6, xmm11
00007FF724EB146C  addps        xmm6, xmm0
00007FF724EB146F  mulps        xmm6, xmm12
00007FF724EB1473  addps        xmm6, xmmword ptr [__xmm@00000000000000000000000000000000 (00007FF724ECC600h)]
00007FF724EB147A  movaps       xmm1, xmm13
00007FF724EB147E  subps        xmm1, xmm6
00007FF724EB1481  movaps       xmm0, xmm6
00007FF724EB1484  mulps        xmm0, xmm6
00007FF724EB1487  movaps       xmm2, xmm0
00007FF724EB148A  shufps       xmm2, xmm0, 55h
00007FF724EB148E  addss        xmm2, xmm0
00007FF724EB1492  movhlps      xmm0, xmm0
00007FF724EB1495  addss        xmm0, xmm2
00007FF724EB1499  shufps       xmm0, xmm0, 0h
00007FF724EB149D  sqrtps       xmm2, xmm0
00007FF724EB14A0  movaps       xmm0, xmm6
00007FF724EB14A3  divps        xmm0, xmm2
00007FF724EB14A6  movaps       xmm7, xmm0
00007FF724EB14A9  unpckhpd     xmm7, xmm0
00007FF724EB14AD  xorps        xmm7, xmm14
00007FF724EB14B1  xorps        xmm2, xmm2
00007FF724EB14B4  movss        xmm2, xmm7
00007FF724EB14B8  shufps       xmm2, xmm0, 4h
00007FF724EB14BC  movaps       xmm3, xmm1
00007FF724EB14BF  shufps       xmm3, xmm1, D2h
00007FF724EB14C3  movaps       xmm4, xmm2
00007FF724EB14C6  shufps       xmm4, xmm2, D2h
00007FF724EB14CA  mulps        xmm3, xmm2
00007FF724EB14CD  mulps        xmm1, xmm4
00007FF724EB14D0  subps        xmm3, xmm1
00007FF724EB14D3  movaps       xmm9, xmm3
00007FF724EB14D7  shufps       xmm9, xmm3, 52h
00007FF724EB14DC  mulps        xmm3, xmm3
00007FF724EB14DF  movaps       xmm1, xmm3
00007FF724EB14E2  unpckhpd     xmm1, xmm3
00007FF724EB14E6  addss        xmm1, xmm3
00007FF724EB14EA  shufps       xmm3, xmm3, 55h
00007FF724EB14EE  addss        xmm3, xmm1
00007FF724EB14F2  shufps       xmm3, xmm3, 0h
00007FF724EB14F6  sqrtps       xmm1, xmm3
00007FF724EB14F9  divps        xmm9, xmm1
00007FF724EB14FD  movaps       xmm1, xmm9
00007FF724EB1501  shufps       xmm1, xmm9, D2h
00007FF724EB1506  mulps        xmm1, xmm2
00007FF724EB1509  mulps        xmm4, xmm9
00007FF724EB150D  subps        xmm1, xmm4
00007FF724EB1510  movd         ebp, xmm6
00007FF724EB1514  movaps       xmm2, xmm6
00007FF724EB1517  shufps       xmm2, xmm6, 55h
00007FF724EB151B  movd         eax, xmm2
00007FF724EB151F  shl          rax, 20h
00007FF724EB1523  or           rbp, rax
00007FF724EB1526  divss        xmm8, xmm15
00007FF724EB152B  movd         esi, xmm9
00007FF724EB1530  movaps       xmm2, xmm9
00007FF724EB1534  shufps       xmm2, xmm9, 55h
00007FF724EB1539  movd         eax, xmm2
00007FF724EB153D  shl          rax, 20h
00007FF724EB1541  or           rsi, rax
00007FF724EB1544  movd         r14d, xmm0
00007FF724EB1549  mov          rcx, 3F80000000000000h
00007FF724EB1553  or           r14, rcx
00007FF724EB1556  movaps       xmm0, xmm1
00007FF724EB1559  unpckhpd     xmm0, xmm1
00007FF724EB155D  movd         r15d, xmm0
00007FF724EB1562  movd         eax, xmm1
00007FF724EB1566  shufps       xmm1, xmm1, 55h
00007FF724EB156A  movd         r12d, xmm1
00007FF724EB156F  shl          rax, 20h
00007FF724EB1573  or           r15, rax
00007FF724EB1576  or           r12, rcx
00007FF724EB1579  cmp          rdi, qword ptr [rsp+50h]
00007FF724EB157E  je           static void Fabled_Engine::main+3E9h (00007FF724EB15F9h)
00007FF724EB1580  punpckhqdq   xmm6, xmm6
00007FF724EB1584  movd         eax, xmm6
00007FF724EB1588  punpckhqdq   xmm9, xmm9
00007FF724EB158D  movd         ecx, xmm9
00007FF724EB1592  movd         edx, xmm7
00007FF724EB1596  shl          rdi, 6h
00007FF724EB159A  mov          qword ptr [rbx+rdi*1], rbp
00007FF724EB159E  mov          dword ptr [rbx+rdi*1+8h], eax
00007FF724EB15A2  movss        dword ptr [rbx+rdi*1+Ch], xmm8
00007FF724EB15A9  mov          dword ptr [rbx+rdi*1+10h], 0h
00007FF724EB15B1  mov          qword ptr [rbx+rdi*1+14h], rsi
00007FF724EB15B6  mov          dword ptr [rbx+rdi*1+1Ch], ecx
00007FF724EB15BA  mov          qword ptr [rbx+rdi*1+28h], r14
00007FF724EB15BF  mov          qword ptr [rbx+rdi*1+20h], rdx
00007FF724EB15C4  mov          qword ptr [rbx+rdi*1+38h], r12
00007FF724EB15C9  mov          qword ptr [rbx+rdi*1+30h], r15
00007FF724EB15CE  mov          rdi, qword ptr [rsp+58h]
00007FF724EB15D3  add          rdi, 1h
00007FF724EB15D7  mov          qword ptr [rsp+58h], rdi
00007FF724EB15DC  cmp          r13, 2h
00007FF724EB15E0  mov          rax, qword ptr [rsp+98h]
00007FF724EB15E8  ja           static void Fabled_Engine::main+405h (00007FF724EB1615h)
00007FF724EB15EA  mov          r13, rax
00007FF724EB15ED  cmp          rax, 4h
00007FF724EB15F1  jb           static void Fabled_Engine::main+1F0h (00007FF724EB1400h)
00007FF724EB15F7  jmp          static void Fabled_Engine::main+405h (00007FF724EB1615h)
00007FF724EB15F9  lea          rcx, [rsp+48h]
00007FF724EB15FE  mov          rdx, rdi
00007FF724EB1601  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF724ECADA0h)
00007FF724EB1606  mov          rbx, qword ptr [rsp+48h]
00007FF724EB160B  mov          rdi, qword ptr [rsp+58h]
00007FF724EB1610  jmp          static void Fabled_Engine::main+370h (00007FF724EB1580h)
00007FF724EB1615  mov          esi, 3h
00007FF724EB161A  mov          rdx, qword ptr [rsp+40h]
00007FF724EB161F  lea          r12, [rsp+30h]
00007FF724EB1624  jmp          static void Fabled_Engine::main+43Bh (00007FF724EB164Bh)
00007FF724EB1626  nop          word ptr cs:[rax+rax*1], ax
00007FF724EB1630  mov          qword ptr [rax+rdx*8], rsi
00007FF724EB1634  add          rdx, 1h
00007FF724EB1638  mov          qword ptr [rsp+40h], rdx
00007FF724EB163D  add          rsi, 1h
00007FF724EB1641  cmp          rsi, 6h
00007FF724EB1645  je           static void Fabled_Engine::main+56Dh (00007FF724EB177Dh)
00007FF724EB164B  cmp          rdx, qword ptr [rsp+38h]
00007FF724EB1650  je           static void Fabled_Engine::main+504h (00007FF724EB1714h)
00007FF724EB1656  mov          rax, qword ptr [rsp+30h]
00007FF724EB165B  mov          qword ptr [rax+rdx*8], 0h
00007FF724EB1663  mov          rdx, qword ptr [rsp+40h]
00007FF724EB1668  add          rdx, 1h
00007FF724EB166C  mov          qword ptr [rsp+40h], rdx
00007FF724EB1671  cmp          rdx, qword ptr [rsp+38h]
00007FF724EB1676  je           static void Fabled_Engine::main+516h (00007FF724EB1726h)
00007FF724EB167C  lea          rbx, [rsi-1h]
00007FF724EB1680  mov          qword ptr [rax+rdx*8], rsi
00007FF724EB1684  mov          rdx, qword ptr [rsp+40h]
00007FF724EB1689  add          rdx, 1h
00007FF724EB168D  mov          qword ptr [rsp+40h], rdx
00007FF724EB1692  cmp          rdx, qword ptr [rsp+38h]
00007FF724EB1697  je           static void Fabled_Engine::main+52Dh (00007FF724EB173Dh)
00007FF724EB169D  mov          qword ptr [rax+rdx*8], rbx
00007FF724EB16A1  mov          rdx, qword ptr [rsp+40h]
00007FF724EB16A6  add          rdx, 1h
00007FF724EB16AA  mov          qword ptr [rsp+40h], rdx
00007FF724EB16AF  cmp          rdx, qword ptr [rsp+38h]
00007FF724EB16B4  je           static void Fabled_Engine::main+544h (00007FF724EB1754h)
00007FF724EB16BA  mov          rax, qword ptr [rsp+30h]
00007FF724EB16BF  mov          qword ptr [rax+rdx*8], 1h
00007FF724EB16C7  mov          rdx, qword ptr [rsp+40h]
00007FF724EB16CC  add          rdx, 1h
00007FF724EB16D0  mov          qword ptr [rsp+40h], rdx
00007FF724EB16D5  cmp          rdx, qword ptr [rsp+38h]
00007FF724EB16DA  je           static void Fabled_Engine::main+556h (00007FF724EB1766h)
00007FF724EB16E0  mov          qword ptr [rax+rdx*8], rbx
00007FF724EB16E4  mov          rdx, qword ptr [rsp+40h]
00007FF724EB16E9  add          rdx, 1h
00007FF724EB16ED  mov          qword ptr [rsp+40h], rdx
00007FF724EB16F2  cmp          rdx, qword ptr [rsp+38h]
00007FF724EB16F7  jne          static void Fabled_Engine::main+420h (00007FF724EB1630h)
00007FF724EB16FD  mov          rcx, r12
00007FF724EB1700  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF724ECAE60h)
00007FF724EB1705  mov          rax, qword ptr [rsp+30h]
00007FF724EB170A  mov          rdx, qword ptr [rsp+40h]
00007FF724EB170F  jmp          static void Fabled_Engine::main+420h (00007FF724EB1630h)
00007FF724EB1714  mov          rcx, r12
00007FF724EB1717  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF724ECAE60h)
00007FF724EB171C  mov          rdx, qword ptr [rsp+40h]
00007FF724EB1721  jmp          static void Fabled_Engine::main+446h (00007FF724EB1656h)
00007FF724EB1726  mov          rcx, r12
00007FF724EB1729  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF724ECAE60h)
00007FF724EB172E  mov          rax, qword ptr [rsp+30h]
00007FF724EB1733  mov          rdx, qword ptr [rsp+40h]
00007FF724EB1738  jmp          static void Fabled_Engine::main+46Ch (00007FF724EB167Ch)
00007FF724EB173D  mov          rcx, r12
00007FF724EB1740  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF724ECAE60h)
00007FF724EB1745  mov          rax, qword ptr [rsp+30h]
00007FF724EB174A  mov          rdx, qword ptr [rsp+40h]
00007FF724EB174F  jmp          static void Fabled_Engine::main+48Dh (00007FF724EB169Dh)
00007FF724EB1754  mov          rcx, r12
00007FF724EB1757  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF724ECAE60h)
00007FF724EB175C  mov          rdx, qword ptr [rsp+40h]
00007FF724EB1761  jmp          static void Fabled_Engine::main+4AAh (00007FF724EB16BAh)
00007FF724EB1766  mov          rcx, r12
00007FF724EB1769  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF724ECAE60h)
00007FF724EB176E  mov          rax, qword ptr [rsp+30h]
00007FF724EB1773  mov          rdx, qword ptr [rsp+40h]
00007FF724EB1778  jmp          static void Fabled_Engine::main+4D0h (00007FF724EB16E0h)
00007FF724EB177D  mov          rax, qword ptr [rsp+40h]
00007FF724EB1782  mov          qword ptr [rsp+7Ch], rax
00007FF724EB1787  movups       xmm0, xmmword ptr [rsp+30h]
00007FF724EB178C  movups       xmmword ptr [rsp+6Ch], xmm0
00007FF724EB1791  mov          rcx, qword ptr [00007FF724ED31C8h]
00007FF724EB1798  test         rcx, rcx
00007FF724EB179B  jne          static void Fabled_Engine::main+5A5h (00007FF724EB17B5h)
00007FF724EB179D  call         GetProcessHeap (00007FF724EC925Ch)
00007FF724EB17A2  test         rax, rax
00007FF724EB17A5  je           static void Fabled_Engine::main+8A4h (00007FF724EB1AB4h)
00007FF724EB17AB  mov          rcx, rax
00007FF724EB17AE  mov          qword ptr [00007FF724ED31C8h], rax
00007FF724EB17B5  mov          r8d, 40h
00007FF724EB17BB  xor          edx, edx
00007FF724EB17BD  call         HeapAlloc (00007FF724EC9262h)
00007FF724EB17C2  test         rax, rax
00007FF724EB17C5  je           static void Fabled_Engine::main+8A4h (00007FF724EB1AB4h)
00007FF724EB17CB  mov          rbx, rax
00007FF724EB17CE  mov          rax, qword ptr [rsp+58h]
00007FF724EB17D3  mov          qword ptr [rbx+10h], rax
00007FF724EB17D7  movups       xmm0, xmmword ptr [rsp+48h]
00007FF724EB17DC  movups       xmmword ptr [rbx], xmm0
00007FF724EB17DF  movups       xmm0, xmmword ptr [rsp+68h]
00007FF724EB17E4  movups       xmm1, xmmword ptr [rsp+74h]
00007FF724EB17E9  movups       xmmword ptr [rbx+1Ch], xmm0
00007FF724EB17ED  movups       xmmword ptr [rbx+28h], xmm1
00007FF724EB17F1  mov          rax, rbx
00007FF724EB17F4  add          rax, 18h
00007FF724EB17F8  mov          dword ptr [rbx+18h], 0h