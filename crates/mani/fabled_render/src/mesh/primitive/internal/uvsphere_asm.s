# --------------- UvSphere Dissassembly -------------------
00007FF7800E1359  mov          rcx, qword ptr [00007FF7801031C8h]
00007FF7800E1360  test         rcx, rcx
00007FF7800E1363  jne          static void Fabled_Engine::main+BDh (00007FF7800E137Dh)
00007FF7800E1365  call         GetProcessHeap (00007FF7800F8F82h)
00007FF7800E136A  test         rax, rax
00007FF7800E136D  je           static void Fabled_Engine::main+7BBh (00007FF7800E1A7Bh)
00007FF7800E1373  mov          rcx, rax
00007FF7800E1376  mov          qword ptr [00007FF7801031C8h], rax
00007FF7800E137D  mov          r8d, 8000h
00007FF7800E1383  xor          edx, edx
00007FF7800E1385  call         HeapAlloc (00007FF7800F8F88h)
00007FF7800E138A  test         rax, rax
00007FF7800E138D  je           static void Fabled_Engine::main+7BBh (00007FF7800E1A7Bh)
00007FF7800E1393  mov          qword ptr [rsp+50h], rax
00007FF7800E1398  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000200 (00007FF7800FC530h)]
00007FF7800E139F  movups       xmmword ptr [rsp+58h], xmm0
00007FF7800E13A4  mov          rcx, qword ptr [00007FF7801031C8h]
00007FF7800E13AB  test         rcx, rcx
00007FF7800E13AE  jne          static void Fabled_Engine::main+108h (00007FF7800E13C8h)
00007FF7800E13B0  call         GetProcessHeap (00007FF7800F8F82h)
00007FF7800E13B5  test         rax, rax
00007FF7800E13B8  je           static void Fabled_Engine::main+7C2h (00007FF7800E1A82h)
00007FF7800E13BE  mov          rcx, rax
00007FF7800E13C1  mov          qword ptr [00007FF7801031C8h], rax
00007FF7800E13C8  mov          r8d, 6000h
00007FF7800E13CE  xor          edx, edx
00007FF7800E13D0  call         HeapAlloc (00007FF7800F8F88h)
00007FF7800E13D5  test         rax, rax
00007FF7800E13D8  je           static void Fabled_Engine::main+7C2h (00007FF7800E1A82h)
00007FF7800E13DE  mov          qword ptr [rsp+30h], rax
00007FF7800E13E3  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000c00 (00007FF7800FC540h)]
00007FF7800E13EA  movups       xmmword ptr [rsp+38h], xmm0
00007FF7800E13EF  xor          esi, esi
00007FF7800E13F1  xorps        xmm15, xmm15
00007FF7800E13F5  lea          r12, [rsp+50h]
00007FF7800E13FA  jmp          static void Fabled_Engine::main+14Dh (00007FF7800E140Dh)
00007FF7800E13FC  nop          dword ptr [rax], eax
00007FF7800E1400  cmp          rsi, 20h
00007FF7800E1404  mov          rsi, rbx
00007FF7800E1407  je           static void Fabled_Engine::main+2F8h (00007FF7800E15B8h)
00007FF7800E140D  test         rsi, rsi
00007FF7800E1410  js           static void Fabled_Engine::main+160h (00007FF7800E1420h)
00007FF7800E1412  xorps        xmm11, xmm11
00007FF7800E1416  cvtsi2ss     xmm11, rsi
00007FF7800E141B  jmp          static void Fabled_Engine::main+17Ch (00007FF7800E143Ch)
00007FF7800E141D  nop          dword ptr [rax], eax
00007FF7800E1420  mov          rax, rsi
00007FF7800E1423  shr          rax, 1h
00007FF7800E1426  mov          ecx, esi
00007FF7800E1428  and          ecx, 1h
00007FF7800E142B  or           rcx, rax
00007FF7800E142E  xorps        xmm11, xmm11
00007FF7800E1432  cvtsi2ss     xmm11, rcx
00007FF7800E1437  addss        xmm11, xmm11
00007FF7800E143C  lea          rbx, [rsi+1h]
00007FF7800E1440  movaps       xmm6, xmm11
00007FF7800E1444  mulss        xmm6, dword ptr [__real@bdc90fdb (00007FF7800FC550h)]
00007FF7800E144C  addss        xmm6, dword ptr [__real@3fc90fdb (00007FF7800FC554h)]
00007FF7800E1454  movaps       xmm0, xmm6
00007FF7800E1457  call         cosf (00007FF7800FA0A9h)
00007FF7800E145C  movaps       xmm9, xmm0
00007FF7800E1460  movss        xmm7, dword ptr [__real@3f000000 (00007FF7800FC558h)]
00007FF7800E1468  mulss        xmm9, xmm7
00007FF7800E146D  movaps       xmm0, xmm6
00007FF7800E1470  call         sinf (00007FF7800FA0AFh)
00007FF7800E1475  movaps       xmm13, xmm0
00007FF7800E1479  mulss        xmm13, xmm7
00007FF7800E147E  mulss        xmm11, dword ptr [__real@3d000000 (00007FF7800FC55Ch)]
00007FF7800E1487  movaps       xmm8, xmm13
00007FF7800E148B  addss        xmm8, xmm13
00007FF7800E1490  mov          rdi, qword ptr [rsp+60h]
00007FF7800E1495  xor          ebp, ebp
00007FF7800E1497  jmp          static void Fabled_Engine::main+2B9h (00007FF7800E1579h)
00007FF7800E149C  nop          dword ptr [rax], eax
00007FF7800E14A0  xorps        xmm12, xmm12
00007FF7800E14A4  cvtsi2ss     xmm12, rbp
00007FF7800E14A9  movaps       xmm6, xmm12
00007FF7800E14AD  mulss        xmm6, dword ptr [__real@3ec90fdb (00007FF7800FC560h)]
00007FF7800E14B5  movaps       xmm0, xmm6
00007FF7800E14B8  call         cosf (00007FF7800FA0A9h)
00007FF7800E14BD  movaps       xmm7, xmm0
00007FF7800E14C0  mulss        xmm7, xmm9
00007FF7800E14C5  movaps       xmm0, xmm6
00007FF7800E14C8  call         sinf (00007FF7800FA0AFh)
00007FF7800E14CD  movaps       xmm6, xmm0
00007FF7800E14D0  movaps       xmm14, xmm7
00007FF7800E14D4  addss        xmm14, xmm7
00007FF7800E14D9  movaps       xmmword ptr [rsp+70h], xmm15
00007FF7800E14DF  add          rbp, 1h
00007FF7800E14E3  mulss        xmm12, dword ptr [__real@3d800000 (00007FF7800FC564h)]
00007FF7800E14EC  movaps       xmmword ptr [rsp+A0h], xmm15
00007FF7800E14F5  mulss        xmm6, xmm9
00007FF7800E14FA  movaps       xmm10, xmm6
00007FF7800E14FE  addss        xmm10, xmm6
00007FF7800E1503  cmp          rdi, qword ptr [rsp+58h]
00007FF7800E1508  je           static void Fabled_Engine::main+2E3h (00007FF7800E15A3h)
00007FF7800E150E  mov          rax, qword ptr [rsp+50h]
00007FF7800E1513  mov          rcx, rdi
00007FF7800E1516  shl          rcx, 6h
00007FF7800E151A  movss        dword ptr [rax+rcx*1], xmm7
00007FF7800E151F  movss        dword ptr [rax+rcx*1+4h], xmm6
00007FF7800E1525  movss        dword ptr [rax+rcx*1+8h], xmm13
00007FF7800E152C  movss        dword ptr [rax+rcx*1+Ch], xmm12
00007FF7800E1533  movss        dword ptr [rax+rcx*1+10h], xmm11
00007FF7800E153A  movss        dword ptr [rax+rcx*1+14h], xmm14
00007FF7800E1541  movss        dword ptr [rax+rcx*1+18h], xmm10
00007FF7800E1548  movss        dword ptr [rax+rcx*1+1Ch], xmm8
00007FF7800E154F  movaps       xmm0, xmmword ptr [rsp+70h]
00007FF7800E1554  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF7800E1559  movaps       xmm0, xmmword ptr [rsp+A0h]
00007FF7800E1561  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF7800E1566  add          rdi, 1h
00007FF7800E156A  mov          qword ptr [rsp+60h], rdi
00007FF7800E156F  cmp          rbp, 11h
00007FF7800E1573  je           static void Fabled_Engine::main+140h (00007FF7800E1400h)
00007FF7800E1579  test         rbp, rbp
00007FF7800E157C  jns          static void Fabled_Engine::main+1E0h (00007FF7800E14A0h)
00007FF7800E1582  mov          rax, rbp
00007FF7800E1585  shr          rax, 1h
00007FF7800E1588  mov          ecx, ebp
00007FF7800E158A  and          ecx, 1h
00007FF7800E158D  or           rcx, rax
00007FF7800E1590  xorps        xmm12, xmm12
00007FF7800E1594  cvtsi2ss     xmm12, rcx
00007FF7800E1599  addss        xmm12, xmm12
00007FF7800E159E  jmp          static void Fabled_Engine::main+1E9h (00007FF7800E14A9h)
00007FF7800E15A3  mov          rcx, r12
00007FF7800E15A6  mov          rdx, rdi
00007FF7800E15A9  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF7800FAAC0h)
00007FF7800E15AE  mov          rdi, qword ptr [rsp+60h]
00007FF7800E15B3  jmp          static void Fabled_Engine::main+24Eh (00007FF7800E150Eh)
00007FF7800E15B8  mov          r13d, 12h
00007FF7800E15BE  xor          eax, eax
00007FF7800E15C0  mov          qword ptr [rsp+68h], rax
00007FF7800E15C5  jmp          static void Fabled_Engine::main+320h (00007FF7800E15E0h)
00007FF7800E15C7  nop          word ptr [rax+rax*1], ax
00007FF7800E15D0  add          r13, 11h
00007FF7800E15D4  cmp          qword ptr [rsp+68h], 20h
00007FF7800E15DA  je           static void Fabled_Engine::main+4B6h (00007FF7800E1776h)
00007FF7800E15E0  mov          r15, qword ptr [rsp+68h]
00007FF7800E15E5  lea          rax, [r15+1h]
00007FF7800E15E9  mov          qword ptr [rsp+68h], rax
00007FF7800E15EE  mov          rsi, r15
00007FF7800E15F1  shl          rsi, 4h
00007FF7800E15F5  add          rsi, r15
00007FF7800E15F8  mov          ebx, Fh
00007FF7800E15FD  mov          rbp, r13
00007FF7800E1600  jmp          static void Fabled_Engine::main+364h (00007FF7800E1624h)
00007FF7800E1602  nop          word ptr cs:[rax+rax*1], ax
00007FF7800E160C  nop          dword ptr [rax], eax
00007FF7800E1610  mov          rsi, rdi
00007FF7800E1613  test         rbx, rbx
00007FF7800E1616  je           static void Fabled_Engine::main+4B6h (00007FF7800E1776h)
00007FF7800E161C  add          rbp, 1h
00007FF7800E1620  add          rbx, FFFFFFFFFFFFFFFFh
00007FF7800E1624  lea          r14, [rbp-1h]
00007FF7800E1628  test         r15, r15
00007FF7800E162B  je           static void Fabled_Engine::main+3DEh (00007FF7800E169Eh)
00007FF7800E162D  mov          rdx, qword ptr [rsp+40h]
00007FF7800E1632  cmp          rdx, qword ptr [rsp+38h]
00007FF7800E1637  je           static void Fabled_Engine::main+47Ah (00007FF7800E173Ah)
00007FF7800E163D  mov          rax, qword ptr [rsp+30h]
00007FF7800E1642  mov          qword ptr [rax+rdx*8], rsi
00007FF7800E1646  mov          rdx, qword ptr [rsp+40h]
00007FF7800E164B  add          rdx, 1h
00007FF7800E164F  mov          qword ptr [rsp+40h], rdx
00007FF7800E1654  cmp          rdx, qword ptr [rsp+38h]
00007FF7800E1659  je           static void Fabled_Engine::main+48Eh (00007FF7800E174Eh)
00007FF7800E165F  mov          rax, qword ptr [rsp+30h]
00007FF7800E1664  mov          qword ptr [rax+rdx*8], r14
00007FF7800E1668  mov          rdx, qword ptr [rsp+40h]
00007FF7800E166D  add          rdx, 1h
00007FF7800E1671  mov          qword ptr [rsp+40h], rdx
00007FF7800E1676  lea          rdi, [rsi+1h]
00007FF7800E167A  cmp          rdx, qword ptr [rsp+38h]
00007FF7800E167F  je           static void Fabled_Engine::main+4A2h (00007FF7800E1762h)
00007FF7800E1685  mov          rax, qword ptr [rsp+30h]
00007FF7800E168A  mov          qword ptr [rax+rdx*8], rdi
00007FF7800E168E  add          qword ptr [rsp+40h], 1h
00007FF7800E1694  cmp          r15, 1Fh
00007FF7800E1698  je           static void Fabled_Engine::main+350h (00007FF7800E1610h)
00007FF7800E169E  add          rsi, 1h
00007FF7800E16A2  mov          rdx, qword ptr [rsp+40h]
00007FF7800E16A7  cmp          rdx, qword ptr [rsp+38h]
00007FF7800E16AC  je           static void Fabled_Engine::main+447h (00007FF7800E1707h)
00007FF7800E16AE  mov          rax, qword ptr [rsp+30h]
00007FF7800E16B3  mov          qword ptr [rax+rdx*8], rsi
00007FF7800E16B7  mov          rdx, qword ptr [rsp+40h]
00007FF7800E16BC  add          rdx, 1h
00007FF7800E16C0  mov          qword ptr [rsp+40h], rdx
00007FF7800E16C5  cmp          rdx, qword ptr [rsp+38h]
00007FF7800E16CA  je           static void Fabled_Engine::main+458h (00007FF7800E1718h)
00007FF7800E16CC  mov          rax, qword ptr [rsp+30h]
00007FF7800E16D1  mov          qword ptr [rax+rdx*8], r14
00007FF7800E16D5  mov          rdx, qword ptr [rsp+40h]
00007FF7800E16DA  add          rdx, 1h
00007FF7800E16DE  mov          qword ptr [rsp+40h], rdx
00007FF7800E16E3  cmp          rdx, qword ptr [rsp+38h]
00007FF7800E16E8  je           static void Fabled_Engine::main+469h (00007FF7800E1729h)
00007FF7800E16EA  mov          rax, qword ptr [rsp+30h]
00007FF7800E16EF  mov          qword ptr [rax+rdx*8], rbp
00007FF7800E16F3  add          qword ptr [rsp+40h], 1h
00007FF7800E16F9  test         rbx, rbx
00007FF7800E16FC  jne          static void Fabled_Engine::main+35Ch (00007FF7800E161Ch)
00007FF7800E1702  jmp          static void Fabled_Engine::main+310h (00007FF7800E15D0h)
00007FF7800E1707  lea          rcx, [rsp+30h]
00007FF7800E170C  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7800FAB80h)
00007FF7800E1711  mov          rdx, qword ptr [rsp+40h]
00007FF7800E1716  jmp          static void Fabled_Engine::main+3EEh (00007FF7800E16AEh)
00007FF7800E1718  lea          rcx, [rsp+30h]
00007FF7800E171D  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7800FAB80h)
00007FF7800E1722  mov          rdx, qword ptr [rsp+40h]
00007FF7800E1727  jmp          static void Fabled_Engine::main+40Ch (00007FF7800E16CCh)
00007FF7800E1729  lea          rcx, [rsp+30h]
00007FF7800E172E  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7800FAB80h)
00007FF7800E1733  mov          rdx, qword ptr [rsp+40h]
00007FF7800E1738  jmp          static void Fabled_Engine::main+42Ah (00007FF7800E16EAh)
00007FF7800E173A  lea          rcx, [rsp+30h]
00007FF7800E173F  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7800FAB80h)
00007FF7800E1744  mov          rdx, qword ptr [rsp+40h]
00007FF7800E1749  jmp          static void Fabled_Engine::main+37Dh (00007FF7800E163Dh)
00007FF7800E174E  lea          rcx, [rsp+30h]
00007FF7800E1753  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7800FAB80h)
00007FF7800E1758  mov          rdx, qword ptr [rsp+40h]
00007FF7800E175D  jmp          static void Fabled_Engine::main+39Fh (00007FF7800E165Fh)
00007FF7800E1762  lea          rcx, [rsp+30h]
00007FF7800E1767  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7800FAB80h)
00007FF7800E176C  mov          rdx, qword ptr [rsp+40h]
00007FF7800E1771  jmp          static void Fabled_Engine::main+3C5h (00007FF7800E1685h)
00007FF7800E1776  mov          rax, qword ptr [rsp+40h]
00007FF7800E177B  mov          qword ptr [rsp+84h], rax
00007FF7800E1783  movups       xmm0, xmmword ptr [rsp+30h]
00007FF7800E1788  movups       xmmword ptr [rsp+74h], xmm0
00007FF7800E178D  mov          rcx, qword ptr [00007FF7801031C8h]
00007FF7800E1794  test         rcx, rcx
00007FF7800E1797  jne          static void Fabled_Engine::main+4F1h (00007FF7800E17B1h)
00007FF7800E1799  call         GetProcessHeap (00007FF7800F8F82h)
00007FF7800E179E  test         rax, rax
00007FF7800E17A1  je           static void Fabled_Engine::main+7D3h (00007FF7800E1A93h)
00007FF7800E17A7  mov          rcx, rax
00007FF7800E17AA  mov          qword ptr [00007FF7801031C8h], rax
00007FF7800E17B1  mov          r8d, 40h
00007FF7800E17B7  xor          edx, edx
00007FF7800E17B9  call         HeapAlloc (00007FF7800F8F88h)
00007FF7800E17BE  test         rax, rax
00007FF7800E17C1  je           static void Fabled_Engine::main+7D3h (00007FF7800E1A93h)
00007FF7800E17C7  mov          rbx, rax
00007FF7800E17CA  mov          rax, qword ptr [rsp+60h]
00007FF7800E17CF  mov          qword ptr [rbx+10h], rax
00007FF7800E17D3  movups       xmm0, xmmword ptr [rsp+50h]
00007FF7800E17D8  movups       xmmword ptr [rbx], xmm0
00007FF7800E17DB  movups       xmm0, xmmword ptr [rsp+70h]
00007FF7800E17E0  movups       xmm1, xmmword ptr [rsp+7Ch]
00007FF7800E17E5  movups       xmmword ptr [rbx+1Ch], xmm0
00007FF7800E17E9  movups       xmmword ptr [rbx+28h], xmm1
00007FF7800E17ED  mov          dword ptr [rbx+18h], 0h
00007FF7800E17F4  lea          rax, [00007FF7800FFAB0h]
00007FF7800E17FB  mov          qword ptr [rsp+A0h], rax
00007FF7800E1803  mov          qword ptr [rsp+A8h], 6h