# --------------- Capsule Dissassembly -------------------
00007FF66DDC1099  xorps        xmm6, xmm6
00007FF66DDC109C  movaps       xmmword ptr [rsp+1A0h], xmm6
00007FF66DDC10A4  lea          rcx, [rsp+A8h]
00007FF66DDC10AC  lea          rdx, [rsp+1A0h]
00007FF66DDC10B4  mov          r8d, 2F5h
00007FF66DDC10BA  call         static struct alloc::vec::Vec<glam::vec3::Vec3A, alloc::alloc::Global> alloc::vec::from_elem<glam::vec3::Vec3A> (00007FF66DDC6B30h)
00007FF66DDC10BF  lea          rcx, [rsp+70h]
00007FF66DDC10C4  mov          edx, 2F5h
00007FF66DDC10C9  call         static struct alloc::vec::Vec<glam::vec2::Vec2, alloc::alloc::Global> alloc::vec::from_elem<glam::vec2::Vec2> (00007FF66DDC6A00h)
00007FF66DDC10CE  movaps       xmmword ptr [rsp+190h], xmm6
00007FF66DDC10D6  lea          rcx, [rsp+88h]
00007FF66DDC10DE  lea          rdx, [rsp+190h]
00007FF66DDC10E6  mov          r8d, 2F5h
00007FF66DDC10EC  call         static struct alloc::vec::Vec<glam::vec3::Vec3A, alloc::alloc::Global> alloc::vec::from_elem<glam::vec3::Vec3A> (00007FF66DDC6B30h)
00007FF66DDC10F1  lea          rcx, [rsp+30h]
00007FF66DDC10F6  mov          edx, 20h
00007FF66DDC10FB  call         static struct alloc::vec::Vec<glam::vec2::Vec2, alloc::alloc::Global> alloc::vec::from_elem<glam::vec2::Vec2> (00007FF66DDC6A00h)
00007FF66DDC1100  lea          rcx, [rsp+48h]
00007FF66DDC1105  mov          edx, 20h
00007FF66DDC110A  call         static struct alloc::vec::Vec<glam::vec2::Vec2, alloc::alloc::Global> alloc::vec::from_elem<glam::vec2::Vec2> (00007FF66DDC6A00h)
00007FF66DDC110F  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC1116  test         rcx, rcx
00007FF66DDC1119  jne          static void Fabled_Engine::main+133h (00007FF66DDC1133h)
00007FF66DDC111B  call         GetProcessHeap (00007FF66DDDA2ECh)
00007FF66DDC1120  test         rax, rax
00007FF66DDC1123  je           static void Fabled_Engine::main+1893h (00007FF66DDC2893h)
00007FF66DDC1129  mov          rcx, rax
00007FF66DDC112C  mov          qword ptr [00007FF66DDE41C8h], rax
00007FF66DDC1133  mov          r8d, 84h
00007FF66DDC1139  mov          edx, 8h
00007FF66DDC113E  call         HeapAlloc (00007FF66DDDA2F2h)
00007FF66DDC1143  test         rax, rax
00007FF66DDC1146  je           static void Fabled_Engine::main+1893h (00007FF66DDC2893h)
00007FF66DDC114C  movss        xmm0, dword ptr [__real@40a00000 (00007FF66DDDD520h)]
00007FF66DDC1154  minss        xmm0, dword ptr [__real@3f800000 (00007FF66DDDD524h)]
00007FF66DDC115C  movaps       xmmword ptr [rsp+180h], xmm0
00007FF66DDC1164  mov          r13, qword ptr [rsp+30h]
00007FF66DDC1169  mov          rcx, qword ptr [rsp+40h]
00007FF66DDC116E  mov          qword ptr [rsp+C0h], rcx
00007FF66DDC1176  mov          rbx, qword ptr [rsp+48h]
00007FF66DDC117B  mov          rcx, qword ptr [rsp+58h]
00007FF66DDC1180  mov          qword ptr [rsp+F0h], rcx
00007FF66DDC1188  mov          rbp, qword ptr [rsp+70h]
00007FF66DDC118D  mov          rcx, qword ptr [rsp+80h]
00007FF66DDC1195  cmp          rcx, 2D5h
00007FF66DDC119C  mov          r15d, 2D5h
00007FF66DDC11A2  mov          qword ptr [rsp+68h], rcx
00007FF66DDC11A7  cmova        r15, rcx
00007FF66DDC11AB  add          r15, FFFFFFFFFFFFFD2Bh
00007FF66DDC11B2  xor          r12d, r12d
00007FF66DDC11B5  movss        xmm8, dword ptr [__real@3f000000 (00007FF66DDDD528h)]
00007FF66DDC11BE  movss        xmm13, dword ptr [__real@bd000000 (00007FF66DDDD52Ch)]
00007FF66DDC11C7  movss        xmm9, dword ptr [__real@3f800000 (00007FF66DDDD524h)]
00007FF66DDC11D0  movss        xmm12, dword ptr [__real@3e490fdb (00007FF66DDDD530h)]
00007FF66DDC11D9  movaps       xmm10, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF66DDDD540h)]
00007FF66DDC11E1  movaps       xmm11, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF66DDDD550h)]
00007FF66DDC11E9  xor          r14d, r14d
00007FF66DDC11EC  mov          qword ptr [rsp+A0h], rax
00007FF66DDC11F4  nop          word ptr cs:[rax+rax*1], ax
00007FF66DDC11FE  nop
00007FF66DDC1200  test         r14, r14
00007FF66DDC1203  js           static void Fabled_Engine::main+210h (00007FF66DDC1210h)
00007FF66DDC1205  xorps        xmm6, xmm6
00007FF66DDC1208  cvtsi2ss     xmm6, r14
00007FF66DDC120D  jmp          static void Fabled_Engine::main+22Bh (00007FF66DDC122Bh)
00007FF66DDC120F  nop
00007FF66DDC1210  mov          rax, r14
00007FF66DDC1213  shr          rax, 1h
00007FF66DDC1216  mov          ecx, r14d
00007FF66DDC1219  and          ecx, 1h
00007FF66DDC121C  or           rcx, rax
00007FF66DDC121F  xorps        xmm6, xmm6
00007FF66DDC1222  cvtsi2ss     xmm6, rcx
00007FF66DDC1227  addss        xmm6, xmm6
00007FF66DDC122B  movaps       xmm7, xmm6
00007FF66DDC122E  addss        xmm7, xmm8
00007FF66DDC1233  mulss        xmm7, xmm13
00007FF66DDC1238  addss        xmm7, xmm9
00007FF66DDC123D  mulss        xmm6, xmm12
00007FF66DDC1242  movaps       xmm0, xmm6
00007FF66DDC1245  call         sinf (00007FF66DDDB419h)
00007FF66DDC124A  movaps       xmm14, xmm0
00007FF66DDC124E  movaps       xmm0, xmm6
00007FF66DDC1251  call         cosf (00007FF66DDDB41Fh)
00007FF66DDC1256  cmp          qword ptr [rsp+C0h], r14
00007FF66DDC125E  je           static void Fabled_Engine::main+147Bh (00007FF66DDC247Bh)
00007FF66DDC1264  movss        dword ptr [r13+r14*8], xmm0
00007FF66DDC126B  movss        dword ptr [r13+r14*8+4h], xmm14
00007FF66DDC1272  cmp          qword ptr [rsp+F0h], r14
00007FF66DDC127A  je           static void Fabled_Engine::main+1494h (00007FF66DDC2494h)
00007FF66DDC1280  mulss        xmm14, xmm8
00007FF66DDC1285  mulss        xmm0, xmm8
00007FF66DDC128A  movss        dword ptr [rbx+r14*8], xmm0
00007FF66DDC1290  movss        dword ptr [rbx+r14*8+4h], xmm14
00007FF66DDC1297  mov          rdx, qword ptr [rsp+B8h]
00007FF66DDC129F  cmp          rdx, r14
00007FF66DDC12A2  jbe          static void Fabled_Engine::main+14ADh (00007FF66DDC24ADh)
00007FF66DDC12A8  mov          rsi, qword ptr [rsp+A8h]
00007FF66DDC12B0  movaps       xmmword ptr [rsi+r12*1], xmm10
00007FF66DDC12B5  cmp          qword ptr [rsp+68h], r14
00007FF66DDC12BA  je           static void Fabled_Engine::main+14BEh (00007FF66DDC24BEh)
00007FF66DDC12C0  movss        dword ptr [rbp+r14*8], xmm7
00007FF66DDC12C7  mov          dword ptr [rbp+r14*8+4h], 3F800000h
00007FF66DDC12D0  mov          rax, qword ptr [rsp+98h]
00007FF66DDC12D8  cmp          rax, r14
00007FF66DDC12DB  jbe          static void Fabled_Engine::main+14D4h (00007FF66DDC24D4h)
00007FF66DDC12E1  mov          rdi, qword ptr [rsp+88h]
00007FF66DDC12E9  movaps       xmmword ptr [rdi+r12*1], xmm10
00007FF66DDC12EE  lea          rcx, [r14+2D5h]
00007FF66DDC12F5  cmp          rdx, rcx
00007FF66DDC12F8  jbe          static void Fabled_Engine::main+14E8h (00007FF66DDC24E8h)
00007FF66DDC12FE  xorps        xmm0, xmm0
00007FF66DDC1301  subps        xmm0, xmmword ptr [rsi+r12*1]
00007FF66DDC1306  movaps       xmmword ptr [rsi+r12*1+2D50h], xmm0
00007FF66DDC130F  cmp          r15, r14
00007FF66DDC1312  je           static void Fabled_Engine::main+14F6h (00007FF66DDC24F6h)
00007FF66DDC1318  movss        dword ptr [rbp+r14*8+16A8h], xmm7
00007FF66DDC1322  mov          dword ptr [rbp+r14*8+16ACh], 0h
00007FF66DDC132E  cmp          rax, rcx
00007FF66DDC1331  jbe          static void Fabled_Engine::main+1509h (00007FF66DDC2509h)
00007FF66DDC1337  add          r14, 1h
00007FF66DDC133B  movaps       xmmword ptr [rdi+r12*1+2D50h], xmm11
00007FF66DDC1344  add          r12, 10h
00007FF66DDC1348  cmp          r14, 20h
00007FF66DDC134C  jne          static void Fabled_Engine::main+200h (00007FF66DDC1200h)
00007FF66DDC1352  mov          r14, qword ptr [rsp+30h]
00007FF66DDC1357  mov          rdx, qword ptr [rsp+80h]
00007FF66DDC135F  cmp          rdx, 107h
00007FF66DDC1366  mov          r8d, 107h
00007FF66DDC136C  cmova        r8, rdx
00007FF66DDC1370  mov          r9, qword ptr [rsp+40h]
00007FF66DDC1375  mov          r15, qword ptr [rsp+48h]
00007FF66DDC137A  add          r8, FFFFFFFFFFFFFEF9h
00007FF66DDC1381  cmp          rdx, 1CDh
00007FF66DDC1388  mov          r11d, 1CDh
00007FF66DDC138E  cmova        r11, rdx
00007FF66DDC1392  mov          r10, qword ptr [rsp+58h]
00007FF66DDC1397  mov          r12, qword ptr [rsp+70h]
00007FF66DDC139C  add          r11, FFFFFFFFFFFFFE33h
00007FF66DDC13A3  xor          edi, edi
00007FF66DDC13A5  movaps       xmm10, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF66DDDD560h)]
00007FF66DDC13AD  movss        xmm0, dword ptr [__real@3f000000 (00007FF66DDDD528h)]
00007FF66DDC13B5  movss        xmm1, dword ptr [__real@bf000000 (00007FF66DDDD570h)]
00007FF66DDC13BD  xor          ebp, ebp
00007FF66DDC13BF  nop
00007FF66DDC13C0  test         rbp, rbp
00007FF66DDC13C3  js           static void Fabled_Engine::main+3D0h (00007FF66DDC13D0h)
00007FF66DDC13C5  xorps        xmm2, xmm2
00007FF66DDC13C8  cvtsi2ss     xmm2, rbp
00007FF66DDC13CD  jmp          static void Fabled_Engine::main+3EAh (00007FF66DDC13EAh)
00007FF66DDC13CF  nop
00007FF66DDC13D0  mov          rax, rbp
00007FF66DDC13D3  shr          rax, 1h
00007FF66DDC13D6  mov          ecx, ebp
00007FF66DDC13D8  and          ecx, 1h
00007FF66DDC13DB  or           rcx, rax
00007FF66DDC13DE  xorps        xmm2, xmm2
00007FF66DDC13E1  cvtsi2ss     xmm2, rcx
00007FF66DDC13E6  addss        xmm2, xmm2
00007FF66DDC13EA  mulss        xmm2, xmm13
00007FF66DDC13EF  addss        xmm2, xmm9
00007FF66DDC13F4  mov          rax, qword ptr [rsp+A0h]
00007FF66DDC13FC  movss        dword ptr [rax+rbp*4], xmm2
00007FF66DDC1401  mov          esi, ebp
00007FF66DDC1403  and          esi, 1Fh
00007FF66DDC1406  cmp          r9, rsi
00007FF66DDC1409  jbe          static void Fabled_Engine::main+151Ah (00007FF66DDC251Ah)
00007FF66DDC140F  cmp          r10, rsi
00007FF66DDC1412  jbe          static void Fabled_Engine::main+152Eh (00007FF66DDC252Eh)
00007FF66DDC1418  lea          rcx, [rbp+107h]
00007FF66DDC141F  mov          r13, qword ptr [rsp+B8h]
00007FF66DDC1427  cmp          r13, rcx
00007FF66DDC142A  jbe          static void Fabled_Engine::main+1542h (00007FF66DDC2542h)
00007FF66DDC1430  movss        xmm6, dword ptr [r14+rsi*8]
00007FF66DDC1436  movss        xmm5, dword ptr [r14+rsi*8+4h]
00007FF66DDC143D  movss        xmm4, dword ptr [r15+rsi*8]
00007FF66DDC1443  movss        xmm3, dword ptr [r15+rsi*8+4h]
00007FF66DDC144A  xorps        xmm3, xmm10
00007FF66DDC144E  mov          rax, qword ptr [rsp+A8h]
00007FF66DDC1456  movaps       xmm7, xmm0
00007FF66DDC1459  movlhps      xmm7, xmm4
00007FF66DDC145C  shufps       xmm7, xmm3, 2h
00007FF66DDC1460  movaps       xmmword ptr [rax+rdi*4+1070h], xmm7
00007FF66DDC1468  cmp          r8, rbp
00007FF66DDC146B  je           static void Fabled_Engine::main+1553h (00007FF66DDC2553h)
00007FF66DDC1471  movss        dword ptr [r12+rbp*8+838h], xmm2
00007FF66DDC147B  mov          dword ptr [r12+rbp*8+83Ch], 3F400000h
00007FF66DDC1487  mov          rsi, qword ptr [rsp+98h]
00007FF66DDC148F  cmp          rsi, rcx
00007FF66DDC1492  jbe          static void Fabled_Engine::main+1561h (00007FF66DDC2561h)
00007FF66DDC1498  xorps        xmm5, xmm10
00007FF66DDC149C  mov          rbx, qword ptr [rsp+88h]
00007FF66DDC14A4  xorps        xmm7, xmm7
00007FF66DDC14A7  movss        xmm7, xmm6
00007FF66DDC14AB  shufps       xmm7, xmm5, 4h
00007FF66DDC14AF  movaps       xmmword ptr [rbx+rdi*4+1070h], xmm7
00007FF66DDC14B7  lea          rcx, [rbp+1CDh]
00007FF66DDC14BE  cmp          r13, rcx
00007FF66DDC14C1  jbe          static void Fabled_Engine::main+156Ah (00007FF66DDC256Ah)
00007FF66DDC14C7  movaps       xmm5, xmm1
00007FF66DDC14CA  movlhps      xmm5, xmm4
00007FF66DDC14CD  shufps       xmm5, xmm3, 2h
00007FF66DDC14D1  movaps       xmmword ptr [rax+rdi*4+1CD0h], xmm5
00007FF66DDC14D9  cmp          r11, rbp
00007FF66DDC14DC  je           static void Fabled_Engine::main+157Bh (00007FF66DDC257Bh)
00007FF66DDC14E2  movss        dword ptr [r12+rbp*8+E68h], xmm2
00007FF66DDC14EC  mov          dword ptr [r12+rbp*8+E6Ch], 3E800000h
00007FF66DDC14F8  cmp          rsi, rcx
00007FF66DDC14FB  jbe          static void Fabled_Engine::main+1589h (00007FF66DDC2589h)
00007FF66DDC1501  add          rbp, 1h
00007FF66DDC1505  movaps       xmm2, xmmword ptr [rbx+rdi*4+1070h]
00007FF66DDC150D  movaps       xmmword ptr [rbx+rdi*4+1CD0h], xmm2
00007FF66DDC1515  add          rdi, 4h
00007FF66DDC1519  cmp          rdi, 84h
00007FF66DDC1520  jne          static void Fabled_Engine::main+3C0h (00007FF66DDC13C0h)
00007FF66DDC1526  mov          r14, qword ptr [rsp+30h]
00007FF66DDC152B  mov          rax, qword ptr [rsp+40h]
00007FF66DDC1530  mov          qword ptr [rsp+158h], rax
00007FF66DDC1538  mov          r15d, F74h
00007FF66DDC153E  add          r15, qword ptr [rsp+70h]
00007FF66DDC1543  mov          r13, qword ptr [rsp+80h]
00007FF66DDC154B  mov          esi, 3DCh
00007FF66DDC1550  xor          r12d, r12d
00007FF66DDC1553  xor          edx, edx
00007FF66DDC1555  nop          word ptr cs:[rax+rax*1], ax
00007FF66DDC155F  nop
00007FF66DDC1560  test         rdx, rdx
00007FF66DDC1563  js           static void Fabled_Engine::main+570h (00007FF66DDC1570h)
00007FF66DDC1565  xorps        xmm7, xmm7
00007FF66DDC1568  cvtsi2ss     xmm7, rdx
00007FF66DDC156D  jmp          static void Fabled_Engine::main+58Ah (00007FF66DDC158Ah)
00007FF66DDC156F  nop
00007FF66DDC1570  mov          rax, rdx
00007FF66DDC1573  shr          rax, 1h
00007FF66DDC1576  mov          ecx, edx
00007FF66DDC1578  and          ecx, 1h
00007FF66DDC157B  or           rcx, rax
00007FF66DDC157E  xorps        xmm7, xmm7
00007FF66DDC1581  cvtsi2ss     xmm7, rcx
00007FF66DDC1586  addss        xmm7, xmm7
00007FF66DDC158A  add          rdx, 1h
00007FF66DDC158E  mov          qword ptr [rsp+68h], rdx
00007FF66DDC1593  addss        xmm7, xmm9
00007FF66DDC1598  movaps       xmm13, xmm7
00007FF66DDC159C  mulss        xmm13, xmm12
00007FF66DDC15A1  movaps       xmm0, xmm13
00007FF66DDC15A5  call         sinf (00007FF66DDDB419h)
00007FF66DDC15AA  movaps       xmm14, xmm0
00007FF66DDC15AE  movaps       xmm0, xmm13
00007FF66DDC15B2  call         cosf (00007FF66DDDB41Fh)
00007FF66DDC15B7  movaps       xmm1, xmm0
00007FF66DDC15BA  xorps        xmm1, xmm10
00007FF66DDC15BE  movaps       xmmword ptr [rsp+F0h], xmm1
00007FF66DDC15C6  movaps       xmm1, xmm10
00007FF66DDC15CA  movaps       xmm10, xmm14
00007FF66DDC15CE  mulss        xmm10, xmm8
00007FF66DDC15D3  movaps       xmm4, xmm0
00007FF66DDC15D6  mulss        xmm4, xmm8
00007FF66DDC15DB  movaps       xmm6, xmm9
00007FF66DDC15DF  movaps       xmm2, xmm4
00007FF66DDC15E2  addss        xmm2, xmm8
00007FF66DDC15E7  movaps       xmmword ptr [rsp+C0h], xmm2
00007FF66DDC15EF  movss        xmm13, dword ptr [__real@bf000000 (00007FF66DDDD570h)]
00007FF66DDC15F8  subss        xmm13, xmm10
00007FF66DDC15FD  mulss        xmm7, dword ptr [__real@3e000000 (00007FF66DDDD574h)]
00007FF66DDC1605  subss        xmm6, xmm7
00007FF66DDC1609  mulss        xmm7, dword ptr [__real@3f400000 (00007FF66DDDD578h)]
00007FF66DDC1611  addss        xmm7, xmm6
00007FF66DDC1615  mulss        xmm6, dword ptr [__real@3e800000 (00007FF66DDDD57Ch)]
00007FF66DDC161D  movaps       xmm2, xmm10
00007FF66DDC1621  xorps        xmm2, xmm1
00007FF66DDC1624  movaps       xmm15, xmm14
00007FF66DDC1628  xorps        xmm14, xmm1
00007FF66DDC162C  movaps       xmm3, xmm4
00007FF66DDC162F  xorps        xmm3, xmm1
00007FF66DDC1632  mov          r8d, 84h
00007FF66DDC1638  mov          r11, rsi
00007FF66DDC163B  xor          ebp, ebp
00007FF66DDC163D  mov          r9, qword ptr [rsp+A0h]
00007FF66DDC1645  mov          r10, qword ptr [rsp+158h]
00007FF66DDC164D  nop          dword ptr [rax], eax
00007FF66DDC1650  mov          eax, ebp
00007FF66DDC1652  and          eax, 1Fh
00007FF66DDC1655  cmp          r10, rax
00007FF66DDC1658  jbe          static void Fabled_Engine::main+13AFh (00007FF66DDC23AFh)
00007FF66DDC165E  lea          rcx, [r12+rbp*1]
00007FF66DDC1662  add          rcx, 20h
00007FF66DDC1666  mov          rdx, qword ptr [rsp+B8h]
00007FF66DDC166E  cmp          rdx, rcx
00007FF66DDC1671  jbe          static void Fabled_Engine::main+13C3h (00007FF66DDC23C3h)
00007FF66DDC1677  movss        xmm8, dword ptr [r9+rbp*4]
00007FF66DDC167D  movss        xmm12, dword ptr [r14+rax*8]
00007FF66DDC1683  movss        xmm11, dword ptr [r14+rax*8+4h]
00007FF66DDC168A  movaps       xmm9, xmm11
00007FF66DDC168E  mulss        xmm9, xmm2
00007FF66DDC1693  mov          rdi, qword ptr [rsp+A8h]
00007FF66DDC169B  movaps       xmm1, xmm10
00007FF66DDC169F  mulss        xmm1, xmm12
00007FF66DDC16A4  unpcklps     xmm1, xmmword ptr [rsp+C0h]
00007FF66DDC16AC  shufps       xmm1, xmm9, 4h
00007FF66DDC16B1  movaps       xmmword ptr [rdi+rsi*8-1CE0h], xmm1
00007FF66DDC16B9  cmp          r13, rcx
00007FF66DDC16BC  jbe          static void Fabled_Engine::main+13D1h (00007FF66DDC23D1h)
00007FF66DDC16C2  movss        dword ptr [r15+rbp*8-E74h], xmm8
00007FF66DDC16CC  movss        dword ptr [r15+rbp*8-E70h], xmm7
00007FF66DDC16D6  mov          rax, qword ptr [rsp+98h]
00007FF66DDC16DE  cmp          rax, rcx
00007FF66DDC16E1  jbe          static void Fabled_Engine::main+13E2h (00007FF66DDC23E2h)
00007FF66DDC16E7  movaps       xmm1, xmm11
00007FF66DDC16EB  mulss        xmm1, xmm14
00007FF66DDC16F0  mov          rbx, qword ptr [rsp+88h]
00007FF66DDC16F8  movaps       xmm5, xmm15
00007FF66DDC16FC  mulss        xmm5, xmm12
00007FF66DDC1701  unpcklps     xmm5, xmm0
00007FF66DDC1704  shufps       xmm5, xmm1, 4h
00007FF66DDC1708  movaps       xmmword ptr [rbx+rsi*8-1CE0h], xmm5
00007FF66DDC1710  lea          rcx, [r12+rbp*1]
00007FF66DDC1714  add          rcx, 1EEh
00007FF66DDC171B  cmp          rdx, rcx
00007FF66DDC171E  jbe          static void Fabled_Engine::main+13F3h (00007FF66DDC23F3h)
00007FF66DDC1724  movaps       xmm1, xmm11
00007FF66DDC1728  mulss        xmm1, xmm3
00007FF66DDC172C  movaps       xmm5, xmm4
00007FF66DDC172F  mulss        xmm5, xmm12
00007FF66DDC1734  unpcklps     xmm5, xmm13
00007FF66DDC1738  shufps       xmm5, xmm1, 4h
00007FF66DDC173C  movaps       xmmword ptr [rdi+rsi*8], xmm5
00007FF66DDC1740  cmp          r13, rcx
00007FF66DDC1743  jbe          static void Fabled_Engine::main+1401h (00007FF66DDC2401h)
00007FF66DDC1749  movss        dword ptr [r15+rbp*8-4h], xmm8
00007FF66DDC1750  movss        dword ptr [r15+rbp*8], xmm6
00007FF66DDC1756  cmp          rax, rcx
00007FF66DDC1759  jbe          static void Fabled_Engine::main+1412h (00007FF66DDC2412h)
00007FF66DDC175F  add          rbp, 1h
00007FF66DDC1763  mulss        xmm11, dword ptr [rsp+F0h]
00007FF66DDC176D  mulss        xmm12, xmm0
00007FF66DDC1772  unpcklps     xmm12, xmm14
00007FF66DDC1776  shufps       xmm12, xmm11, 4h
00007FF66DDC177B  movaps       xmmword ptr [rbx+rsi*8], xmm12
00007FF66DDC1780  add          rsi, 2h
00007FF66DDC1784  add          r8, FFFFFFFFFFFFFFFCh
00007FF66DDC1788  jne          static void Fabled_Engine::main+650h (00007FF66DDC1650h)
00007FF66DDC178E  add          r12, 21h
00007FF66DDC1792  mov          rsi, r11
00007FF66DDC1795  add          rsi, 42h
00007FF66DDC1799  add          r15, 108h
00007FF66DDC17A0  mov          rdx, qword ptr [rsp+68h]
00007FF66DDC17A5  cmp          rdx, 7h
00007FF66DDC17A9  movss        xmm8, dword ptr [__real@3f000000 (00007FF66DDDD528h)]
00007FF66DDC17B2  movss        xmm9, dword ptr [__real@3f800000 (00007FF66DDDD524h)]
00007FF66DDC17BB  movaps       xmm10, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF66DDDD560h)]
00007FF66DDC17C3  movss        xmm12, dword ptr [__real@3e490fdb (00007FF66DDDD530h)]
00007FF66DDC17CC  jne          static void Fabled_Engine::main+560h (00007FF66DDC1560h)
00007FF66DDC17D2  movss        xmm0, dword ptr [__real@5f000000 (00007FF66DDDD580h)]
00007FF66DDC17DA  movaps       xmm2, xmmword ptr [rsp+180h]
00007FF66DDC17E2  movaps       xmm1, xmm2
00007FF66DDC17E5  subss        xmm1, xmm0
00007FF66DDC17E9  cvttss2si    rax, xmm1
00007FF66DDC17EE  mov          rcx, 8000000000000000h
00007FF66DDC17F8  xor          rcx, rax
00007FF66DDC17FB  cvttss2si    rax, xmm2
00007FF66DDC1800  ucomiss      xmm2, xmm0
00007FF66DDC1803  cmovae       rax, rcx
00007FF66DDC1807  xor          ecx, ecx
00007FF66DDC1809  xorps        xmm0, xmm0
00007FF66DDC180C  ucomiss      xmm2, xmm0
00007FF66DDC180F  cmovae       rcx, rax
00007FF66DDC1813  ucomiss      xmm2, dword ptr [__real@5f7fffff (00007FF66DDDD584h)]
00007FF66DDC181A  mov          rax, FFFFFFFFFFFFFFFFh
00007FF66DDC1821  cmovbe       rax, rcx
00007FF66DDC1825  add          rax, rax
00007FF66DDC1828  lea          rax, [rax+rax*2]
00007FF66DDC182C  mov          qword ptr [rsp+F0h], rax
00007FF66DDC1834  test         rax, rax
00007FF66DDC1837  je           static void Fabled_Engine::main+9CDh (00007FF66DDC19CDh)
00007FF66DDC183D  mov          rbx, qword ptr [rsp+30h]
00007FF66DDC1842  mov          rdx, qword ptr [rsp+40h]
00007FF66DDC1847  mov          rdi, qword ptr [rsp+48h]
00007FF66DDC184C  mov          r14, qword ptr [rsp+58h]
00007FF66DDC1851  mov          r10, qword ptr [rsp+70h]
00007FF66DDC1856  mov          rax, qword ptr [rsp+80h]
00007FF66DDC185E  mov          qword ptr [rsp+C0h], rax
00007FF66DDC1866  add          r10, 4h
00007FF66DDC186A  mov          r11d, 1h
00007FF66DDC1870  mov          r15d, 128h
00007FF66DDC1876  movss        xmm0, dword ptr [__real@3e2aaaab (00007FF66DDDD588h)]
00007FF66DDC187E  nop
00007FF66DDC1880  test         r11, r11
00007FF66DDC1883  js           static void Fabled_Engine::main+890h (00007FF66DDC1890h)
00007FF66DDC1885  xorps        xmm1, xmm1
00007FF66DDC1888  cvtsi2ss     xmm1, r11
00007FF66DDC188D  jmp          static void Fabled_Engine::main+8ABh (00007FF66DDC18ABh)
00007FF66DDC188F  nop
00007FF66DDC1890  mov          rax, r11
00007FF66DDC1893  shr          rax, 1h
00007FF66DDC1896  mov          ecx, r11d
00007FF66DDC1899  and          ecx, 1h
00007FF66DDC189C  or           rcx, rax
00007FF66DDC189F  xorps        xmm1, xmm1
00007FF66DDC18A2  cvtsi2ss     xmm1, rcx
00007FF66DDC18A7  addss        xmm1, xmm1
00007FF66DDC18AB  add          r11, 1h
00007FF66DDC18AF  mulss        xmm1, xmm0
00007FF66DDC18B3  movaps       xmm3, xmm9
00007FF66DDC18B7  subss        xmm3, xmm1
00007FF66DDC18BB  mulss        xmm3, dword ptr [__real@3f400000 (00007FF66DDDD578h)]
00007FF66DDC18C3  movaps       xmm2, xmm8
00007FF66DDC18C7  subss        xmm2, xmm1
00007FF66DDC18CB  mulss        xmm1, dword ptr [__real@3e800000 (00007FF66DDDD57Ch)]
00007FF66DDC18D3  addss        xmm1, xmm3
00007FF66DDC18D7  mov          r13, qword ptr [rsp+C0h]
00007FF66DDC18DF  sub          r13, r15
00007FF66DDC18E2  mov          eax, 0h
00007FF66DDC18E7  cmovb        r13, rax
00007FF66DDC18EB  mov          rax, r15
00007FF66DDC18EE  shl          rax, 4h
00007FF66DDC18F2  lea          r12, [r10+r15*8]
00007FF66DDC18F6  mov          r9d, 84h
00007FF66DDC18FC  xor          r8d, r8d
00007FF66DDC18FF  nop
00007FF66DDC1900  mov          ebp, r8d
00007FF66DDC1903  and          ebp, 1Fh
00007FF66DDC1906  cmp          rdx, rbp
00007FF66DDC1909  jbe          static void Fabled_Engine::main+1423h (00007FF66DDC2423h)
00007FF66DDC190F  cmp          r14, rbp
00007FF66DDC1912  jbe          static void Fabled_Engine::main+1434h (00007FF66DDC2434h)
00007FF66DDC1918  lea          rcx, [r15+r8*1]
00007FF66DDC191C  mov          rsi, qword ptr [rsp+B8h]
00007FF66DDC1924  cmp          rsi, rcx
00007FF66DDC1927  jbe          static void Fabled_Engine::main+1448h (00007FF66DDC2448h)
00007FF66DDC192D  mov          rsi, qword ptr [rsp+A0h]
00007FF66DDC1935  movss        xmm5, dword ptr [rsi+r8*4]
00007FF66DDC193B  movss        xmm4, dword ptr [rbx+rbp*8]
00007FF66DDC1940  movss        xmm3, dword ptr [rbx+rbp*8+4h]
00007FF66DDC1946  movss        xmm6, dword ptr [rdi+rbp*8+4h]
00007FF66DDC194C  xorps        xmm6, xmm10
00007FF66DDC1950  mov          rsi, qword ptr [rsp+A8h]
00007FF66DDC1958  movss        xmm7, dword ptr [rdi+rbp*8]
00007FF66DDC195D  unpcklps     xmm7, xmm2
00007FF66DDC1960  shufps       xmm7, xmm6, 4h
00007FF66DDC1964  movaps       xmmword ptr [rsi+rax*1], xmm7
00007FF66DDC1968  cmp          r13, r8
00007FF66DDC196B  je           static void Fabled_Engine::main+1454h (00007FF66DDC2454h)
00007FF66DDC1971  movss        dword ptr [r12+r8*8-4h], xmm5
00007FF66DDC1978  movss        dword ptr [r12+r8*8], xmm1
00007FF66DDC197E  mov          rbp, qword ptr [rsp+98h]
00007FF66DDC1986  cmp          rbp, rcx
00007FF66DDC1989  jbe          static void Fabled_Engine::main+146Ah (00007FF66DDC246Ah)
00007FF66DDC198F  add          r8, 1h
00007FF66DDC1993  xorps        xmm3, xmm10
00007FF66DDC1997  mov          rcx, qword ptr [rsp+88h]
00007FF66DDC199F  xorps        xmm5, xmm5
00007FF66DDC19A2  movss        xmm5, xmm4
00007FF66DDC19A6  shufps       xmm5, xmm3, 4h
00007FF66DDC19AA  movaps       xmmword ptr [rcx+rax*1], xmm5
00007FF66DDC19AE  add          rax, 10h
00007FF66DDC19B2  add          r9, FFFFFFFFFFFFFFFCh
00007FF66DDC19B6  jne          static void Fabled_Engine::main+900h (00007FF66DDC1900h)
00007FF66DDC19BC  add          r15, r8
00007FF66DDC19BF  cmp          r11, qword ptr [rsp+F0h]
00007FF66DDC19C7  jne          static void Fabled_Engine::main+880h (00007FF66DDC1880h)
00007FF66DDC19CD  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC19D4  test         rcx, rcx
00007FF66DDC19D7  jne          static void Fabled_Engine::main+9F1h (00007FF66DDC19F1h)
00007FF66DDC19D9  call         GetProcessHeap (00007FF66DDDA2ECh)
00007FF66DDC19DE  test         rax, rax
00007FF66DDC19E1  je           static void Fabled_Engine::main+18A4h (00007FF66DDC28A4h)
00007FF66DDC19E7  mov          rcx, rax
00007FF66DDC19EA  mov          qword ptr [00007FF66DDE41C8h], rax
00007FF66DDC19F1  mov          r8d, 7E00h
00007FF66DDC19F7  mov          edx, 8h
00007FF66DDC19FC  call         HeapAlloc (00007FF66DDDA2F2h)
00007FF66DDC1A01  test         rax, rax
00007FF66DDC1A04  je           static void Fabled_Engine::main+18A4h (00007FF66DDC28A4h)
00007FF66DDC1A0A  mov          r14, rax
00007FF66DDC1A0D  xor          eax, eax
00007FF66DDC1A0F  movdqa       xmm0, xmmword ptr [__xmm@00000000000000210000000000000020 (00007FF66DDDD590h)]
00007FF66DDC1A17  movdqa       xmm1, xmmword ptr [__xmm@00000000000002b500000000000002d5 (00007FF66DDDD5A0h)]
00007FF66DDC1A1F  mov          rcx, r14
00007FF66DDC1A22  nop          word ptr cs:[rax+rax*1], ax
00007FF66DDC1A2C  nop          dword ptr [rax], eax
00007FF66DDC1A30  mov          qword ptr [rcx], rax
00007FF66DDC1A33  movq         xmm2, rax
00007FF66DDC1A38  pshufd       xmm2, xmm2, 44h
00007FF66DDC1A3D  movdqa       xmm3, xmm2
00007FF66DDC1A41  paddq        xmm3, xmm0
00007FF66DDC1A45  movdqu       xmmword ptr [rcx+8h], xmm3
00007FF66DDC1A4A  paddq        xmm2, xmm1
00007FF66DDC1A4E  movdqu       xmmword ptr [rcx+7B00h], xmm2
00007FF66DDC1A56  lea          rdx, [rax+2B4h]
00007FF66DDC1A5D  mov          qword ptr [rcx+7B10h], rdx
00007FF66DDC1A64  add          rcx, 18h
00007FF66DDC1A68  add          rax, 1h
00007FF66DDC1A6C  cmp          rax, 20h
00007FF66DDC1A70  jne          static void Fabled_Engine::main+A30h (00007FF66DDC1A30h)
00007FF66DDC1A72  lea          r8, [r14+28h]
00007FF66DDC1A76  mov          r11d, 60h
00007FF66DDC1A7C  mov          r15d, A20h
00007FF66DDC1A82  mov          r10d, 1EFh
00007FF66DDC1A88  xor          r9d, r9d
00007FF66DDC1A8B  nop          dword ptr [rax+rax*1], eax
00007FF66DDC1A90  lea          r12, [r8+r15*8]
00007FF66DDC1A94  lea          rsi, [r8+r11*8]
00007FF66DDC1A98  mov          rdi, r10
00007FF66DDC1A9B  xor          ebp, ebp
00007FF66DDC1A9D  nop          dword ptr [rax], eax
00007FF66DDC1AA0  lea          rcx, [r11+rbp*1]
00007FF66DDC1AA4  cmp          rcx, FC0h
00007FF66DDC1AAB  jae          static void Fabled_Engine::main+159Ah (00007FF66DDC259Ah)
00007FF66DDC1AB1  lea          rdx, [rdi-1CFh]
00007FF66DDC1AB8  lea          rax, [rdi-1ADh]
00007FF66DDC1ABF  mov          qword ptr [rsi+rbp*8-28h], rdx
00007FF66DDC1AC4  mov          qword ptr [rsi+rbp*8-20h], rax
00007FF66DDC1AC9  cmp          rcx, FBEh
00007FF66DDC1AD0  jae          static void Fabled_Engine::main+15ADh (00007FF66DDC25ADh)
00007FF66DDC1AD6  lea          rbx, [rdi-1CEh]
00007FF66DDC1ADD  mov          qword ptr [rsi+rbp*8-18h], rbx
00007FF66DDC1AE2  mov          qword ptr [rsi+rbp*8-10h], rdx
00007FF66DDC1AE7  cmp          rcx, FBCh
00007FF66DDC1AEE  jae          static void Fabled_Engine::main+15C8h (00007FF66DDC25C8h)
00007FF66DDC1AF4  lea          rcx, [r15+rbp*1]
00007FF66DDC1AF8  lea          rdx, [rdi-1AEh]
00007FF66DDC1AFF  mov          qword ptr [rsi+rbp*8-8h], rdx
00007FF66DDC1B04  mov          qword ptr [rsi+rbp*8], rax
00007FF66DDC1B08  cmp          rcx, FC0h
00007FF66DDC1B0F  jae          static void Fabled_Engine::main+15E3h (00007FF66DDC25E3h)
00007FF66DDC1B15  lea          rax, [rdi-22h]
00007FF66DDC1B19  mov          qword ptr [r12+rbp*8-28h], rax
00007FF66DDC1B1E  mov          qword ptr [r12+rbp*8-20h], rdi
00007FF66DDC1B23  cmp          rcx, FBEh
00007FF66DDC1B2A  jae          static void Fabled_Engine::main+15F6h (00007FF66DDC25F6h)
00007FF66DDC1B30  lea          rdx, [rdi-21h]
00007FF66DDC1B34  mov          qword ptr [r12+rbp*8-18h], rdx
00007FF66DDC1B39  mov          qword ptr [r12+rbp*8-10h], rax
00007FF66DDC1B3E  cmp          rcx, FBCh
00007FF66DDC1B45  jae          static void Fabled_Engine::main+1611h (00007FF66DDC2611h)
00007FF66DDC1B4B  lea          rax, [rdi-1h]
00007FF66DDC1B4F  mov          qword ptr [r12+rbp*8-8h], rax
00007FF66DDC1B54  mov          qword ptr [r12+rbp*8], rdi
00007FF66DDC1B58  add          rbp, 6h
00007FF66DDC1B5C  add          rdi, 1h
00007FF66DDC1B60  cmp          rbp, C0h
00007FF66DDC1B67  jne          static void Fabled_Engine::main+AA0h (00007FF66DDC1AA0h)
00007FF66DDC1B6D  add          r9, 1h
00007FF66DDC1B71  add          r10, 21h
00007FF66DDC1B75  add          r15, rbp
00007FF66DDC1B78  add          r11, rbp
00007FF66DDC1B7B  cmp          r9, 7h
00007FF66DDC1B7F  jne          static void Fabled_Engine::main+A90h (00007FF66DDC1A90h)
00007FF66DDC1B85  mov          ecx, 5A5h
00007FF66DDC1B8A  mov          edx, 129h
00007FF66DDC1B8F  xor          edi, edi
00007FF66DDC1B91  nop          word ptr cs:[rax+rax*1], ax
00007FF66DDC1B9B  nop          dword ptr [rax+rax*1], eax
00007FF66DDC1BA0  lea          rax, [rcx-5h]
00007FF66DDC1BA4  cmp          rax, FBFh
00007FF66DDC1BAA  ja           static void Fabled_Engine::main+1799h (00007FF66DDC2799h)
00007FF66DDC1BB0  lea          rbx, [rdx-22h]
00007FF66DDC1BB4  mov          qword ptr [r14+rcx*8-28h], rbx
00007FF66DDC1BB9  lea          rbp, [rcx-4h]
00007FF66DDC1BBD  cmp          rbp, FBFh
00007FF66DDC1BC4  ja           static void Fabled_Engine::main+17E4h (00007FF66DDC27E4h)
00007FF66DDC1BCA  mov          qword ptr [r14+rcx*8-20h], rdx
00007FF66DDC1BCF  cmp          rax, FBDh
00007FF66DDC1BD5  ja           static void Fabled_Engine::main+17AFh (00007FF66DDC27AFh)
00007FF66DDC1BDB  lea          rbp, [rdx-21h]
00007FF66DDC1BDF  mov          qword ptr [r14+rcx*8-18h], rbp
00007FF66DDC1BE4  mov          qword ptr [r14+rcx*8-10h], rbx
00007FF66DDC1BE9  cmp          rax, FBBh
00007FF66DDC1BEF  ja           static void Fabled_Engine::main+17C6h (00007FF66DDC27C6h)
00007FF66DDC1BF5  lea          rax, [rdx-1h]
00007FF66DDC1BF9  mov          qword ptr [r14+rcx*8-8h], rax
00007FF66DDC1BFE  mov          qword ptr [r14+rcx*8], rdx
00007FF66DDC1C02  add          rdi, FFFFFFFFFFFFFFD0h
00007FF66DDC1C06  add          rcx, 6h
00007FF66DDC1C0A  add          rdx, 1h
00007FF66DDC1C0E  cmp          rcx, 665h
00007FF66DDC1C15  jne          static void Fabled_Engine::main+BA0h (00007FF66DDC1BA0h)
00007FF66DDC1C17  mov          edx, 14Ah
00007FF66DDC1C1C  nop          dword ptr [rax], eax
00007FF66DDC1C20  lea          rax, [rcx-5h]
00007FF66DDC1C24  cmp          rax, FBFh
00007FF66DDC1C2A  ja           static void Fabled_Engine::main+1799h (00007FF66DDC2799h)
00007FF66DDC1C30  lea          rbx, [rdx-22h]
00007FF66DDC1C34  mov          qword ptr [r14+rcx*8-28h], rbx
00007FF66DDC1C39  lea          rbp, [rcx-4h]
00007FF66DDC1C3D  cmp          rbp, FBFh
00007FF66DDC1C44  ja           static void Fabled_Engine::main+17E4h (00007FF66DDC27E4h)
00007FF66DDC1C4A  mov          qword ptr [r14+rcx*8-20h], rdx
00007FF66DDC1C4F  cmp          rax, FBDh
00007FF66DDC1C55  ja           static void Fabled_Engine::main+17AFh (00007FF66DDC27AFh)
00007FF66DDC1C5B  lea          rbp, [rdx-21h]
00007FF66DDC1C5F  mov          qword ptr [r14+rcx*8-18h], rbp
00007FF66DDC1C64  mov          qword ptr [r14+rcx*8-10h], rbx
00007FF66DDC1C69  cmp          rax, FBBh
00007FF66DDC1C6F  ja           static void Fabled_Engine::main+17C6h (00007FF66DDC27C6h)
00007FF66DDC1C75  lea          rax, [rdx-1h]
00007FF66DDC1C79  mov          qword ptr [r14+rcx*8-8h], rax
00007FF66DDC1C7E  mov          qword ptr [r14+rcx*8], rdx
00007FF66DDC1C82  add          rcx, 6h
00007FF66DDC1C86  add          rdi, FFFFFFFFFFFFFFD0h
00007FF66DDC1C8A  add          rdx, 1h
00007FF66DDC1C8E  cmp          rdx, 16Ah
00007FF66DDC1C95  jne          static void Fabled_Engine::main+C20h (00007FF66DDC1C20h)
00007FF66DDC1C97  mov          edx, 16Bh
00007FF66DDC1C9C  nop          dword ptr [rax], eax
00007FF66DDC1CA0  lea          rax, [rcx-5h]
00007FF66DDC1CA4  cmp          rax, FBFh
00007FF66DDC1CAA  ja           static void Fabled_Engine::main+1799h (00007FF66DDC2799h)
00007FF66DDC1CB0  lea          rbx, [rdx-22h]
00007FF66DDC1CB4  mov          qword ptr [r14+rcx*8-28h], rbx
00007FF66DDC1CB9  lea          rbp, [rcx-4h]
00007FF66DDC1CBD  cmp          rbp, FBFh
00007FF66DDC1CC4  ja           static void Fabled_Engine::main+17E4h (00007FF66DDC27E4h)
00007FF66DDC1CCA  mov          qword ptr [r14+rcx*8-20h], rdx
00007FF66DDC1CCF  cmp          rax, FBDh
00007FF66DDC1CD5  ja           static void Fabled_Engine::main+17AFh (00007FF66DDC27AFh)
00007FF66DDC1CDB  lea          rbp, [rdx-21h]
00007FF66DDC1CDF  mov          qword ptr [r14+rcx*8-18h], rbp
00007FF66DDC1CE4  mov          qword ptr [r14+rcx*8-10h], rbx
00007FF66DDC1CE9  cmp          rax, FBBh
00007FF66DDC1CEF  ja           static void Fabled_Engine::main+17C6h (00007FF66DDC27C6h)
00007FF66DDC1CF5  lea          rax, [rdx-1h]
00007FF66DDC1CF9  mov          qword ptr [r14+rcx*8-8h], rax
00007FF66DDC1CFE  mov          qword ptr [r14+rcx*8], rdx
00007FF66DDC1D02  add          rdi, FFFFFFFFFFFFFFD0h
00007FF66DDC1D06  add          rcx, 6h
00007FF66DDC1D0A  add          rdx, 1h
00007FF66DDC1D0E  cmp          rdx, 18Bh
00007FF66DDC1D15  jne          static void Fabled_Engine::main+CA0h (00007FF66DDC1CA0h)
00007FF66DDC1D17  mov          rdx, r14
00007FF66DDC1D1A  sub          rdx, rdi
00007FF66DDC1D1D  add          rdx, 2D28h
00007FF66DDC1D24  mov          ebp, 18Ch
00007FF66DDC1D29  nop          dword ptr [rax], eax
00007FF66DDC1D30  lea          rax, [rcx-5h]
00007FF66DDC1D34  cmp          rax, FBFh
00007FF66DDC1D3A  ja           static void Fabled_Engine::main+1799h (00007FF66DDC2799h)
00007FF66DDC1D40  lea          rbx, [rbp-22h]
00007FF66DDC1D44  mov          qword ptr [rdx-28h], rbx
00007FF66DDC1D48  cmp          rcx, FC4h
00007FF66DDC1D4F  je           static void Fabled_Engine::main+17DDh (00007FF66DDC27DDh)
00007FF66DDC1D55  mov          qword ptr [rdx-20h], rbp
00007FF66DDC1D59  cmp          rax, FBDh
00007FF66DDC1D5F  ja           static void Fabled_Engine::main+17AFh (00007FF66DDC27AFh)
00007FF66DDC1D65  lea          rdi, [rbp-21h]
00007FF66DDC1D69  mov          qword ptr [rdx-18h], rdi
00007FF66DDC1D6D  cmp          rcx, FC2h
00007FF66DDC1D74  je           static void Fabled_Engine::main+17FAh (00007FF66DDC27FAh)
00007FF66DDC1D7A  mov          qword ptr [rdx-10h], rbx
00007FF66DDC1D7E  cmp          rax, FBBh
00007FF66DDC1D84  ja           static void Fabled_Engine::main+17C6h (00007FF66DDC27C6h)
00007FF66DDC1D8A  lea          rax, [rbp-1h]
00007FF66DDC1D8E  mov          qword ptr [rdx-8h], rax
00007FF66DDC1D92  cmp          rcx, FC0h
00007FF66DDC1D99  je           static void Fabled_Engine::main+1811h (00007FF66DDC2811h)
00007FF66DDC1D9F  mov          qword ptr [rdx], rbp
00007FF66DDC1DA2  add          rcx, 6h
00007FF66DDC1DA6  add          rdx, 30h
00007FF66DDC1DAA  add          rbp, 1h
00007FF66DDC1DAE  cmp          rbp, 1ACh
00007FF66DDC1DB5  jne          static void Fabled_Engine::main+D30h (00007FF66DDC1D30h)
00007FF66DDC1DBB  mov          ebp, 1ADh
00007FF66DDC1DC0  lea          rax, [rcx-5h]
00007FF66DDC1DC4  cmp          rax, FBFh
00007FF66DDC1DCA  ja           static void Fabled_Engine::main+1799h (00007FF66DDC2799h)
00007FF66DDC1DD0  lea          rbx, [rbp-22h]
00007FF66DDC1DD4  mov          qword ptr [rdx-28h], rbx
00007FF66DDC1DD8  cmp          rcx, FC4h
00007FF66DDC1DDF  je           static void Fabled_Engine::main+17DDh (00007FF66DDC27DDh)
00007FF66DDC1DE5  mov          qword ptr [rdx-20h], rbp
00007FF66DDC1DE9  cmp          rax, FBDh
00007FF66DDC1DEF  ja           static void Fabled_Engine::main+17AFh (00007FF66DDC27AFh)
00007FF66DDC1DF5  lea          rdi, [rbp-21h]
00007FF66DDC1DF9  mov          qword ptr [rdx-18h], rdi
00007FF66DDC1DFD  cmp          rcx, FC2h
00007FF66DDC1E04  je           static void Fabled_Engine::main+17FAh (00007FF66DDC27FAh)
00007FF66DDC1E0A  mov          qword ptr [rdx-10h], rbx
00007FF66DDC1E0E  cmp          rax, FBBh
00007FF66DDC1E14  ja           static void Fabled_Engine::main+17C6h (00007FF66DDC27C6h)
00007FF66DDC1E1A  lea          rax, [rbp-1h]
00007FF66DDC1E1E  mov          qword ptr [rdx-8h], rax
00007FF66DDC1E22  cmp          rcx, FC0h
00007FF66DDC1E29  je           static void Fabled_Engine::main+1811h (00007FF66DDC2811h)
00007FF66DDC1E2F  mov          qword ptr [rdx], rbp
00007FF66DDC1E32  add          rdx, 30h
00007FF66DDC1E36  add          rcx, 6h
00007FF66DDC1E3A  add          rbp, 1h
00007FF66DDC1E3E  cmp          rbp, 1CDh
00007FF66DDC1E45  jne          static void Fabled_Engine::main+DC0h (00007FF66DDC1DC0h)
00007FF66DDC1E4B  mov          ebp, 1CEh
00007FF66DDC1E50  lea          rax, [rcx-5h]
00007FF66DDC1E54  cmp          rax, FBFh
00007FF66DDC1E5A  ja           static void Fabled_Engine::main+1799h (00007FF66DDC2799h)
00007FF66DDC1E60  lea          rbx, [rbp-22h]
00007FF66DDC1E64  mov          qword ptr [rdx-28h], rbx
00007FF66DDC1E68  cmp          rcx, FC4h
00007FF66DDC1E6F  je           static void Fabled_Engine::main+17DDh (00007FF66DDC27DDh)
00007FF66DDC1E75  mov          qword ptr [rdx-20h], rbp
00007FF66DDC1E79  cmp          rax, FBDh
00007FF66DDC1E7F  ja           static void Fabled_Engine::main+17AFh (00007FF66DDC27AFh)
00007FF66DDC1E85  lea          rdi, [rbp-21h]
00007FF66DDC1E89  mov          qword ptr [rdx-18h], rdi
00007FF66DDC1E8D  cmp          rcx, FC2h
00007FF66DDC1E94  je           static void Fabled_Engine::main+17FAh (00007FF66DDC27FAh)
00007FF66DDC1E9A  mov          qword ptr [rdx-10h], rbx
00007FF66DDC1E9E  cmp          rax, FBBh
00007FF66DDC1EA4  ja           static void Fabled_Engine::main+17C6h (00007FF66DDC27C6h)
00007FF66DDC1EAA  lea          rax, [rbp-1h]
00007FF66DDC1EAE  mov          qword ptr [rdx-8h], rax
00007FF66DDC1EB2  cmp          rcx, FC0h
00007FF66DDC1EB9  je           static void Fabled_Engine::main+1811h (00007FF66DDC2811h)
00007FF66DDC1EBF  mov          qword ptr [rdx], rbp
00007FF66DDC1EC2  add          rbp, 1h
00007FF66DDC1EC6  add          rcx, 6h
00007FF66DDC1ECA  add          rdx, 30h
00007FF66DDC1ECE  cmp          rbp, 1EEh
00007FF66DDC1ED5  jne          static void Fabled_Engine::main+E50h (00007FF66DDC1E50h)
00007FF66DDC1EDB  mov          qword ptr [rsp+130h], 0h
00007FF66DDC1EE7  mov          dword ptr [rsp+138h], 0h
00007FF66DDC1EF2  mov          qword ptr [rsp+148h], 0h
00007FF66DDC1EFE  mov          dword ptr [rsp+150h], 0h
00007FF66DDC1F09  pxor         xmm0, xmm0
00007FF66DDC1F0D  movdqa       xmmword ptr [rsp+1C0h], xmm0
00007FF66DDC1F16  movdqa       xmmword ptr [rsp+1B0h], xmm0
00007FF66DDC1F1F  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC1F26  test         rcx, rcx
00007FF66DDC1F29  jne          static void Fabled_Engine::main+F43h (00007FF66DDC1F43h)
00007FF66DDC1F2B  call         GetProcessHeap (00007FF66DDDA2ECh)
00007FF66DDC1F30  test         rax, rax
00007FF66DDC1F33  je           static void Fabled_Engine::main+18B5h (00007FF66DDC28B5h)
00007FF66DDC1F39  mov          rcx, rax
00007FF66DDC1F3C  mov          qword ptr [00007FF66DDE41C8h], rax
00007FF66DDC1F43  mov          r8d, BD40h
00007FF66DDC1F49  xor          edx, edx
00007FF66DDC1F4B  call         HeapAlloc (00007FF66DDDA2F2h)
00007FF66DDC1F50  test         rax, rax
00007FF66DDC1F53  je           static void Fabled_Engine::main+18B5h (00007FF66DDC28B5h)
00007FF66DDC1F59  mov          qword ptr [rsp+100h], rax
00007FF66DDC1F61  movaps       xmm0, xmmword ptr [__xmm@000000000000000000000000000002f5 (00007FF66DDDD5B0h)]
00007FF66DDC1F68  movups       xmmword ptr [rsp+108h], xmm0
00007FF66DDC1F70  mov          rcx, qword ptr [rsp+130h]
00007FF66DDC1F78  mov          qword ptr [rsp+D0h], rcx
00007FF66DDC1F80  mov          ecx, dword ptr [rsp+138h]
00007FF66DDC1F87  mov          dword ptr [rsp+D8h], ecx
00007FF66DDC1F8E  mov          rcx, qword ptr [rsp+148h]
00007FF66DDC1F96  mov          qword ptr [rsp+E0h], rcx
00007FF66DDC1F9E  mov          ecx, dword ptr [rsp+150h]
00007FF66DDC1FA5  mov          dword ptr [rsp+E8h], ecx
00007FF66DDC1FAC  movaps       xmm0, xmmword ptr [rsp+1C0h]
00007FF66DDC1FB4  movaps       xmmword ptr [rsp+170h], xmm0
00007FF66DDC1FBC  movaps       xmm0, xmmword ptr [rsp+1B0h]
00007FF66DDC1FC4  movaps       xmmword ptr [rsp+160h], xmm0
00007FF66DDC1FCC  xor          ecx, ecx
00007FF66DDC1FCE  nop
00007FF66DDC1FD0  mov          edx, dword ptr [rsp+D8h]
00007FF66DDC1FD7  mov          dword ptr [rax+rcx*1+8h], edx
00007FF66DDC1FDB  mov          rdx, qword ptr [rsp+D0h]
00007FF66DDC1FE3  mov          qword ptr [rax+rcx*1], rdx
00007FF66DDC1FE7  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF66DDC1FF0  mov          edx, dword ptr [rsp+E8h]
00007FF66DDC1FF7  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF66DDC1FFB  mov          rdx, qword ptr [rsp+E0h]
00007FF66DDC2003  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF66DDC2008  movaps       xmm0, xmmword ptr [rsp+170h]
00007FF66DDC2010  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF66DDC2015  movaps       xmm0, xmmword ptr [rsp+160h]
00007FF66DDC201D  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF66DDC2022  add          rcx, 40h
00007FF66DDC2026  cmp          rcx, BD00h
00007FF66DDC202D  jne          static void Fabled_Engine::main+FD0h (00007FF66DDC1FD0h)
00007FF66DDC202F  mov          edx, dword ptr [rsp+D8h]
00007FF66DDC2036  mov          dword ptr [rax+rcx*1+8h], edx
00007FF66DDC203A  mov          rdx, qword ptr [rsp+D0h]
00007FF66DDC2042  mov          qword ptr [rax+rcx*1], rdx
00007FF66DDC2046  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF66DDC204F  mov          edx, dword ptr [rsp+E8h]
00007FF66DDC2056  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF66DDC205A  mov          rdx, qword ptr [rsp+E0h]
00007FF66DDC2062  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF66DDC2067  movaps       xmm0, xmmword ptr [rsp+170h]
00007FF66DDC206F  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF66DDC2074  movdqa       xmm0, xmmword ptr [rsp+160h]
00007FF66DDC207D  movdqa       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF66DDC2083  mov          qword ptr [rsp+110h], 2F5h
00007FF66DDC208F  mov          r8, qword ptr [rsp+70h]
00007FF66DDC2094  mov          rcx, qword ptr [rsp+80h]
00007FF66DDC209C  mov          rdi, qword ptr [rsp+100h]
00007FF66DDC20A4  mov          esi, 4h
00007FF66DDC20A9  xor          eax, eax
00007FF66DDC20AB  mov          r9, qword ptr [rsp+B8h]
00007FF66DDC20B3  pxor         xmm0, xmm0
00007FF66DDC20B7  nop          word ptr [rax+rax*1], ax
00007FF66DDC20C0  cmp          r9, rax
00007FF66DDC20C3  je           static void Fabled_Engine::main+185Dh (00007FF66DDC285Dh)
00007FF66DDC20C9  cmp          rcx, rax
00007FF66DDC20CC  je           static void Fabled_Engine::main+1871h (00007FF66DDC2871h)
00007FF66DDC20D2  mov          rdx, qword ptr [rsp+98h]
00007FF66DDC20DA  cmp          rdx, rax
00007FF66DDC20DD  jbe          static void Fabled_Engine::main+1882h (00007FF66DDC2882h)
00007FF66DDC20E3  mov          rdx, qword ptr [rsp+A8h]
00007FF66DDC20EB  mov          r10d, dword ptr [rdx+rsi*2]
00007FF66DDC20EF  mov          rdx, qword ptr [rdx+rsi*2-8h]
00007FF66DDC20F4  mov          r11, qword ptr [r8+rax*8]
00007FF66DDC20F8  add          rax, 1h
00007FF66DDC20FC  mov          rbx, qword ptr [rsp+88h]
00007FF66DDC2104  mov          rbp, qword ptr [rbx+rsi*2-8h]
00007FF66DDC2109  mov          ebx, dword ptr [rbx+rsi*2]
00007FF66DDC210C  mov          qword ptr [rdi+rsi*8-20h], rdx
00007FF66DDC2111  mov          dword ptr [rdi+rsi*8-18h], r10d
00007FF66DDC2116  mov          qword ptr [rdi+rsi*8-14h], r11
00007FF66DDC211B  mov          dword ptr [rdi+rsi*8-4h], ebx
00007FF66DDC211F  mov          qword ptr [rdi+rsi*8-Ch], rbp
00007FF66DDC2124  movdqa       xmmword ptr [rdi+rsi*8], xmm0
00007FF66DDC2129  movdqa       xmmword ptr [rdi+rsi*8+10h], xmm0
00007FF66DDC212F  add          rsi, 8h
00007FF66DDC2133  cmp          rax, 2F5h
00007FF66DDC2139  jne          static void Fabled_Engine::main+10C0h (00007FF66DDC20C0h)
00007FF66DDC213B  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC2142  test         rcx, rcx
00007FF66DDC2145  jne          static void Fabled_Engine::main+115Fh (00007FF66DDC215Fh)
00007FF66DDC2147  call         GetProcessHeap (00007FF66DDDA2ECh)
00007FF66DDC214C  test         rax, rax
00007FF66DDC214F  je           static void Fabled_Engine::main+18BCh (00007FF66DDC28BCh)
00007FF66DDC2155  mov          rcx, rax
00007FF66DDC2158  mov          qword ptr [00007FF66DDE41C8h], rax
00007FF66DDC215F  mov          r8d, 40h
00007FF66DDC2165  xor          edx, edx
00007FF66DDC2167  call         HeapAlloc (00007FF66DDDA2F2h)
00007FF66DDC216C  test         rax, rax
00007FF66DDC216F  je           static void Fabled_Engine::main+18BCh (00007FF66DDC28BCh)
00007FF66DDC2175  mov          rbp, rax
00007FF66DDC2178  mov          rax, qword ptr [rsp+110h]
00007FF66DDC2180  mov          qword ptr [rbp+10h], rax
00007FF66DDC2184  movups       xmm0, xmmword ptr [rsp+100h]
00007FF66DDC218C  movups       xmmword ptr [rbp], xmm0
00007FF66DDC2190  mov          qword ptr [rbp+20h], r14
00007FF66DDC2194  movaps       xmm0, xmmword ptr [__xmm@0000000000000fc00000000000000fc0 (00007FF66DDDD5C0h)]
00007FF66DDC219B  movups       xmmword ptr [rbp+28h], xmm0
00007FF66DDC219F  mov          dword ptr [rbp+18h], 0h
00007FF66DDC21A6  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC21AD  xor          edx, edx
00007FF66DDC21AF  mov          r8, qword ptr [rsp+A0h]
00007FF66DDC21B7  call         HeapFree (00007FF66DDDA2F8h)
00007FF66DDC21BC  mov          rax, qword ptr [rsp+50h]
00007FF66DDC21C1  test         rax, rax
00007FF66DDC21C4  je           static void Fabled_Engine::main+11E7h (00007FF66DDC21E7h)
00007FF66DDC21C6  shl          rax, 3h
00007FF66DDC21CA  test         rax, rax
00007FF66DDC21CD  je           static void Fabled_Engine::main+11E7h (00007FF66DDC21E7h)
00007FF66DDC21CF  mov          r8, qword ptr [rsp+48h]
00007FF66DDC21D4  test         r8, r8
00007FF66DDC21D7  je           static void Fabled_Engine::main+11E7h (00007FF66DDC21E7h)
00007FF66DDC21D9  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC21E0  xor          edx, edx
00007FF66DDC21E2  call         HeapFree (00007FF66DDDA2F8h)
00007FF66DDC21E7  mov          rax, qword ptr [rsp+38h]
00007FF66DDC21EC  test         rax, rax
00007FF66DDC21EF  lea          rdi, [rsp+70h]
00007FF66DDC21F4  je           static void Fabled_Engine::main+1217h (00007FF66DDC2217h)
00007FF66DDC21F6  shl          rax, 3h
00007FF66DDC21FA  test         rax, rax
00007FF66DDC21FD  je           static void Fabled_Engine::main+1217h (00007FF66DDC2217h)
00007FF66DDC21FF  mov          r8, qword ptr [rsp+30h]
00007FF66DDC2204  test         r8, r8
00007FF66DDC2207  je           static void Fabled_Engine::main+1217h (00007FF66DDC2217h)
00007FF66DDC2209  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC2210  xor          edx, edx
00007FF66DDC2212  call         HeapFree (00007FF66DDDA2F8h)
00007FF66DDC2217  mov          rax, qword ptr [rsp+90h]
00007FF66DDC221F  test         rax, rax
00007FF66DDC2222  je           static void Fabled_Engine::main+1245h (00007FF66DDC2245h)
00007FF66DDC2224  shl          rax, 4h
00007FF66DDC2228  je           static void Fabled_Engine::main+1245h (00007FF66DDC2245h)
00007FF66DDC222A  mov          r8, qword ptr [rsp+88h]
00007FF66DDC2232  test         r8, r8
00007FF66DDC2235  je           static void Fabled_Engine::main+1245h (00007FF66DDC2245h)
00007FF66DDC2237  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC223E  xor          edx, edx
00007FF66DDC2240  call         HeapFree (00007FF66DDDA2F8h)
00007FF66DDC2245  mov          rax, qword ptr [rsp+78h]
00007FF66DDC224A  test         rax, rax
00007FF66DDC224D  je           static void Fabled_Engine::main+1270h (00007FF66DDC2270h)
00007FF66DDC224F  shl          rax, 3h
00007FF66DDC2253  test         rax, rax
00007FF66DDC2256  je           static void Fabled_Engine::main+1270h (00007FF66DDC2270h)
00007FF66DDC2258  mov          r8, qword ptr [rsp+70h]
00007FF66DDC225D  test         r8, r8
00007FF66DDC2260  je           static void Fabled_Engine::main+1270h (00007FF66DDC2270h)
00007FF66DDC2262  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC2269  xor          edx, edx
00007FF66DDC226B  call         HeapFree (00007FF66DDDA2F8h)
00007FF66DDC2270  mov          rax, qword ptr [rsp+B0h]
00007FF66DDC2278  test         rax, rax
00007FF66DDC227B  je           static void Fabled_Engine::main+129Eh (00007FF66DDC229Eh)
00007FF66DDC227D  shl          rax, 4h
00007FF66DDC2281  je           static void Fabled_Engine::main+129Eh (00007FF66DDC229Eh)
00007FF66DDC2283  mov          r8, qword ptr [rsp+A8h]
00007FF66DDC228B  test         r8, r8
00007FF66DDC228E  je           static void Fabled_Engine::main+129Eh (00007FF66DDC229Eh)
00007FF66DDC2290  mov          rcx, qword ptr [00007FF66DDE41C8h]
00007FF66DDC2297  xor          edx, edx
00007FF66DDC2299  call         HeapFree (00007FF66DDDA2F8h)