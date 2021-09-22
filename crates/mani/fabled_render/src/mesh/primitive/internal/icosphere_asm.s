# --------------- IcoSphere Dissassembly -------------------
00007FF6F0EE1335  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE133C  test         rcx, rcx
00007FF6F0EE133F  jne          static void Fabled_Engine::main+99h (00007FF6F0EE1359h)
00007FF6F0EE1341  call         GetProcessHeap (00007FF6F0EF98C2h)
00007FF6F0EE1346  test         rax, rax
00007FF6F0EE1349  je           static void Fabled_Engine::main+ACDh (00007FF6F0EE1D8Dh)
00007FF6F0EE134F  mov          rcx, rax
00007FF6F0EE1352  mov          qword ptr [00007FF6F0F041C8h], rax
00007FF6F0EE1359  mov          r8d, 90h
00007FF6F0EE135F  xor          edx, edx
00007FF6F0EE1361  call         HeapAlloc (00007FF6F0EF98C8h)
00007FF6F0EE1366  test         rax, rax
00007FF6F0EE1369  je           static void Fabled_Engine::main+ACDh (00007FF6F0EE1D8Dh)
00007FF6F0EE136F  movaps       xmm0, xmmword ptr [__xmm@3f06963e000000003f59c433bf06963e (00007FF6F0EFD530h)]
00007FF6F0EE1376  movups       xmmword ptr [rax], xmm0
00007FF6F0EE1379  movaps       xmm0, xmmword ptr [__xmm@bf59c433bf06963e000000003f59c433 (00007FF6F0EFD540h)]
00007FF6F0EE1380  movups       xmmword ptr [rax+10h], xmm0
00007FF6F0EE1384  movaps       xmm0, xmmword ptr [__xmm@00000000bf59c4333f06963e00000000 (00007FF6F0EFD550h)]
00007FF6F0EE138B  movups       xmmword ptr [rax+20h], xmm0
00007FF6F0EE138F  movaps       xmm1, xmmword ptr [__xmm@000000003f59c433bf06963e00000000 (00007FF6F0EFD560h)]
00007FF6F0EE1396  movups       xmmword ptr [rax+30h], xmm1
00007FF6F0EE139A  movaps       xmm1, xmmword ptr [__xmm@bf06963e000000003f59c4333f06963e (00007FF6F0EFD570h)]
00007FF6F0EE13A1  movups       xmmword ptr [rax+40h], xmm1
00007FF6F0EE13A5  movaps       xmm1, xmmword ptr [__xmm@bf59c4333f06963e00000000bf59c433 (00007FF6F0EFD580h)]
00007FF6F0EE13AC  movups       xmmword ptr [rax+50h], xmm1
00007FF6F0EE13B0  movaps       xmm1, xmmword ptr [__xmm@3f59c433bf06963e000000003f59c433 (00007FF6F0EFD590h)]
00007FF6F0EE13B7  movups       xmmword ptr [rax+60h], xmm1
00007FF6F0EE13BB  movups       xmmword ptr [rax+70h], xmm0
00007FF6F0EE13BF  movaps       xmm0, xmmword ptr [__xmm@3f06963e00000000bf59c433bf06963e (00007FF6F0EFD5A0h)]
00007FF6F0EE13C6  movups       xmmword ptr [rax+80h], xmm0
00007FF6F0EE13CD  mov          qword ptr [rsp+30h], rax
00007FF6F0EE13D2  movaps       xmm0, xmmword ptr [__xmm@00000000000000240000000000000024 (00007FF6F0EFD5B0h)]
00007FF6F0EE13D9  movups       xmmword ptr [rsp+38h], xmm0
00007FF6F0EE13DE  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE13E5  test         rcx, rcx
00007FF6F0EE13E8  jne          static void Fabled_Engine::main+142h (00007FF6F0EE1402h)
00007FF6F0EE13EA  call         GetProcessHeap (00007FF6F0EF98C2h)
00007FF6F0EE13EF  test         rax, rax
00007FF6F0EE13F2  je           static void Fabled_Engine::main+AD4h (00007FF6F0EE1D94h)
00007FF6F0EE13F8  mov          rcx, rax
00007FF6F0EE13FB  mov          qword ptr [00007FF6F0F041C8h], rax
00007FF6F0EE1402  mov          r8d, 1E0h
00007FF6F0EE1408  xor          edx, edx
00007FF6F0EE140A  call         HeapAlloc (00007FF6F0EF98C8h)
00007FF6F0EE140F  test         rax, rax
00007FF6F0EE1412  je           static void Fabled_Engine::main+AD4h (00007FF6F0EE1D94h)
00007FF6F0EE1418  mov          rbx, rax
00007FF6F0EE141B  movaps       xmm0, xmmword ptr [__xmm@000000000000000b0000000000000000 (00007FF6F0EFD5C0h)]
00007FF6F0EE1422  movups       xmmword ptr [rax], xmm0
00007FF6F0EE1425  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000005 (00007FF6F0EFD5D0h)]
00007FF6F0EE142C  movups       xmmword ptr [rax+10h], xmm0
00007FF6F0EE1430  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000005 (00007FF6F0EFD5E0h)]
00007FF6F0EE1437  movups       xmmword ptr [rax+20h], xmm0
00007FF6F0EE143B  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000000 (00007FF6F0EFD5F0h)]
00007FF6F0EE1442  movups       xmmword ptr [rax+30h], xmm0
00007FF6F0EE1446  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000007 (00007FF6F0EFD600h)]
00007FF6F0EE144D  movups       xmmword ptr [rax+40h], xmm0
00007FF6F0EE1451  movaps       xmm0, xmmword ptr [__xmm@000000000000000a0000000000000007 (00007FF6F0EFD610h)]
00007FF6F0EE1458  movups       xmmword ptr [rax+50h], xmm0
00007FF6F0EE145C  movaps       xmm0, xmmword ptr [__xmm@000000000000000a0000000000000000 (00007FF6F0EFD620h)]
00007FF6F0EE1463  movups       xmmword ptr [rax+60h], xmm0
00007FF6F0EE1467  movaps       xmm0, xmmword ptr [__xmm@0000000000000001000000000000000b (00007FF6F0EFD630h)]
00007FF6F0EE146E  movups       xmmword ptr [rax+70h], xmm0
00007FF6F0EE1472  movaps       xmm0, xmmword ptr [__xmm@00000000000000090000000000000005 (00007FF6F0EFD640h)]
00007FF6F0EE1479  movups       xmmword ptr [rax+80h], xmm0
00007FF6F0EE1480  movaps       xmm0, xmmword ptr [__xmm@000000000000000b0000000000000005 (00007FF6F0EFD650h)]
00007FF6F0EE1487  movups       xmmword ptr [rax+90h], xmm0
00007FF6F0EE148E  movaps       xmm0, xmmword ptr [__xmm@000000000000000b0000000000000004 (00007FF6F0EFD660h)]
00007FF6F0EE1495  movups       xmmword ptr [rax+A0h], xmm0
00007FF6F0EE149C  movaps       xmm0, xmmword ptr [__xmm@0000000000000002000000000000000a (00007FF6F0EFD670h)]
00007FF6F0EE14A3  movups       xmmword ptr [rax+B0h], xmm0
00007FF6F0EE14AA  movaps       xmm0, xmmword ptr [__xmm@0000000000000007000000000000000a (00007FF6F0EFD680h)]
00007FF6F0EE14B1  movups       xmmword ptr [rax+C0h], xmm0
00007FF6F0EE14B8  movaps       xmm0, xmmword ptr [__xmm@00000000000000070000000000000006 (00007FF6F0EFD690h)]
00007FF6F0EE14BF  movups       xmmword ptr [rax+D0h], xmm0
00007FF6F0EE14C6  movaps       xmm0, xmmword ptr [__xmm@00000000000000080000000000000001 (00007FF6F0EFD6A0h)]
00007FF6F0EE14CD  movups       xmmword ptr [rax+E0h], xmm0
00007FF6F0EE14D4  movaps       xmm0, xmmword ptr [__xmm@00000000000000090000000000000003 (00007FF6F0EFD6B0h)]
00007FF6F0EE14DB  movups       xmmword ptr [rax+F0h], xmm0
00007FF6F0EE14E2  movaps       xmm0, xmmword ptr [__xmm@00000000000000030000000000000004 (00007FF6F0EFD6C0h)]
00007FF6F0EE14E9  movups       xmmword ptr [rax+100h], xmm0
00007FF6F0EE14F0  movaps       xmm0, xmmword ptr [__xmm@00000000000000020000000000000004 (00007FF6F0EFD6D0h)]
00007FF6F0EE14F7  movups       xmmword ptr [rax+110h], xmm0
00007FF6F0EE14FE  movaps       xmm0, xmmword ptr [__xmm@00000000000000020000000000000003 (00007FF6F0EFD6E0h)]
00007FF6F0EE1505  movups       xmmword ptr [rax+120h], xmm0
00007FF6F0EE150C  movaps       xmm0, xmmword ptr [__xmm@00000000000000030000000000000006 (00007FF6F0EFD6F0h)]
00007FF6F0EE1513  movups       xmmword ptr [rax+130h], xmm0
00007FF6F0EE151A  movaps       xmm0, xmmword ptr [__xmm@00000000000000080000000000000006 (00007FF6F0EFD700h)]
00007FF6F0EE1521  movups       xmmword ptr [rax+140h], xmm0
00007FF6F0EE1528  movaps       xmm0, xmmword ptr [__xmm@00000000000000080000000000000003 (00007FF6F0EFD710h)]
00007FF6F0EE152F  movups       xmmword ptr [rax+150h], xmm0
00007FF6F0EE1536  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000009 (00007FF6F0EFD720h)]
00007FF6F0EE153D  movups       xmmword ptr [rax+160h], xmm0
00007FF6F0EE1544  movaps       xmm0, xmmword ptr [__xmm@00000000000000050000000000000009 (00007FF6F0EFD730h)]
00007FF6F0EE154B  movups       xmmword ptr [rax+170h], xmm0
00007FF6F0EE1552  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000002 (00007FF6F0EFD740h)]
00007FF6F0EE1559  movups       xmmword ptr [rax+180h], xmm0
00007FF6F0EE1560  movaps       xmm0, xmmword ptr [__xmm@0000000000000006000000000000000b (00007FF6F0EFD750h)]
00007FF6F0EE1567  movups       xmmword ptr [rax+190h], xmm0
00007FF6F0EE156E  movaps       xmm0, xmmword ptr [__xmm@000000000000000a0000000000000002 (00007FF6F0EFD760h)]
00007FF6F0EE1575  movups       xmmword ptr [rax+1A0h], xmm0
00007FF6F0EE157C  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000008 (00007FF6F0EFD770h)]
00007FF6F0EE1583  movups       xmmword ptr [rax+1B0h], xmm0
00007FF6F0EE158A  movaps       xmm0, xmmword ptr [__xmm@00000000000000090000000000000007 (00007FF6F0EFD780h)]
00007FF6F0EE1591  movups       xmmword ptr [rax+1C0h], xmm0
00007FF6F0EE1598  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000008 (00007FF6F0EFD790h)]
00007FF6F0EE159F  movups       xmmword ptr [rax+1D0h], xmm0
00007FF6F0EE15A6  xorps        xmm0, xmm0
00007FF6F0EE15A9  movups       xmmword ptr [rsp+80h], xmm0
00007FF6F0EE15B1  mov          qword ptr [rsp+70h], 0h
00007FF6F0EE15BA  lea          rax, [00007FF6F0EFEE60h]
00007FF6F0EE15C1  mov          qword ptr [rsp+78h], rax
00007FF6F0EE15C6  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE15CD  test         rcx, rcx
00007FF6F0EE15D0  jne          static void Fabled_Engine::main+32Ah (00007FF6F0EE15EAh)
00007FF6F0EE15D2  call         GetProcessHeap (00007FF6F0EF98C2h)
00007FF6F0EE15D7  test         rax, rax
00007FF6F0EE15DA  je           static void Fabled_Engine::main+B9Fh (00007FF6F0EE1E5Fh)
00007FF6F0EE15E0  mov          rcx, rax
00007FF6F0EE15E3  mov          qword ptr [00007FF6F0F041C8h], rax
00007FF6F0EE15EA  mov          esi, 780h
00007FF6F0EE15EF  mov          r8d, 780h
00007FF6F0EE15F5  mov          edx, 8h
00007FF6F0EE15FA  call         HeapAlloc (00007FF6F0EF98C8h)
00007FF6F0EE15FF  test         rax, rax
00007FF6F0EE1602  je           static void Fabled_Engine::main+BABh (00007FF6F0EE1E6Bh)
00007FF6F0EE1608  mov          r13, rax
00007FF6F0EE160B  xor          esi, esi
00007FF6F0EE160D  mov          qword ptr [rsp+68h], rbx
00007FF6F0EE1612  nop          word ptr cs:[rax+rax*1], ax
00007FF6F0EE161C  nop          dword ptr [rax], eax
00007FF6F0EE1620  mov          rdi, qword ptr [rbx+rsi*2]
00007FF6F0EE1624  mov          rbp, qword ptr [rbx+rsi*2+8h]
00007FF6F0EE1629  mov          rbx, qword ptr [rbx+rsi*2+10h]
00007FF6F0EE162E  mov          rcx, rdi
00007FF6F0EE1631  mov          rdx, rbp
00007FF6F0EE1634  lea          r12, [rsp+30h]
00007FF6F0EE1639  mov          r8, r12
00007FF6F0EE163C  lea          r15, [rsp+70h]
00007FF6F0EE1641  mov          r9, r15
00007FF6F0EE1644  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF6F0EE5C50h)
00007FF6F0EE1649  mov          r14, rax
00007FF6F0EE164C  mov          rcx, rbp
00007FF6F0EE164F  mov          rdx, rbx
00007FF6F0EE1652  mov          r8, r12
00007FF6F0EE1655  mov          r9, r15
00007FF6F0EE1658  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF6F0EE5C50h)
00007FF6F0EE165D  mov          r15, rax
00007FF6F0EE1660  mov          rcx, rbx
00007FF6F0EE1663  mov          rdx, rdi
00007FF6F0EE1666  mov          r8, r12
00007FF6F0EE1669  lea          r9, [rsp+70h]
00007FF6F0EE166E  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF6F0EE5C50h)
00007FF6F0EE1673  cmp          rsi, FCh
00007FF6F0EE167A  je           static void Fabled_Engine::main+AB0h (00007FF6F0EE1D70h)
00007FF6F0EE1680  mov          qword ptr [r13+rsi*8], rdi
00007FF6F0EE1685  mov          qword ptr [r13+rsi*8+8h], r14
00007FF6F0EE168A  mov          qword ptr [r13+rsi*8+10h], rax
00007FF6F0EE168F  mov          qword ptr [r13+rsi*8+18h], rbp
00007FF6F0EE1694  mov          qword ptr [r13+rsi*8+20h], r15
00007FF6F0EE1699  mov          qword ptr [r13+rsi*8+28h], r14
00007FF6F0EE169E  mov          qword ptr [r13+rsi*8+30h], rbx
00007FF6F0EE16A3  mov          qword ptr [r13+rsi*8+38h], rax
00007FF6F0EE16A8  mov          qword ptr [r13+rsi*8+40h], r15
00007FF6F0EE16AD  mov          qword ptr [r13+rsi*8+48h], r14
00007FF6F0EE16B2  mov          qword ptr [r13+rsi*8+50h], r15
00007FF6F0EE16B7  mov          qword ptr [r13+rsi*8+58h], rax
00007FF6F0EE16BC  add          rsi, Ch
00007FF6F0EE16C0  cmp          rsi, F0h
00007FF6F0EE16C7  mov          rbx, qword ptr [rsp+68h]
00007FF6F0EE16CC  jne          static void Fabled_Engine::main+360h (00007FF6F0EE1620h)
00007FF6F0EE16D2  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE16D9  xor          edx, edx
00007FF6F0EE16DB  mov          r8, rbx
00007FF6F0EE16DE  call         HeapFree (00007FF6F0EF98CEh)
00007FF6F0EE16E3  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE16EA  test         rcx, rcx
00007FF6F0EE16ED  jne          static void Fabled_Engine::main+447h (00007FF6F0EE1707h)
00007FF6F0EE16EF  call         GetProcessHeap (00007FF6F0EF98C2h)
00007FF6F0EE16F4  test         rax, rax
00007FF6F0EE16F7  je           static void Fabled_Engine::main+BA6h (00007FF6F0EE1E66h)
00007FF6F0EE16FD  mov          rcx, rax
00007FF6F0EE1700  mov          qword ptr [00007FF6F0F041C8h], rax
00007FF6F0EE1707  mov          esi, 1E00h
00007FF6F0EE170C  mov          r8d, 1E00h
00007FF6F0EE1712  mov          edx, 8h
00007FF6F0EE1717  call         HeapAlloc (00007FF6F0EF98C8h)
00007FF6F0EE171C  mov          qword ptr [rsp+68h], rax
00007FF6F0EE1721  test         rax, rax
00007FF6F0EE1724  je           static void Fabled_Engine::main+BABh (00007FF6F0EE1E6Bh)
00007FF6F0EE172A  xor          esi, esi
00007FF6F0EE172C  xor          r12d, r12d
00007FF6F0EE172F  nop
00007FF6F0EE1730  mov          rbp, qword ptr [r13+rsi*2]
00007FF6F0EE1735  mov          rbx, qword ptr [r13+rsi*2+8h]
00007FF6F0EE173A  mov          rdi, qword ptr [r13+rsi*2+10h]
00007FF6F0EE173F  mov          rcx, rbp
00007FF6F0EE1742  mov          rdx, rbx
00007FF6F0EE1745  lea          r14, [rsp+30h]
00007FF6F0EE174A  mov          r8, r14
00007FF6F0EE174D  lea          r9, [rsp+70h]
00007FF6F0EE1752  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF6F0EE5C50h)
00007FF6F0EE1757  mov          r15, rax
00007FF6F0EE175A  mov          rcx, rbx
00007FF6F0EE175D  mov          rdx, rdi
00007FF6F0EE1760  mov          r8, r14
00007FF6F0EE1763  lea          r9, [rsp+70h]
00007FF6F0EE1768  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF6F0EE5C50h)
00007FF6F0EE176D  mov          r14, rax
00007FF6F0EE1770  mov          rcx, rdi
00007FF6F0EE1773  mov          rdx, rbp
00007FF6F0EE1776  lea          r8, [rsp+30h]
00007FF6F0EE177B  lea          r9, [rsp+70h]
00007FF6F0EE1780  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF6F0EE5C50h)
00007FF6F0EE1785  cmp          r12, 50h
00007FF6F0EE1789  je           static void Fabled_Engine::main+A96h (00007FF6F0EE1D56h)
00007FF6F0EE178F  cmp          r12, 51h
00007FF6F0EE1793  je           static void Fabled_Engine::main+AB7h (00007FF6F0EE1D77h)
00007FF6F0EE1799  add          r12, 1h
00007FF6F0EE179D  mov          rcx, qword ptr [rsp+68h]
00007FF6F0EE17A2  mov          qword ptr [rcx+rsi*8], rbp
00007FF6F0EE17A6  mov          qword ptr [rcx+rsi*8+8h], r15
00007FF6F0EE17AB  mov          qword ptr [rcx+rsi*8+10h], rax
00007FF6F0EE17B0  mov          qword ptr [rcx+rsi*8+18h], rbx
00007FF6F0EE17B5  mov          qword ptr [rcx+rsi*8+20h], r14
00007FF6F0EE17BA  mov          qword ptr [rcx+rsi*8+28h], r15
00007FF6F0EE17BF  mov          qword ptr [rcx+rsi*8+30h], rdi
00007FF6F0EE17C4  mov          qword ptr [rcx+rsi*8+38h], rax
00007FF6F0EE17C9  mov          qword ptr [rcx+rsi*8+40h], r14
00007FF6F0EE17CE  mov          qword ptr [rcx+rsi*8+48h], r15
00007FF6F0EE17D3  mov          qword ptr [rcx+rsi*8+50h], r14
00007FF6F0EE17D8  mov          qword ptr [rcx+rsi*8+58h], rax
00007FF6F0EE17DD  add          rsi, Ch
00007FF6F0EE17E1  cmp          r12, 50h
00007FF6F0EE17E5  jne          static void Fabled_Engine::main+470h (00007FF6F0EE1730h)
00007FF6F0EE17EB  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE17F2  xor          edx, edx
00007FF6F0EE17F4  mov          r8, r13
00007FF6F0EE17F7  call         HeapFree (00007FF6F0EF98CEh)
00007FF6F0EE17FC  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE1803  test         rcx, rcx
00007FF6F0EE1806  jne          static void Fabled_Engine::main+560h (00007FF6F0EE1820h)
00007FF6F0EE1808  call         GetProcessHeap (00007FF6F0EF98C2h)
00007FF6F0EE180D  test         rax, rax
00007FF6F0EE1810  je           static void Fabled_Engine::main+AE5h (00007FF6F0EE1DA5h)
00007FF6F0EE1816  mov          rcx, rax
00007FF6F0EE1819  mov          qword ptr [00007FF6F0F041C8h], rax
00007FF6F0EE1820  mov          r8d, 788h
00007FF6F0EE1826  mov          edx, 8h
00007FF6F0EE182B  call         HeapAlloc (00007FF6F0EF98C8h)
00007FF6F0EE1830  test         rax, rax
00007FF6F0EE1833  je           static void Fabled_Engine::main+AE5h (00007FF6F0EE1DA5h)
00007FF6F0EE1839  mov          rbx, rax
00007FF6F0EE183C  mov          r14, qword ptr [rsp+40h]
00007FF6F0EE1841  mov          r15, AAAAAAAAAAAAAAABh
00007FF6F0EE184B  mov          rax, r14
00007FF6F0EE184E  mul          r15
00007FF6F0EE1851  mov          rdi, qword ptr [rsp+30h]
00007FF6F0EE1856  shr          rdx, 1h
00007FF6F0EE1859  lea          rax, [rdx+rdx*2]
00007FF6F0EE185D  cmp          rax, 3h
00007FF6F0EE1861  jb           static void Fabled_Engine::main+636h (00007FF6F0EE18F6h)
00007FF6F0EE1867  add          rax, FFFFFFFFFFFFFFFDh
00007FF6F0EE186B  mul          r15
00007FF6F0EE186E  mov          r12, rdx
00007FF6F0EE1871  shr          r12, 1h
00007FF6F0EE1874  add          r12, 1h
00007FF6F0EE1878  lea          rbp, [rdi+8h]
00007FF6F0EE187C  xor          esi, esi
00007FF6F0EE187E  movss        xmm8, dword ptr [__real@3ea2f983 (00007FF6F0EFD7A0h)]
00007FF6F0EE1887  movss        xmm7, dword ptr [__real@3f000000 (00007FF6F0EFD7A4h)]
00007FF6F0EE188F  movss        xmm9, dword ptr [__real@3e22f983 (00007FF6F0EFD7A8h)]
00007FF6F0EE1898  nop          dword ptr [rax+rax*1], eax
00007FF6F0EE18A0  movss        xmm1, dword ptr [rbp-8h]
00007FF6F0EE18A5  movss        xmm0, dword ptr [rbp]
00007FF6F0EE18AA  call         atan2f (00007FF6F0EFA9E9h)
00007FF6F0EE18AF  movaps       xmm6, xmm0
00007FF6F0EE18B2  movss        xmm0, dword ptr [rbp-4h]
00007FF6F0EE18B7  call         asinf (00007FF6F0EFA9EFh)
00007FF6F0EE18BC  cmp          rsi, F1h
00007FF6F0EE18C3  je           static void Fabled_Engine::main+A96h (00007FF6F0EE1D56h)
00007FF6F0EE18C9  mulss        xmm0, xmm8
00007FF6F0EE18CE  addss        xmm0, xmm7
00007FF6F0EE18D2  mulss        xmm6, xmm9
00007FF6F0EE18D7  addss        xmm6, xmm7
00007FF6F0EE18DB  movss        dword ptr [rbx+rsi*8], xmm6
00007FF6F0EE18E0  movss        dword ptr [rbx+rsi*8+4h], xmm0
00007FF6F0EE18E6  lea          rax, [rsi+1h]
00007FF6F0EE18EA  add          rbp, Ch
00007FF6F0EE18EE  mov          rsi, rax
00007FF6F0EE18F1  cmp          r12, rax
00007FF6F0EE18F4  jne          static void Fabled_Engine::main+5E0h (00007FF6F0EE18A0h)
00007FF6F0EE18F6  mov          rcx, qword ptr [00007FF6F0EFECE0h]
00007FF6F0EE18FD  mov          qword ptr [rsp+48h], rcx
00007FF6F0EE1902  xorps        xmm8, xmm8
00007FF6F0EE1906  movups       xmmword ptr [rsp+50h], xmm8
00007FF6F0EE190C  mov          rax, r14
00007FF6F0EE190F  mul          r15
00007FF6F0EE1912  shr          rdx, 1h
00007FF6F0EE1915  cmp          rdx, F1h
00007FF6F0EE191C  mov          ebp, F1h
00007FF6F0EE1921  cmovb        rbp, rdx
00007FF6F0EE1925  test         rbp, rbp
00007FF6F0EE1928  je           static void Fabled_Engine::main+750h (00007FF6F0EE1A10h)
00007FF6F0EE192E  add          rbp, FFFFFFFFFFFFFFFFh
00007FF6F0EE1932  add          rdi, 8h
00007FF6F0EE1936  xor          edx, edx
00007FF6F0EE1938  lea          r14, [rsp+48h]
00007FF6F0EE193D  xor          eax, eax
00007FF6F0EE193F  xor          esi, esi
00007FF6F0EE1941  nop          word ptr cs:[rax+rax*1], ax
00007FF6F0EE194B  nop          dword ptr [rax+rax*1], eax
00007FF6F0EE1950  movss        xmm7, dword ptr [rdi-8h]
00007FF6F0EE1955  movss        xmm6, dword ptr [rdi-4h]
00007FF6F0EE195A  movss        xmm9, dword ptr [rdi]
00007FF6F0EE195F  movss        xmm10, dword ptr [rbx+rsi*8]
00007FF6F0EE1965  movss        xmm11, dword ptr [rbx+rsi*8+4h]
00007FF6F0EE196C  movaps       xmmword ptr [rsp+A0h], xmm8
00007FF6F0EE1975  movaps       xmmword ptr [rsp+B0h], xmm8
00007FF6F0EE197E  cmp          rax, rdx
00007FF6F0EE1981  je           static void Fabled_Engine::main+739h (00007FF6F0EE19F9h)
00007FF6F0EE1983  shl          rax, 6h
00007FF6F0EE1987  movss        dword ptr [rcx+rax*1], xmm7
00007FF6F0EE198C  movss        dword ptr [rcx+rax*1+4h], xmm6
00007FF6F0EE1992  movss        dword ptr [rcx+rax*1+8h], xmm9
00007FF6F0EE1999  movss        dword ptr [rcx+rax*1+Ch], xmm10
00007FF6F0EE19A0  movss        dword ptr [rcx+rax*1+10h], xmm11
00007FF6F0EE19A7  movss        dword ptr [rcx+rax*1+14h], xmm7
00007FF6F0EE19AD  movss        dword ptr [rcx+rax*1+18h], xmm6
00007FF6F0EE19B3  movss        dword ptr [rcx+rax*1+1Ch], xmm9
00007FF6F0EE19BA  movaps       xmm0, xmmword ptr [rsp+A0h]
00007FF6F0EE19C2  movaps       xmmword ptr [rcx+rax*1+20h], xmm0
00007FF6F0EE19C7  movaps       xmm0, xmmword ptr [rsp+B0h]
00007FF6F0EE19CF  movaps       xmmword ptr [rcx+rax*1+30h], xmm0
00007FF6F0EE19D4  mov          rax, qword ptr [rsp+58h]
00007FF6F0EE19D9  add          rax, 1h
00007FF6F0EE19DD  mov          qword ptr [rsp+58h], rax
00007FF6F0EE19E2  cmp          rbp, rsi
00007FF6F0EE19E5  je           static void Fabled_Engine::main+750h (00007FF6F0EE1A10h)
00007FF6F0EE19E7  add          rsi, 1h
00007FF6F0EE19EB  mov          rdx, qword ptr [rsp+50h]
00007FF6F0EE19F0  add          rdi, Ch
00007FF6F0EE19F4  jmp          static void Fabled_Engine::main+690h (00007FF6F0EE1950h)
00007FF6F0EE19F9  mov          rcx, r14
00007FF6F0EE19FC  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF6F0EFB930h)
00007FF6F0EE1A01  mov          rcx, qword ptr [rsp+48h]
00007FF6F0EE1A06  mov          rax, qword ptr [rsp+58h]
00007FF6F0EE1A0B  jmp          static void Fabled_Engine::main+6C3h (00007FF6F0EE1983h)
00007FF6F0EE1A10  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE1A17  test         rcx, rcx
00007FF6F0EE1A1A  jne          static void Fabled_Engine::main+774h (00007FF6F0EE1A34h)
00007FF6F0EE1A1C  call         GetProcessHeap (00007FF6F0EF98C2h)
00007FF6F0EE1A21  test         rax, rax
00007FF6F0EE1A24  je           static void Fabled_Engine::main+AF6h (00007FF6F0EE1DB6h)
00007FF6F0EE1A2A  mov          rcx, rax
00007FF6F0EE1A2D  mov          qword ptr [00007FF6F0F041C8h], rax
00007FF6F0EE1A34  mov          r8d, 40h
00007FF6F0EE1A3A  xor          edx, edx
00007FF6F0EE1A3C  call         HeapAlloc (00007FF6F0EF98C8h)
00007FF6F0EE1A41  test         rax, rax
00007FF6F0EE1A44  je           static void Fabled_Engine::main+AF6h (00007FF6F0EE1DB6h)
00007FF6F0EE1A4A  mov          r14, rax
00007FF6F0EE1A4D  mov          rax, qword ptr [rsp+58h]
00007FF6F0EE1A52  mov          qword ptr [r14+10h], rax
00007FF6F0EE1A56  movups       xmm0, xmmword ptr [rsp+48h]
00007FF6F0EE1A5B  movups       xmmword ptr [r14], xmm0
00007FF6F0EE1A5F  mov          rax, qword ptr [rsp+68h]
00007FF6F0EE1A64  mov          qword ptr [r14+20h], rax
00007FF6F0EE1A68  movaps       xmm0, xmmword ptr [__xmm@00000000000003c000000000000003c0 (00007FF6F0EFD7B0h)]
00007FF6F0EE1A6F  movups       xmmword ptr [r14+28h], xmm0
00007FF6F0EE1A74  mov          dword ptr [r14+18h], 0h
00007FF6F0EE1A7C  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE1A83  xor          edx, edx
00007FF6F0EE1A85  mov          r8, rbx
00007FF6F0EE1A88  call         HeapFree (00007FF6F0EF98CEh)
00007FF6F0EE1A8D  mov          rcx, qword ptr [rsp+70h]
00007FF6F0EE1A92  test         rcx, rcx
00007FF6F0EE1A95  je           static void Fabled_Engine::main+802h (00007FF6F0EE1AC2h)
00007FF6F0EE1A97  lea          rax, [rcx+1h]
00007FF6F0EE1A9B  mov          edx, 10h
00007FF6F0EE1AA0  mul          rdx
00007FF6F0EE1AA3  add          rcx, rax
00007FF6F0EE1AA6  cmp          rcx, FFFFFFFFFFFFFFEFh
00007FF6F0EE1AAA  je           static void Fabled_Engine::main+802h (00007FF6F0EE1AC2h)
00007FF6F0EE1AAC  mov          r8, qword ptr [rsp+78h]
00007FF6F0EE1AB1  sub          r8, rax
00007FF6F0EE1AB4  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE1ABB  xor          edx, edx
00007FF6F0EE1ABD  call         HeapFree (00007FF6F0EF98CEh)
00007FF6F0EE1AC2  mov          rax, qword ptr [rsp+38h]
00007FF6F0EE1AC7  test         rax, rax
00007FF6F0EE1ACA  je           static void Fabled_Engine::main+82Dh (00007FF6F0EE1AEDh)
00007FF6F0EE1ACC  shl          rax, 2h
00007FF6F0EE1AD0  test         rax, rax
00007FF6F0EE1AD3  je           static void Fabled_Engine::main+82Dh (00007FF6F0EE1AEDh)
00007FF6F0EE1AD5  mov          r8, qword ptr [rsp+30h]
00007FF6F0EE1ADA  test         r8, r8
00007FF6F0EE1ADD  je           static void Fabled_Engine::main+82Dh (00007FF6F0EE1AEDh)
00007FF6F0EE1ADF  mov          rcx, qword ptr [00007FF6F0F041C8h]
00007FF6F0EE1AE6  xor          edx, edx
00007FF6F0EE1AE8  call         HeapFree (00007FF6F0EF98CEh)