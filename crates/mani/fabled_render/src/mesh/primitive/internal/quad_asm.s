# --------------- Quad Dissassembly -------------------
00007FF6E1AB137F  movaps       xmm1, xmmword ptr [__xmm@bf00000000000000bf0000003f000000 (00007FF6E1ACC520h)]
00007FF6E1AB1386  movaps       xmmword ptr [rsp+40h], xmm1
00007FF6E1AB138B  movaps       xmm1, xmmword ptr [__xmm@bf000000bf000000000000003f000000 (00007FF6E1ACC530h)]
00007FF6E1AB1392  movaps       xmmword ptr [rsp+50h], xmm1
00007FF6E1AB1397  movaps       xmm1, xmmword ptr [__xmm@000000003f0000003f00000000000000 (00007FF6E1ACC540h)]
00007FF6E1AB139E  movaps       xmmword ptr [rsp+60h], xmm1
00007FF6E1AB13A3  movss        xmm2, dword ptr [rsp+44h]
00007FF6E1AB13A9  movss        xmm3, dword ptr [rsp+4Ch]
00007FF6E1AB13AF  movss        xmm1, dword ptr [__real@3f000000 (00007FF6E1ACC550h)]
00007FF6E1AB13B7  mov          dword ptr [rsp+70h], 3F000000h
00007FF6E1AB13BF  movss        dword ptr [rsp+74h], xmm2
00007FF6E1AB13C5  addss        xmm2, xmm1
00007FF6E1AB13C9  mov          rax, 3F80000000000000h
00007FF6E1AB13D3  mov          qword ptr [rsp+78h], rax
00007FF6E1AB13D8  movss        dword ptr [rsp+80h], xmm2
00007FF6E1AB13E1  mov          eax, dword ptr [00007FF6E1ACDAA8h]
00007FF6E1AB13E7  mov          dword ptr [rsp+8Ch], eax
00007FF6E1AB13EE  mov          rcx, qword ptr [00007FF6E1ACDAA0h]
00007FF6E1AB13F5  mov          qword ptr [rsp+84h], rcx
00007FF6E1AB13FD  movaps       xmmword ptr [rsp+A0h], xmm0
00007FF6E1AB1405  movaps       xmmword ptr [rsp+90h], xmm0
00007FF6E1AB140D  movss        xmm2, dword ptr [rsp+50h]
00007FF6E1AB1413  movss        dword ptr [rsp+B0h], xmm3
00007FF6E1AB141C  addss        xmm3, xmm1
00007FF6E1AB1420  movss        dword ptr [rsp+B4h], xmm2
00007FF6E1AB1429  addss        xmm2, xmm1
00007FF6E1AB142D  mov          dword ptr [rsp+B8h], 0h
00007FF6E1AB1438  movss        dword ptr [rsp+BCh], xmm3
00007FF6E1AB1441  movss        dword ptr [rsp+C0h], xmm2
00007FF6E1AB144A  mov          dword ptr [rsp+CCh], eax
00007FF6E1AB1451  mov          qword ptr [rsp+C4h], rcx
00007FF6E1AB1459  movaps       xmmword ptr [rsp+E0h], xmm0
00007FF6E1AB1461  movaps       xmmword ptr [rsp+D0h], xmm0
00007FF6E1AB1469  movss        xmm2, dword ptr [rsp+58h]
00007FF6E1AB146F  movss        xmm3, dword ptr [rsp+5Ch]
00007FF6E1AB1475  movss        dword ptr [rsp+F0h], xmm2
00007FF6E1AB147E  addss        xmm2, xmm1
00007FF6E1AB1482  movss        dword ptr [rsp+F4h], xmm3
00007FF6E1AB148B  addss        xmm3, xmm1
00007FF6E1AB148F  mov          dword ptr [rsp+F8h], 0h
00007FF6E1AB149A  movss        dword ptr [rsp+FCh], xmm2
00007FF6E1AB14A3  movss        dword ptr [rsp+100h], xmm3
00007FF6E1AB14AC  mov          dword ptr [rsp+10Ch], eax
00007FF6E1AB14B3  mov          qword ptr [rsp+104h], rcx
00007FF6E1AB14BB  movaps       xmmword ptr [rsp+120h], xmm0
00007FF6E1AB14C3  movaps       xmmword ptr [rsp+110h], xmm0
00007FF6E1AB14CB  movss        xmm2, dword ptr [rsp+64h]
00007FF6E1AB14D1  movss        xmm3, dword ptr [rsp+68h]
00007FF6E1AB14D7  movss        dword ptr [rsp+130h], xmm2
00007FF6E1AB14E0  movss        dword ptr [rsp+134h], xmm3
00007FF6E1AB14E9  addss        xmm3, xmm1
00007FF6E1AB14ED  addss        xmm1, xmm2
00007FF6E1AB14F1  mov          dword ptr [rsp+138h], 0h
00007FF6E1AB14FC  movss        dword ptr [rsp+13Ch], xmm1
00007FF6E1AB1505  movss        dword ptr [rsp+140h], xmm3
00007FF6E1AB150E  mov          dword ptr [rsp+14Ch], eax
00007FF6E1AB1515  mov          qword ptr [rsp+144h], rcx
00007FF6E1AB151D  movaps       xmmword ptr [rsp+160h], xmm0
00007FF6E1AB1525  movaps       xmmword ptr [rsp+150h], xmm0
00007FF6E1AB152D  mov          rcx, qword ptr [00007FF6E1AD31C8h]
00007FF6E1AB1534  test         rcx, rcx
00007FF6E1AB1537  jne          static void Fabled_Engine::main+291h (00007FF6E1AB1551h)
00007FF6E1AB1539  call         GetProcessHeap (00007FF6E1AC8BE2h)
00007FF6E1AB153E  test         rax, rax
00007FF6E1AB1541  je           static void Fabled_Engine::main+5B4h (00007FF6E1AB1874h)
00007FF6E1AB1547  mov          rcx, rax
00007FF6E1AB154A  mov          qword ptr [00007FF6E1AD31C8h], rax
00007FF6E1AB1551  mov          r8d, 100h
00007FF6E1AB1557  xor          edx, edx
00007FF6E1AB1559  call         HeapAlloc (00007FF6E1AC8BE8h)
00007FF6E1AB155E  test         rax, rax
00007FF6E1AB1561  je           static void Fabled_Engine::main+5B4h (00007FF6E1AB1874h)
00007FF6E1AB1567  mov          rdi, rax
00007FF6E1AB156A  lea          rdx, [rsp+70h]
00007FF6E1AB156F  mov          r8d, 100h
00007FF6E1AB1575  mov          rcx, rax
00007FF6E1AB1578  call         memcpy (00007FF6E1AC9CC7h)
00007FF6E1AB157D  mov          rcx, qword ptr [00007FF6E1AD31C8h]
00007FF6E1AB1584  test         rcx, rcx
00007FF6E1AB1587  jne          static void Fabled_Engine::main+2E1h (00007FF6E1AB15A1h)
00007FF6E1AB1589  call         GetProcessHeap (00007FF6E1AC8BE2h)
00007FF6E1AB158E  test         rax, rax
00007FF6E1AB1591  je           static void Fabled_Engine::main+5BBh (00007FF6E1AB187Bh)
00007FF6E1AB1597  mov          rcx, rax
00007FF6E1AB159A  mov          qword ptr [00007FF6E1AD31C8h], rax
00007FF6E1AB15A1  mov          r8d, 30h
00007FF6E1AB15A7  xor          edx, edx
00007FF6E1AB15A9  call         HeapAlloc (00007FF6E1AC8BE8h)
00007FF6E1AB15AE  test         rax, rax
00007FF6E1AB15B1  je           static void Fabled_Engine::main+5BBh (00007FF6E1AB187Bh)
00007FF6E1AB15B7  mov          rbx, rax
00007FF6E1AB15BA  movups       xmm0, xmmword ptr [00007FF6E1ACDA90h]
00007FF6E1AB15C1  movups       xmmword ptr [rax+20h], xmm0
00007FF6E1AB15C5  movups       xmm0, xmmword ptr [00007FF6E1ACDA80h]
00007FF6E1AB15CC  movups       xmmword ptr [rax+10h], xmm0
00007FF6E1AB15D0  movups       xmm0, xmmword ptr [00007FF6E1ACDA70h]
00007FF6E1AB15D7  movups       xmmword ptr [rax], xmm0
00007FF6E1AB15DA  mov          rcx, qword ptr [00007FF6E1AD31C8h]
00007FF6E1AB15E1  test         rcx, rcx
00007FF6E1AB15E4  jne          static void Fabled_Engine::main+33Eh (00007FF6E1AB15FEh)
00007FF6E1AB15E6  call         GetProcessHeap (00007FF6E1AC8BE2h)
00007FF6E1AB15EB  test         rax, rax
00007FF6E1AB15EE  je           static void Fabled_Engine::main+5CCh (00007FF6E1AB188Ch)
00007FF6E1AB15F4  mov          rcx, rax
00007FF6E1AB15F7  mov          qword ptr [00007FF6E1AD31C8h], rax
00007FF6E1AB15FE  mov          r8d, 40h
00007FF6E1AB1604  xor          edx, edx
00007FF6E1AB1606  call         HeapAlloc (00007FF6E1AC8BE8h)
00007FF6E1AB160B  test         rax, rax
00007FF6E1AB160E  je           static void Fabled_Engine::main+5CCh (00007FF6E1AB188Ch)
00007FF6E1AB1614  mov          rsi, rax
00007FF6E1AB1617  mov          qword ptr [rax], rdi
00007FF6E1AB161A  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF6E1ACC510h)]
00007FF6E1AB1621  movups       xmmword ptr [rax+8h], xmm0
00007FF6E1AB1625  mov          dword ptr [rax+18h], 0h
00007FF6E1AB162C  mov          qword ptr [rax+20h], rbx
00007FF6E1AB1630  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF6E1ACC560h)]
00007FF6E1AB1637  movups       xmmword ptr [rax+28h], xmm0
00007FF6E1AB163B  lea          rax, [00007FF6E1ACFAF0h]
00007FF6E1AB1642  mov          qword ptr [rsp+170h], rax
00007FF6E1AB164A  mov          qword ptr [rsp+178h], 6h