# --------------- Cube Dissassembly -------------------
00007FF79ECD1348  mov          qword ptr [rsp+C0h], 0h
00007FF79ECD1354  mov          dword ptr [rsp+C8h], 0h
00007FF79ECD135F  mov          qword ptr [rsp+D0h], 0h
00007FF79ECD136B  mov          dword ptr [rsp+D8h], 0h
00007FF79ECD1376  xorps        xmm0, xmm0
00007FF79ECD1379  movaps       xmmword ptr [rsp+E0h], xmm0
00007FF79ECD1381  movaps       xmmword ptr [rsp+100h], xmm0
00007FF79ECD1389  mov          rcx, qword ptr [00007FF79ECF31C8h]
00007FF79ECD1390  test         rcx, rcx
00007FF79ECD1393  jne          static void Fabled_Engine::main+EDh (00007FF79ECD13ADh)
00007FF79ECD1395  call         GetProcessHeap (00007FF79ECE8F32h)
00007FF79ECD139A  test         rax, rax
00007FF79ECD139D  je           static void Fabled_Engine::main+8FBh (00007FF79ECD1BBBh)
00007FF79ECD13A3  mov          rcx, rax
00007FF79ECD13A6  mov          qword ptr [00007FF79ECF31C8h], rax
00007FF79ECD13AD  mov          r8d, 600h
00007FF79ECD13B3  xor          edx, edx
00007FF79ECD13B5  call         HeapAlloc (00007FF79ECE8F38h)
00007FF79ECD13BA  test         rax, rax
00007FF79ECD13BD  je           static void Fabled_Engine::main+8FBh (00007FF79ECD1BBBh)
00007FF79ECD13C3  mov          qword ptr [rsp+40h], rax
00007FF79ECD13C8  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000018 (00007FF79ECEC520h)]
00007FF79ECD13CF  movups       xmmword ptr [rsp+48h], xmm0
00007FF79ECD13D4  mov          rcx, qword ptr [rsp+C0h]
00007FF79ECD13DC  mov          qword ptr [rsp+60h], rcx
00007FF79ECD13E1  mov          ecx, dword ptr [rsp+C8h]
00007FF79ECD13E8  mov          dword ptr [rsp+68h], ecx
00007FF79ECD13EC  mov          rcx, qword ptr [rsp+D0h]
00007FF79ECD13F4  mov          qword ptr [rsp+70h], rcx
00007FF79ECD13F9  mov          ecx, dword ptr [rsp+D8h]
00007FF79ECD1400  mov          dword ptr [rsp+78h], ecx
00007FF79ECD1404  movaps       xmm0, xmmword ptr [rsp+E0h]
00007FF79ECD140C  movaps       xmmword ptr [rsp+80h], xmm0
00007FF79ECD1414  movaps       xmm0, xmmword ptr [rsp+100h]
00007FF79ECD141C  movaps       xmmword ptr [rsp+30h], xmm0
00007FF79ECD1421  xor          ecx, ecx
00007FF79ECD1423  nop          word ptr cs:[rax+rax*1], ax
00007FF79ECD142D  nop          dword ptr [rax], eax
00007FF79ECD1430  mov          edx, dword ptr [rsp+68h]
00007FF79ECD1434  mov          dword ptr [rax+rcx*1+8h], edx
00007FF79ECD1438  mov          rdx, qword ptr [rsp+60h]
00007FF79ECD143D  mov          qword ptr [rax+rcx*1], rdx
00007FF79ECD1441  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF79ECD144A  mov          edx, dword ptr [rsp+78h]
00007FF79ECD144E  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF79ECD1452  mov          rdx, qword ptr [rsp+70h]
00007FF79ECD1457  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF79ECD145C  movaps       xmm0, xmmword ptr [rsp+80h]
00007FF79ECD1464  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF79ECD1469  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF79ECD146E  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF79ECD1473  add          rcx, 40h
00007FF79ECD1477  cmp          rcx, 5C0h
00007FF79ECD147E  jne          static void Fabled_Engine::main+170h (00007FF79ECD1430h)
00007FF79ECD1480  mov          edx, dword ptr [rsp+68h]
00007FF79ECD1484  mov          dword ptr [rax+rcx*1+8h], edx
00007FF79ECD1488  mov          rdx, qword ptr [rsp+60h]
00007FF79ECD148D  mov          qword ptr [rax+rcx*1], rdx
00007FF79ECD1491  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF79ECD149A  mov          edx, dword ptr [rsp+78h]
00007FF79ECD149E  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF79ECD14A2  mov          rdx, qword ptr [rsp+70h]
00007FF79ECD14A7  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF79ECD14AC  movaps       xmm0, xmmword ptr [rsp+80h]
00007FF79ECD14B4  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF79ECD14B9  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF79ECD14BE  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF79ECD14C3  mov          qword ptr [rsp+50h], 18h
00007FF79ECD14CC  mov          rcx, qword ptr [00007FF79ECF31C8h]
00007FF79ECD14D3  test         rcx, rcx
00007FF79ECD14D6  jne          static void Fabled_Engine::main+230h (00007FF79ECD14F0h)
00007FF79ECD14D8  call         GetProcessHeap (00007FF79ECE8F32h)
00007FF79ECD14DD  test         rax, rax
00007FF79ECD14E0  je           static void Fabled_Engine::main+902h (00007FF79ECD1BC2h)
00007FF79ECD14E6  mov          rcx, rax
00007FF79ECD14E9  mov          qword ptr [00007FF79ECF31C8h], rax
00007FF79ECD14F0  mov          r8d, 120h
00007FF79ECD14F6  mov          edx, 8h
00007FF79ECD14FB  call         HeapAlloc (00007FF79ECE8F38h)
00007FF79ECD1500  test         rax, rax
00007FF79ECD1503  je           static void Fabled_Engine::main+902h (00007FF79ECD1BC2h)
00007FF79ECD1509  mov          r14, rax
00007FF79ECD150C  mov          rdx, qword ptr [rsp+50h]
00007FF79ECD1511  mov          eax, E0h
00007FF79ECD1516  add          rax, qword ptr [rsp+40h]
00007FF79ECD151B  mov          qword ptr [rsp+F8h], r14
00007FF79ECD1523  add          r14, 28h
00007FF79ECD1527  xor          r9d, r9d
00007FF79ECD152A  lea          r8, [00007FF79ECEDA50h]
00007FF79ECD1531  xorps        xmm8, xmm8
00007FF79ECD1535  mov          rcx, BF800000BF800000h
00007FF79ECD153F  movq         xmm9, rcx
00007FF79ECD1544  mov          rcx, 3F8000003F800000h
00007FF79ECD154E  movq         xmm10, rcx
00007FF79ECD1553  movss        xmm11, dword ptr [__real@3f000000 (00007FF79ECEC530h)]
00007FF79ECD155C  mov          r12, rdx
00007FF79ECD155F  xor          ecx, ecx
00007FF79ECD1561  nop          word ptr cs:[rax+rax*1], ax
00007FF79ECD156B  nop          dword ptr [rax+rax*1], eax
00007FF79ECD1570  mov          rbp, r9
00007FF79ECD1573  movaps       xmm0, xmmword ptr [r8+rcx*4]
00007FF79ECD1578  movaps       xmm1, xmmword ptr [r8+rcx*4+60h]
00007FF79ECD157E  movaps       xmm2, xmmword ptr [r8+rcx*4+C0h]
00007FF79ECD1587  movaps       xmm3, xmm0
00007FF79ECD158A  subps        xmm3, xmm2
00007FF79ECD158D  movaps       xmm4, xmm1
00007FF79ECD1590  addps        xmm4, xmm3
00007FF79ECD1593  subps        xmm3, xmm1
00007FF79ECD1596  addps        xmm2, xmm0
00007FF79ECD1599  movaps       xmm5, xmm1
00007FF79ECD159C  addps        xmm5, xmm2
00007FF79ECD159F  subps        xmm2, xmm1
00007FF79ECD15A2  movaps       xmmword ptr [rsp+80h], xmm3
00007FF79ECD15AA  movaps       xmmword ptr [rsp+90h], xmm4
00007FF79ECD15B2  movaps       xmmword ptr [rsp+A0h], xmm5
00007FF79ECD15BA  movaps       xmmword ptr [rsp+B0h], xmm2
00007FF79ECD15C2  lea          rbx, [r9+1h]
00007FF79ECD15C6  lea          rdi, [r9+2h]
00007FF79ECD15CA  add          r9, 3h
00007FF79ECD15CE  mov          qword ptr [r14-28h], rbp
00007FF79ECD15D2  mov          qword ptr [r14-20h], rbx
00007FF79ECD15D6  mov          qword ptr [r14-18h], rdi
00007FF79ECD15DA  mov          qword ptr [r14-10h], rdi
00007FF79ECD15DE  mov          qword ptr [r14-8h], r9
00007FF79ECD15E2  mov          qword ptr [r14], rbp
00007FF79ECD15E5  cmp          rdx, rcx
00007FF79ECD15E8  jb           static void Fabled_Engine::main+8D3h (00007FF79ECD1B93h)
00007FF79ECD15EE  cmp          r12, 3h
00007FF79ECD15F2  jbe          static void Fabled_Engine::main+8E1h (00007FF79ECD1BA1h)
00007FF79ECD15F8  movd         r13d, xmm0
00007FF79ECD15FD  movaps       xmm1, xmm0
00007FF79ECD1600  shufps       xmm1, xmm0, 55h
00007FF79ECD1604  movd         r11d, xmm1
00007FF79ECD1609  punpckhqdq   xmm0, xmm0
00007FF79ECD160D  movd         r10d, xmm0
00007FF79ECD1612  movaps       xmm3, xmmword ptr [rsp+80h]
00007FF79ECD161A  movaps       xmm0, xmmword ptr [rsp+90h]
00007FF79ECD1622  movaps       xmm2, xmmword ptr [rsp+A0h]
00007FF79ECD162A  movaps       xmm13, xmmword ptr [rsp+B0h]
00007FF79ECD1633  xorps        xmm4, xmm4
00007FF79ECD1636  cmpps        xmm4, xmm3, 2h
00007FF79ECD163A  movaps       xmm5, xmm4
00007FF79ECD163D  andnps       xmm5, xmm9
00007FF79ECD1641  andps        xmm4, xmm10
00007FF79ECD1645  orps         xmm4, xmm5
00007FF79ECD1648  movaps       xmm7, xmm3
00007FF79ECD164B  cmpps        xmm7, xmm3, 3h
00007FF79ECD164F  movaps       xmm5, xmm7
00007FF79ECD1652  andnps       xmm5, xmm4
00007FF79ECD1655  andps        xmm7, xmm3
00007FF79ECD1658  orps         xmm7, xmm5
00007FF79ECD165B  xorps        xmm4, xmm4
00007FF79ECD165E  cmpps        xmm4, xmm0, 2h
00007FF79ECD1662  movaps       xmm5, xmm4
00007FF79ECD1665  andnps       xmm5, xmm9
00007FF79ECD1669  andps        xmm4, xmm10
00007FF79ECD166D  orps         xmm4, xmm5
00007FF79ECD1670  movaps       xmm6, xmm0
00007FF79ECD1673  cmpps        xmm6, xmm0, 3h
00007FF79ECD1677  movaps       xmm5, xmm6
00007FF79ECD167A  andnps       xmm5, xmm4
00007FF79ECD167D  andps        xmm6, xmm0
00007FF79ECD1680  orps         xmm6, xmm5
00007FF79ECD1683  xorps        xmm4, xmm4
00007FF79ECD1686  cmpps        xmm4, xmm2, 2h
00007FF79ECD168A  movaps       xmm5, xmm4
00007FF79ECD168D  andnps       xmm5, xmm9
00007FF79ECD1691  andps        xmm4, xmm10
00007FF79ECD1695  orps         xmm4, xmm5
00007FF79ECD1698  movaps       xmm1, xmm2
00007FF79ECD169B  cmpps        xmm1, xmm2, 3h
00007FF79ECD169F  movaps       xmm5, xmm1
00007FF79ECD16A2  andnps       xmm5, xmm4
00007FF79ECD16A5  andps        xmm1, xmm2
00007FF79ECD16A8  orps         xmm1, xmm5
00007FF79ECD16AB  xorps        xmm5, xmm5
00007FF79ECD16AE  cmpps        xmm5, xmm13, 2h
00007FF79ECD16B3  movaps       xmm4, xmm5
00007FF79ECD16B6  andnps       xmm4, xmm9
00007FF79ECD16BA  andps        xmm5, xmm10
00007FF79ECD16BE  orps         xmm5, xmm4
00007FF79ECD16C1  movaps       xmm4, xmm13
00007FF79ECD16C5  cmpps        xmm4, xmm13, 3h
00007FF79ECD16CA  movaps       xmm12, xmm4
00007FF79ECD16CE  andnps       xmm12, xmm5
00007FF79ECD16D2  movaps       xmm5, xmm3
00007FF79ECD16D5  shufps       xmm5, xmm3, 55h
00007FF79ECD16D9  movd         ebx, xmm5
00007FF79ECD16DD  movd         ebp, xmm3
00007FF79ECD16E1  punpckhqdq   xmm3, xmm3
00007FF79ECD16E5  movd         r15d, xmm3
00007FF79ECD16EA  movaps       xmm3, xmm0
00007FF79ECD16ED  shufps       xmm3, xmm0, 55h
00007FF79ECD16F1  movd         edi, xmm3
00007FF79ECD16F5  shl          rbx, 20h
00007FF79ECD16F9  or           rbp, rbx
00007FF79ECD16FC  movd         ebx, xmm0
00007FF79ECD1700  punpckhqdq   xmm0, xmm0
00007FF79ECD1704  mov          qword ptr [rax-E0h], rbp
00007FF79ECD170B  movd         ebp, xmm0
00007FF79ECD170F  movaps       xmm0, xmm2
00007FF79ECD1712  shufps       xmm0, xmm2, 55h
00007FF79ECD1716  mov          dword ptr [rax-D8h], r15d
00007FF79ECD171D  movd         esi, xmm0
00007FF79ECD1721  pshufd       xmm0, xmm7, 55h
00007FF79ECD1726  mulss        xmm7, xmm11
00007FF79ECD172B  addss        xmm7, xmm11
00007FF79ECD1730  movss        dword ptr [rax-D4h], xmm7
00007FF79ECD1738  mulss        xmm0, xmm11
00007FF79ECD173D  addss        xmm0, xmm11
00007FF79ECD1742  movss        dword ptr [rax-D0h], xmm0
00007FF79ECD174A  shl          r11, 20h
00007FF79ECD174E  or           r13, r11
00007FF79ECD1751  shl          rdi, 20h
00007FF79ECD1755  or           rbx, rdi
00007FF79ECD1758  movd         edi, xmm2
00007FF79ECD175C  punpckhqdq   xmm2, xmm2
00007FF79ECD1760  mov          qword ptr [rax-CCh], r13
00007FF79ECD1767  mov          dword ptr [rax-C4h], r10d
00007FF79ECD176E  movaps       xmmword ptr [rax-C0h], xmm8
00007FF79ECD1776  movaps       xmmword ptr [rax-B0h], xmm8
00007FF79ECD177E  mov          qword ptr [rax-A0h], rbx
00007FF79ECD1785  movd         ebx, xmm2
00007FF79ECD1789  movaps       xmm0, xmm13
00007FF79ECD178D  shufps       xmm0, xmm13, 55h
00007FF79ECD1792  mov          dword ptr [rax-98h], ebp
00007FF79ECD1798  movd         ebp, xmm0
00007FF79ECD179C  pshufd       xmm0, xmm6, 55h
00007FF79ECD17A1  mulss        xmm6, xmm11
00007FF79ECD17A6  addss        xmm6, xmm11
00007FF79ECD17AB  movss        dword ptr [rax-94h], xmm6
00007FF79ECD17B3  mulss        xmm0, xmm11
00007FF79ECD17B8  addss        xmm0, xmm11
00007FF79ECD17BD  movss        dword ptr [rax-90h], xmm0
00007FF79ECD17C5  shl          rsi, 20h
00007FF79ECD17C9  or           rdi, rsi
00007FF79ECD17CC  andps        xmm4, xmm13
00007FF79ECD17D0  mov          qword ptr [rax-8Ch], r13
00007FF79ECD17D7  mov          dword ptr [rax-84h], r10d
00007FF79ECD17DE  movaps       xmmword ptr [rax-80h], xmm8
00007FF79ECD17E3  movaps       xmmword ptr [rax-70h], xmm8
00007FF79ECD17E8  mov          qword ptr [rax-60h], rdi
00007FF79ECD17EC  movd         edi, xmm13
00007FF79ECD17F1  punpckhqdq   xmm13, xmm13
00007FF79ECD17F6  mov          dword ptr [rax-58h], ebx
00007FF79ECD17F9  movd         ebx, xmm13
00007FF79ECD17FE  pshufd       xmm0, xmm1, 55h
00007FF79ECD1803  mulss        xmm1, xmm11
00007FF79ECD1808  addss        xmm1, xmm11
00007FF79ECD180D  movss        dword ptr [rax-54h], xmm1
00007FF79ECD1812  mulss        xmm0, xmm11
00007FF79ECD1817  addss        xmm0, xmm11
00007FF79ECD181C  movss        dword ptr [rax-50h], xmm0
00007FF79ECD1821  shl          rbp, 20h
00007FF79ECD1825  or           rdi, rbp
00007FF79ECD1828  mov          qword ptr [rax-4Ch], r13
00007FF79ECD182C  mov          dword ptr [rax-44h], r10d
00007FF79ECD1830  movaps       xmmword ptr [rax-40h], xmm8
00007FF79ECD1835  movaps       xmmword ptr [rax-30h], xmm8
00007FF79ECD183A  mov          dword ptr [rax-18h], ebx
00007FF79ECD183D  mov          qword ptr [rax-20h], rdi
00007FF79ECD1841  orps         xmm4, xmm12
00007FF79ECD1845  pshufd       xmm0, xmm4, 55h
00007FF79ECD184A  mulss        xmm4, xmm11
00007FF79ECD184F  addss        xmm4, xmm11
00007FF79ECD1854  movss        dword ptr [rax-14h], xmm4
00007FF79ECD1859  mulss        xmm0, xmm11
00007FF79ECD185E  addss        xmm0, xmm11
00007FF79ECD1863  movss        dword ptr [rax-10h], xmm0
00007FF79ECD1868  mov          dword ptr [rax-4h], r10d
00007FF79ECD186C  mov          qword ptr [rax-Ch], r13
00007FF79ECD1870  movaps       xmmword ptr [rax], xmm8
00007FF79ECD1874  movaps       xmmword ptr [rax+10h], xmm8
00007FF79ECD1879  add          rcx, 4h
00007FF79ECD187D  add          rax, 100h
00007FF79ECD1883  add          r12, FFFFFFFFFFFFFFFCh
00007FF79ECD1887  add          r14, 30h
00007FF79ECD188B  cmp          r9, 12h
00007FF79ECD188F  jne          static void Fabled_Engine::main+2B0h (00007FF79ECD1570h)
00007FF79ECD1895  mov          rcx, qword ptr [00007FF79ECF31C8h]
00007FF79ECD189C  test         rcx, rcx
00007FF79ECD189F  jne          static void Fabled_Engine::main+5F9h (00007FF79ECD18B9h)
00007FF79ECD18A1  call         GetProcessHeap (00007FF79ECE8F32h)
00007FF79ECD18A6  test         rax, rax
00007FF79ECD18A9  je           static void Fabled_Engine::main+913h (00007FF79ECD1BD3h)
00007FF79ECD18AF  mov          rcx, rax
00007FF79ECD18B2  mov          qword ptr [00007FF79ECF31C8h], rax
00007FF79ECD18B9  mov          r8d, 40h
00007FF79ECD18BF  xor          edx, edx
00007FF79ECD18C1  call         HeapAlloc (00007FF79ECE8F38h)
00007FF79ECD18C6  test         rax, rax
00007FF79ECD18C9  je           static void Fabled_Engine::main+913h (00007FF79ECD1BD3h)
00007FF79ECD18CF  mov          rdi, rax
00007FF79ECD18D2  mov          rax, qword ptr [rsp+50h]
00007FF79ECD18D7  mov          qword ptr [rdi+10h], rax
00007FF79ECD18DB  movups       xmm0, xmmword ptr [rsp+40h]
00007FF79ECD18E0  movups       xmmword ptr [rdi], xmm0
00007FF79ECD18E3  mov          rax, qword ptr [rsp+F8h]
00007FF79ECD18EB  mov          qword ptr [rdi+20h], rax
00007FF79ECD18EF  movaps       xmm0, xmmword ptr [__xmm@00000000000000240000000000000024 (00007FF79ECEC540h)]
00007FF79ECD18F6  movups       xmmword ptr [rdi+28h], xmm0
00007FF79ECD18FA  mov          dword ptr [rdi+18h], 0h
00007FF79ECD1901  lea          rax, [00007FF79ECEFC18h]
00007FF79ECD1908  mov          qword ptr [rsp+E0h], rax
00007FF79ECD1910  mov          qword ptr [rsp+E8h], 6h