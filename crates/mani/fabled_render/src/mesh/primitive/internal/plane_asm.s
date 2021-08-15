# --------------- Cube Dissassembly -------------------
//    let plane = fabled_render::mesh::Plane::default();
00007FF736AD1359  movss        xmm0, dword ptr [__real@41200000 (00007FF736AEC520h)]
00007FF736AD1361  minss        xmm0, xmm0
00007FF736AD1365  xorps        xmm1, xmm1
00007FF736AD1368  maxss        xmm1, xmm0
00007FF736AD136C  movss        xmm0, dword ptr [__real@437f0000 (00007FF736AEC524h)]
00007FF736AD1374  minss        xmm0, xmm1
00007FF736AD1378  cvttss2si    esi, xmm0

//    let plane_model: fabled_render::mesh::Model = plane.into();
00007FF736AD137C  lea          r14d, [rsi+1h]
00007FF736AD1380  mov          eax, esi
00007FF736AD1382  mul          sil
00007FF736AD1385  mov          ebp, eax
00007FF736AD1387  mov          eax, r14d
00007FF736AD138A  mul          r14b
00007FF736AD138D  movzx        edi, al
00007FF736AD1390  test         dil, dil
00007FF736AD1393  je           static void Fabled_Engine::main+11Ah (00007FF736AD13DAh)
00007FF736AD1395  mov          rbx, rdi
00007FF736AD1398  shl          rbx, 6h
00007FF736AD139C  mov          rcx, qword ptr [00007FF736AF31C8h]
00007FF736AD13A3  test         rcx, rcx
00007FF736AD13A6  jne          static void Fabled_Engine::main+FCh (00007FF736AD13BCh)
00007FF736AD13A8  call         GetProcessHeap (00007FF736AE8F42h)
00007FF736AD13AD  test         rax, rax
00007FF736AD13B0  je           static void Fabled_Engine::main+10Bh (00007FF736AD13CBh)
00007FF736AD13B2  mov          rcx, rax
00007FF736AD13B5  mov          qword ptr [00007FF736AF31C8h], rax
00007FF736AD13BC  xor          edx, edx
00007FF736AD13BE  mov          r8, rbx
00007FF736AD13C1  call         HeapAlloc (00007FF736AE8F48h)
00007FF736AD13C6  test         rax, rax
00007FF736AD13C9  jne          static void Fabled_Engine::main+11Fh (00007FF736AD13DFh)
00007FF736AD13CB  mov          edx, 10h
00007FF736AD13D0  mov          rcx, rbx
00007FF736AD13D3  call         static void alloc::alloc::handle_alloc_error (00007FF736AEA150h)
00007FF736AD13D8  ud2
00007FF736AD13DA  mov          eax, 10h
00007FF736AD13DF  movzx        ecx, bpl
00007FF736AD13E3  add          ecx, ecx
00007FF736AD13E5  lea          ecx, [rcx+rcx*2]
00007FF736AD13E8  movzx        ebx, cl
00007FF736AD13EB  mov          qword ptr [rsp+48h], rax
00007FF736AD13F0  mov          qword ptr [rsp+50h], rdi
00007FF736AD13F5  mov          qword ptr [rsp+58h], 0h
00007FF736AD13FE  test         cl, cl
00007FF736AD1400  je           static void Fabled_Engine::main+18Bh (00007FF736AD144Bh)
00007FF736AD1402  lea          rbp, [rbx*8]
00007FF736AD140A  mov          rcx, qword ptr [00007FF736AF31C8h]
00007FF736AD1411  test         rcx, rcx
00007FF736AD1414  jne          static void Fabled_Engine::main+16Ah (00007FF736AD142Ah)
00007FF736AD1416  call         GetProcessHeap (00007FF736AE8F42h)
00007FF736AD141B  test         rax, rax
00007FF736AD141E  je           static void Fabled_Engine::main+17Ch (00007FF736AD143Ch)
00007FF736AD1420  mov          rcx, rax
00007FF736AD1423  mov          qword ptr [00007FF736AF31C8h], rax
00007FF736AD142A  xor          edx, edx
00007FF736AD142C  mov          r8, rbp
00007FF736AD142F  call         HeapAlloc (00007FF736AE8F48h)
00007FF736AD1434  mov          r15, rax
00007FF736AD1437  test         rax, rax
00007FF736AD143A  jne          static void Fabled_Engine::main+191h (00007FF736AD1451h)
00007FF736AD143C  mov          edx, 8h
00007FF736AD1441  mov          rcx, rbp
00007FF736AD1444  call         static void alloc::alloc::handle_alloc_error (00007FF736AEA150h)
00007FF736AD1449  ud2
00007FF736AD144B  mov          r15d, 8h
00007FF736AD1451  mov          qword ptr [rsp+28h], r15
00007FF736AD1456  mov          qword ptr [rsp+30h], rbx
00007FF736AD145B  mov          qword ptr [rsp+38h], 0h
00007FF736AD1464  test         r14b, r14b
00007FF736AD1467  je           static void Fabled_Engine::main+2E3h (00007FF736AD15A3h)
00007FF736AD146D  movzx        eax, sil
00007FF736AD1471  xorps        xmm0, xmm0
00007FF736AD1474  cvtsi2ss     xmm0, eax
00007FF736AD1478  movss        xmm9, dword ptr [__real@3f800000 (00007FF736AEC528h)]
00007FF736AD1481  movaps       xmm13, xmm9
00007FF736AD1485  divss        xmm13, xmm0
00007FF736AD148A  xorps        xmm0, xmm0
00007FF736AD148D  xor          edx, edx
00007FF736AD148F  movss        xmm10, dword ptr [__real@bf000000 (00007FF736AEC52Ch)]
00007FF736AD1498  lea          r13, [rsp+48h]
00007FF736AD149D  mov          ebp, dword ptr [00007FF736AEDA38h]
00007FF736AD14A3  mov          rdi, qword ptr [00007FF736AEDA30h]
00007FF736AD14AA  movaps       xmm11, xmmword ptr [00007FF736AEDA40h]
00007FF736AD14B2  movaps       xmm12, xmmword ptr [00007FF736AEDA50h]
00007FF736AD14BA  xor          r12d, r12d
00007FF736AD14BD  jmp          static void Fabled_Engine::main+216h (00007FF736AD14D6h)
00007FF736AD14BF  nop
00007FF736AD14C0  addss        xmm0, xmm9
00007FF736AD14C5  lea          eax, [r12+1h]
00007FF736AD14CA  cmp          r12b, sil
00007FF736AD14CD  mov          r12d, eax
00007FF736AD14D0  je           static void Fabled_Engine::main+2DAh (00007FF736AD159Ah)
00007FF736AD14D6  movaps       xmm15, xmm13
00007FF736AD14DA  mulss        xmm15, xmm0
00007FF736AD14DF  movaps       xmm14, xmm15
00007FF736AD14E3  addss        xmm14, xmm10
00007FF736AD14E8  mov          bl, FFh
00007FF736AD14EA  xorps        xmm6, xmm6
00007FF736AD14ED  jmp          static void Fabled_Engine::main+28Bh (00007FF736AD154Bh)
00007FF736AD14EF  nop
00007FF736AD14F0  mov          rcx, rdx
00007FF736AD14F3  mov          rdx, qword ptr [rsp+48h]
00007FF736AD14F8  shl          rcx, 6h
00007FF736AD14FC  movss        dword ptr [rdx+rcx*1], xmm8
00007FF736AD1502  mov          dword ptr [rdx+rcx*1+4h], 0h
00007FF736AD150A  movss        dword ptr [rdx+rcx*1+8h], xmm14
00007FF736AD1511  movss        dword ptr [rdx+rcx*1+Ch], xmm7
00007FF736AD1517  movss        dword ptr [rdx+rcx*1+10h], xmm15
00007FF736AD151E  mov          dword ptr [rdx+rcx*1+1Ch], ebp
00007FF736AD1522  mov          qword ptr [rdx+rcx*1+14h], rdi
00007FF736AD1527  movaps       xmmword ptr [rdx+rcx*1+20h], xmm11
00007FF736AD152D  movaps       xmmword ptr [rdx+rcx*1+30h], xmm12
00007FF736AD1533  mov          rdx, rax
00007FF736AD1536  add          rdx, 1h
00007FF736AD153A  mov          qword ptr [rsp+58h], rdx
00007FF736AD153F  add          bl, 1h
00007FF736AD1542  cmp          sil, bl
00007FF736AD1545  je           static void Fabled_Engine::main+200h (00007FF736AD14C0h)
00007FF736AD154B  movaps       xmm7, xmm13
00007FF736AD154F  mulss        xmm7, xmm6
00007FF736AD1553  addss        xmm6, xmm9
00007FF736AD1558  movaps       xmm8, xmm7
00007FF736AD155C  addss        xmm8, xmm10
00007FF736AD1561  mov          rax, qword ptr [rsp+58h]
00007FF736AD1566  cmp          rdx, qword ptr [rsp+50h]
00007FF736AD156B  jne          static void Fabled_Engine::main+230h (00007FF736AD14F0h)
00007FF736AD156D  mov          rcx, rax
00007FF736AD1570  cmp          rdx, rax
00007FF736AD1573  jne          static void Fabled_Engine::main+233h (00007FF736AD14F3h)
00007FF736AD1579  mov          rcx, r13
00007FF736AD157C  movss        dword ptr [rsp+60h], xmm0
00007FF736AD1582  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF736AEAA70h)
00007FF736AD1587  movss        xmm0, dword ptr [rsp+60h]
00007FF736AD158D  mov          rax, qword ptr [rsp+58h]
00007FF736AD1592  mov          rcx, rax
00007FF736AD1595  jmp          static void Fabled_Engine::main+233h (00007FF736AD14F3h)
00007FF736AD159A  test         sil, sil
00007FF736AD159D  je           static void Fabled_Engine::main+48Dh (00007FF736AD174Dh)
00007FF736AD15A3  movzx        eax, sil
00007FF736AD15A7  mov          qword ptr [rsp+98h], rax
00007FF736AD15AF  xor          ebx, ebx
00007FF736AD15B1  xor          edx, edx
00007FF736AD15B3  xor          eax, eax
00007FF736AD15B5  jmp          static void Fabled_Engine::main+311h (00007FF736AD15D1h)
00007FF736AD15B7  nop          word ptr [rax+rax*1], ax
00007FF736AD15C0  add          bl, r14b
00007FF736AD15C3  mov          rax, qword ptr [rsp+60h]
00007FF736AD15C8  cmp          al, sil
00007FF736AD15CB  je           static void Fabled_Engine::main+48Dh (00007FF736AD174Dh)
00007FF736AD15D1  add          al, 1h
00007FF736AD15D3  mov          qword ptr [rsp+60h], rax
00007FF736AD15D8  test         sil, sil
00007FF736AD15DB  je           static void Fabled_Engine::main+300h (00007FF736AD15C0h)
00007FF736AD15DD  mov          rbp, qword ptr [rsp+98h]
00007FF736AD15E5  mov          r13d, ebx
00007FF736AD15E8  jmp          static void Fabled_Engine::main+347h (00007FF736AD1607h)
00007FF736AD15EA  nop          word ptr [rax+rax*1], ax
00007FF736AD15F0  mov          qword ptr [r15+rdx*8], r12
00007FF736AD15F4  add          rdx, 1h
00007FF736AD15F8  mov          qword ptr [rsp+38h], rdx
00007FF736AD15FD  add          r13b, 1h
00007FF736AD1601  add          rbp, FFFFFFFFFFFFFFFFh
00007FF736AD1605  je           static void Fabled_Engine::main+300h (00007FF736AD15C0h)
00007FF736AD1607  cmp          rdx, qword ptr [rsp+30h]
00007FF736AD160C  je           static void Fabled_Engine::main+41Ah (00007FF736AD16DAh)
00007FF736AD1612  lea          edi, [r14+r13*1]
00007FF736AD1616  movzx        r12d, r13b
00007FF736AD161A  mov          qword ptr [r15+rdx*8], r12
00007FF736AD161E  mov          rdx, qword ptr [rsp+38h]
00007FF736AD1623  add          rdx, 1h
00007FF736AD1627  mov          qword ptr [rsp+38h], rdx
00007FF736AD162C  cmp          rdx, qword ptr [rsp+30h]
00007FF736AD1631  je           static void Fabled_Engine::main+433h (00007FF736AD16F3h)
00007FF736AD1637  movzx        edi, dil
00007FF736AD163B  mov          rax, qword ptr [rsp+28h]
00007FF736AD1640  mov          qword ptr [rax+rdx*8], rdi
00007FF736AD1644  mov          rdx, qword ptr [rsp+38h]
00007FF736AD1649  add          rdx, 1h
00007FF736AD164D  mov          qword ptr [rsp+38h], rdx
00007FF736AD1652  add          r12, 1h
00007FF736AD1656  cmp          rdx, qword ptr [rsp+30h]
00007FF736AD165B  je           static void Fabled_Engine::main+447h (00007FF736AD1707h)
00007FF736AD1661  mov          qword ptr [rax+rdx*8], r12
00007FF736AD1665  mov          rdx, qword ptr [rsp+38h]
00007FF736AD166A  add          rdx, 1h
00007FF736AD166E  mov          qword ptr [rsp+38h], rdx
00007FF736AD1673  cmp          rdx, qword ptr [rsp+30h]
00007FF736AD1678  je           static void Fabled_Engine::main+460h (00007FF736AD1720h)
00007FF736AD167E  mov          qword ptr [rax+rdx*8], rdi
00007FF736AD1682  mov          rdx, qword ptr [rsp+38h]
00007FF736AD1687  add          rdx, 1h
00007FF736AD168B  mov          qword ptr [rsp+38h], rdx
00007FF736AD1690  add          rdi, 1h
00007FF736AD1694  cmp          rdx, qword ptr [rsp+30h]
00007FF736AD1699  je           static void Fabled_Engine::main+479h (00007FF736AD1739h)
00007FF736AD169F  mov          r15, qword ptr [rsp+28h]
00007FF736AD16A4  mov          qword ptr [r15+rdx*8], rdi
00007FF736AD16A8  mov          rdx, qword ptr [rsp+38h]
00007FF736AD16AD  add          rdx, 1h
00007FF736AD16B1  mov          qword ptr [rsp+38h], rdx
00007FF736AD16B6  cmp          rdx, qword ptr [rsp+30h]
00007FF736AD16BB  jne          static void Fabled_Engine::main+330h (00007FF736AD15F0h)
00007FF736AD16C1  lea          rcx, [rsp+28h]
00007FF736AD16C6  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF736AEAB30h)
00007FF736AD16CB  mov          r15, qword ptr [rsp+28h]
00007FF736AD16D0  mov          rdx, qword ptr [rsp+38h]
00007FF736AD16D5  jmp          static void Fabled_Engine::main+330h (00007FF736AD15F0h)
00007FF736AD16DA  lea          rcx, [rsp+28h]
00007FF736AD16DF  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF736AEAB30h)
    [rsp+28h]
00007FF736AD16E9  mov          rdx, qword ptr [rsp+38h]
00007FF736AD16EE  jmp          static void Fabled_Engine::main+352h (00007FF736AD1612h)
00007FF736AD16F3  lea          rcx, [rsp+28h]
00007FF736AD16F8  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF736AEAB30h)
00007FF736AD16FD  mov          rdx, qword ptr [rsp+38h]
00007FF736AD1702  jmp          static void Fabled_Engine::main+377h (00007FF736AD1637h)
00007FF736AD1707  lea          rcx, [rsp+28h]
00007FF736AD170C  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF736AEAB30h)
00007FF736AD1711  mov          rax, qword ptr [rsp+28h]
00007FF736AD1716  mov          rdx, qword ptr [rsp+38h]
00007FF736AD171B  jmp          static void Fabled_Engine::main+3A1h (00007FF736AD1661h)
00007FF736AD1720  lea          rcx, [rsp+28h]
00007FF736AD1725  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF736AEAB30h)
00007FF736AD172A  mov          rax, qword ptr [rsp+28h]
00007FF736AD172F  mov          rdx, qword ptr [rsp+38h]
00007FF736AD1734  jmp          static void Fabled_Engine::main+3BEh (00007FF736AD167Eh)
00007FF736AD1739  lea          rcx, [rsp+28h]
00007FF736AD173E  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF736AEAB30h)
00007FF736AD1743  mov          rdx, qword ptr [rsp+38h]
00007FF736AD1748  jmp          static void Fabled_Engine::main+3DFh (00007FF736AD169Fh)
00007FF736AD174D  mov          rax, qword ptr [rsp+38h]
00007FF736AD1752  mov          qword ptr [rsp+7Ch], rax
00007FF736AD1757  movups       xmm0, xmmword ptr [rsp+28h]
00007FF736AD175C  movups       xmmword ptr [rsp+6Ch], xmm0
00007FF736AD1761  mov          rcx, qword ptr [00007FF736AF31C8h]
00007FF736AD1768  test         rcx, rcx
00007FF736AD176B  jne          static void Fabled_Engine::main+4C5h (00007FF736AD1785h)
00007FF736AD176D  call         GetProcessHeap (00007FF736AE8F42h)
00007FF736AD1772  test         rax, rax
00007FF736AD1775  je           static void Fabled_Engine::main+794h (00007FF736AD1A54h)
00007FF736AD177B  mov          rcx, rax
00007FF736AD177E  mov          qword ptr [00007FF736AF31C8h], rax
00007FF736AD1785  mov          r8d, 40h
00007FF736AD178B  xor          edx, edx
00007FF736AD178D  call         HeapAlloc (00007FF736AE8F48h)
00007FF736AD1792  test         rax, rax
00007FF736AD1795  je           static void Fabled_Engine::main+794h (00007FF736AD1A54h)
00007FF736AD179B  mov          rsi, rax
00007FF736AD179E  mov          rax, qword ptr [rsp+58h]
00007FF736AD17A3  mov          qword ptr [rsi+10h], rax
00007FF736AD17A7  movups       xmm0, xmmword ptr [rsp+48h]
00007FF736AD17AC  movups       xmmword ptr [rsi], xmm0
00007FF736AD17AF  movups       xmm0, xmmword ptr [rsp+68h]
00007FF736AD17B4  movups       xmm1, xmmword ptr [rsp+74h]
00007FF736AD17B9  movups       xmmword ptr [rsi+1Ch], xmm0
00007FF736AD17BD  movups       xmmword ptr [rsi+28h], xmm1
00007FF736AD17C1  mov          dword ptr [rsi+18h], 0h
00007FF736AD17C8  lea          rax, [00007FF736AEFAA0h]
00007FF736AD17CF  mov          qword ptr [rsp+A0h], rax
00007FF736AD17D7  mov          qword ptr [rsp+A8h], 6h