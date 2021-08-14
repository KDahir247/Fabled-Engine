# --------------- Cube Dissassembly -------------------
00007FF7F1D810A0  mov          rcx, qword ptr [00007FF7F1DA21C8h]
00007FF7F1D810A7  test         rcx, rcx
00007FF7F1D810AA  jne          static void Fabled_Engine::main+C4h (00007FF7F1D810C4h)
00007FF7F1D810AC  call         GetProcessHeap (00007FF7F1D9850Ch)
00007FF7F1D810B1  test         rax, rax
00007FF7F1D810B4  je           static void Fabled_Engine::main+6AEh (00007FF7F1D816AEh)
00007FF7F1D810BA  mov          rcx, rax
00007FF7F1D810BD  mov          qword ptr [00007FF7F1DA21C8h], rax
00007FF7F1D810C4  mov          r8d, 600h
00007FF7F1D810CA  xor          edx, edx
00007FF7F1D810CC  call         HeapAlloc (00007FF7F1D98512h)
00007FF7F1D810D1  test         rax, rax
00007FF7F1D810D4  je           static void Fabled_Engine::main+6AEh (00007FF7F1D816AEh)
00007FF7F1D810DA  mov          qword ptr [rsp+28h], rax
00007FF7F1D810DF  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000018 (00007FF7F1D9B500h)]
00007FF7F1D810E6  movups       xmmword ptr [rsp+30h], xmm0
00007FF7F1D810EB  lea          r12, [00007FF7F1D9C9F0h]
00007FF7F1D810F2  xor          ebx, ebx
00007FF7F1D810F4  mov          r13, 3F80000000000000h
00007FF7F1D810FE  mov          rax, BF800000BF800000h
00007FF7F1D81108  movq         xmm9, rax
00007FF7F1D8110D  mov          rax, 3F8000003F800000h
00007FF7F1D81117  movq         xmm12, rax
00007FF7F1D8111C  movss        xmm13, dword ptr [__real@3f000000 (00007FF7F1D9B520h)]
00007FF7F1D81125  jmp          static void Fabled_Engine::main+17Fh (00007FF7F1D8117Fh)
00007FF7F1D81127  nop          word ptr [rax+rax*1], ax
00007FF7F1D81130  mov          rsi, qword ptr [rsp+28h]
00007FF7F1D81135  mov          rbp, rdi
00007FF7F1D81138  shl          rbp, 6h
00007FF7F1D8113C  mov          qword ptr [rsi+rbp*1], rbx
00007FF7F1D81140  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF7F1D9B530h)]
00007FF7F1D81147  movups       xmmword ptr [rsi+rbp*1+8h], xmm0
00007FF7F1D8114C  mov          dword ptr [rsi+rbp*1+18h], 0h
00007FF7F1D81154  mov          qword ptr [rsi+rbp*1+20h], r15
00007FF7F1D81159  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF7F1D9B540h)]
00007FF7F1D81160  movups       xmmword ptr [rsi+rbp*1+28h], xmm0
00007FF7F1D81165  add          rdi, 1h
00007FF7F1D81169  mov          qword ptr [rsp+38h], rdi
00007FF7F1D8116E  add          r12, 10h
00007FF7F1D81172  mov          rbx, r14
00007FF7F1D81175  cmp          r14, 12h
00007FF7F1D81179  je           static void Fabled_Engine::main+597h (00007FF7F1D81597h)
00007FF7F1D8117F  movaps       xmm8, xmmword ptr [r12]
00007FF7F1D81184  movaps       xmm7, xmmword ptr [r12+60h]
00007FF7F1D8118A  movaps       xmm14, xmmword ptr [r12+C0h]
00007FF7F1D81193  movaps       xmm0, xmm8
00007FF7F1D81197  subps        xmm0, xmm14
00007FF7F1D8119B  movaps       xmm1, xmm7
00007FF7F1D8119E  addps        xmm1, xmm0
00007FF7F1D811A1  subps        xmm0, xmm7
00007FF7F1D811A4  movaps       xmm4, xmmword ptr [__xmm@3f0000003f0000003f0000003f000000 (00007FF7F1D9B510h)]
00007FF7F1D811AB  mulps        xmm0, xmm4
00007FF7F1D811AE  mulps        xmm1, xmm4
00007FF7F1D811B1  movaps       xmm2, xmm8
00007FF7F1D811B5  addps        xmm2, xmm14
00007FF7F1D811B9  movaps       xmm3, xmm7
00007FF7F1D811BC  addps        xmm3, xmm2
00007FF7F1D811BF  mulps        xmm3, xmm4
00007FF7F1D811C2  subps        xmm2, xmm7
00007FF7F1D811C5  mulps        xmm2, xmm4
00007FF7F1D811C8  movaps       xmmword ptr [rsp+40h], xmm0
00007FF7F1D811CD  movaps       xmmword ptr [rsp+50h], xmm1
00007FF7F1D811D2  movaps       xmmword ptr [rsp+60h], xmm3
00007FF7F1D811D7  movaps       xmmword ptr [rsp+70h], xmm2
00007FF7F1D811DC  mov          rax, qword ptr [00007FF7F1DA21C8h]
00007FF7F1D811E3  test         rax, rax
00007FF7F1D811E6  jne          static void Fabled_Engine::main+1FDh (00007FF7F1D811FDh)
00007FF7F1D811E8  call         GetProcessHeap (00007FF7F1D9850Ch)
00007FF7F1D811ED  test         rax, rax
00007FF7F1D811F0  je           static void Fabled_Engine::main+68Ch (00007FF7F1D8168Ch)
00007FF7F1D811F6  mov          qword ptr [00007FF7F1DA21C8h], rax
00007FF7F1D811FD  mov          r8d, 30h
00007FF7F1D81203  mov          rcx, rax
00007FF7F1D81206  xor          edx, edx
00007FF7F1D81208  call         HeapAlloc (00007FF7F1D98512h)
00007FF7F1D8120D  test         rax, rax
00007FF7F1D81210  je           static void Fabled_Engine::main+68Ch (00007FF7F1D8168Ch)
00007FF7F1D81216  mov          r15, rax
00007FF7F1D81219  lea          rax, [rbx+1h]
00007FF7F1D8121D  lea          rcx, [rbx+2h]
00007FF7F1D81221  lea          r14, [rbx+3h]
00007FF7F1D81225  mov          qword ptr [r15], rbx
00007FF7F1D81228  mov          qword ptr [r15+8h], rax
00007FF7F1D8122C  mov          qword ptr [r15+10h], rcx
00007FF7F1D81230  mov          qword ptr [r15+18h], rcx
00007FF7F1D81234  mov          qword ptr [r15+20h], r14
00007FF7F1D81238  mov          qword ptr [r15+28h], rbx
00007FF7F1D8123C  movaps       xmm11, xmmword ptr [rsp+40h]
00007FF7F1D81242  movaps       xmm10, xmmword ptr [rsp+50h]
00007FF7F1D81248  movaps       xmm6, xmmword ptr [rsp+60h]
00007FF7F1D8124D  movaps       xmm15, xmmword ptr [rsp+70h]
00007FF7F1D81253  mov          rax, qword ptr [00007FF7F1DA21C8h]
00007FF7F1D8125A  test         rax, rax
00007FF7F1D8125D  jne          static void Fabled_Engine::main+274h (00007FF7F1D81274h)
00007FF7F1D8125F  call         GetProcessHeap (00007FF7F1D9850Ch)
00007FF7F1D81264  test         rax, rax
00007FF7F1D81267  je           static void Fabled_Engine::main+69Dh (00007FF7F1D8169Dh)
00007FF7F1D8126D  mov          qword ptr [00007FF7F1DA21C8h], rax
00007FF7F1D81274  mov          r8d, 100h
00007FF7F1D8127A  mov          rcx, rax
00007FF7F1D8127D  xor          edx, edx
00007FF7F1D8127F  call         HeapAlloc (00007FF7F1D98512h)
00007FF7F1D81284  test         rax, rax
00007FF7F1D81287  je           static void Fabled_Engine::main+69Dh (00007FF7F1D8169Dh)
00007FF7F1D8128D  mov          rbx, rax
00007FF7F1D81290  movd         r9d, xmm8
00007FF7F1D81295  movaps       xmm0, xmm8
00007FF7F1D81299  shufps       xmm0, xmm8, 55h
00007FF7F1D8129E  movd         eax, xmm0
00007FF7F1D812A2  punpckhqdq   xmm8, xmm8
00007FF7F1D812A7  movd         r8d, xmm8
00007FF7F1D812AC  shl          rax, 20h
00007FF7F1D812B0  or           r9, rax
00007FF7F1D812B3  movd         edx, xmm7
00007FF7F1D812B7  movaps       xmm0, xmm7
00007FF7F1D812BA  shufps       xmm0, xmm7, 55h
00007FF7F1D812BE  movd         eax, xmm0
00007FF7F1D812C2  punpckhqdq   xmm7, xmm7
00007FF7F1D812C6  shl          rax, 20h
00007FF7F1D812CA  or           rdx, rax
00007FF7F1D812CD  movd         edi, xmm14
00007FF7F1D812D2  movaps       xmm0, xmm14
00007FF7F1D812D6  shufps       xmm0, xmm14, 55h
00007FF7F1D812DB  movd         eax, xmm0
00007FF7F1D812DF  shl          rax, 20h
00007FF7F1D812E3  or           rdi, rax
00007FF7F1D812E6  movaps       xmm0, xmm15
00007FF7F1D812EA  shufps       xmm0, xmm15, 55h
00007FF7F1D812EF  movd         eax, xmm0
00007FF7F1D812F3  shl          rax, 20h
00007FF7F1D812F7  movd         r10d, xmm15
00007FF7F1D812FC  or           r10, rax
00007FF7F1D812FF  movaps       xmm0, xmm6
00007FF7F1D81302  shufps       xmm0, xmm6, 55h
00007FF7F1D81306  movd         eax, xmm0
00007FF7F1D8130A  shl          rax, 20h
00007FF7F1D8130E  movd         ebp, xmm6
00007FF7F1D81312  or           rbp, rax
00007FF7F1D81315  movaps       xmm0, xmm10
00007FF7F1D81319  shufps       xmm0, xmm10, 55h
00007FF7F1D8131E  movd         ecx, xmm0
00007FF7F1D81322  shl          rcx, 20h
00007FF7F1D81326  movd         eax, xmm10
00007FF7F1D8132B  or           rax, rcx
00007FF7F1D8132E  movaps       xmm0, xmm11
00007FF7F1D81332  shufps       xmm0, xmm11, 55h
00007FF7F1D81337  movd         ecx, xmm0
00007FF7F1D8133B  shl          rcx, 20h
00007FF7F1D8133F  movd         esi, xmm11
00007FF7F1D81344  or           rsi, rcx
00007FF7F1D81347  movd         ecx, xmm7
00007FF7F1D8134B  or           rcx, r13
00007FF7F1D8134E  punpckhqdq   xmm14, xmm14
00007FF7F1D81353  xorps        xmm1, xmm1
00007FF7F1D81356  cmpps        xmm1, xmm11, 2h
00007FF7F1D8135B  movaps       xmm0, xmm11
00007FF7F1D8135F  cmpps        xmm0, xmm11, 3h
00007FF7F1D81364  movaps       xmm2, xmm0
00007FF7F1D81367  andps        xmm0, xmm11
00007FF7F1D8136B  punpckhqdq   xmm11, xmm11
00007FF7F1D81370  mov          qword ptr [rbx], rsi
00007FF7F1D81373  movd         esi, xmm11
00007FF7F1D81378  mov          dword ptr [rbx+8h], esi
00007FF7F1D8137B  movd         esi, xmm14
00007FF7F1D81380  or           rsi, r13
00007FF7F1D81383  movaps       xmm3, xmm1
00007FF7F1D81386  andnps       xmm3, xmm9
00007FF7F1D8138A  andps        xmm1, xmm12
00007FF7F1D8138E  orps         xmm1, xmm3
00007FF7F1D81391  andnps       xmm2, xmm1
00007FF7F1D81394  orps         xmm0, xmm2
00007FF7F1D81397  xorps        xmm1, xmm1
00007FF7F1D8139A  cmpps        xmm1, xmm10, 2h
00007FF7F1D8139F  movaps       xmm2, xmm10
00007FF7F1D813A3  cmpps        xmm2, xmm10, 3h
00007FF7F1D813A8  movaps       xmm3, xmm2
00007FF7F1D813AB  andps        xmm2, xmm10
00007FF7F1D813AF  punpckhqdq   xmm10, xmm10
00007FF7F1D813B4  pshufd       xmm4, xmm0, 55h
00007FF7F1D813B9  mulss        xmm4, xmm13
00007FF7F1D813BE  addss        xmm4, xmm13
00007FF7F1D813C3  mulss        xmm0, xmm13
00007FF7F1D813C8  addss        xmm0, xmm13
00007FF7F1D813CD  movss        dword ptr [rbx+Ch], xmm0
00007FF7F1D813D2  movss        dword ptr [rbx+10h], xmm4
00007FF7F1D813D7  mov          qword ptr [rbx+14h], r9
00007FF7F1D813DB  mov          dword ptr [rbx+1Ch], r8d
00007FF7F1D813DF  mov          qword ptr [rbx+20h], rdx
00007FF7F1D813E3  mov          qword ptr [rbx+28h], rcx
00007FF7F1D813E7  mov          qword ptr [rbx+30h], rdi
00007FF7F1D813EB  mov          qword ptr [rbx+38h], rsi
00007FF7F1D813EF  mov          qword ptr [rbx+40h], rax
00007FF7F1D813F3  movd         eax, xmm10
00007FF7F1D813F8  mov          dword ptr [rbx+48h], eax
00007FF7F1D813FB  movaps       xmm0, xmm1
00007FF7F1D813FE  andnps       xmm0, xmm9
00007FF7F1D81402  andps        xmm1, xmm12
00007FF7F1D81406  orps         xmm1, xmm0
00007FF7F1D81409  andnps       xmm3, xmm1
00007FF7F1D8140C  orps         xmm2, xmm3
00007FF7F1D8140F  xorps        xmm0, xmm0
00007FF7F1D81412  cmpps        xmm0, xmm6, 2h
00007FF7F1D81416  movaps       xmm1, xmm6
00007FF7F1D81419  cmpps        xmm1, xmm6, 3h
00007FF7F1D8141D  movaps       xmm3, xmm1
00007FF7F1D81420  andps        xmm1, xmm6
00007FF7F1D81423  punpckhqdq   xmm6, xmm6
00007FF7F1D81427  pshufd       xmm4, xmm2, 55h
00007FF7F1D8142C  mulss        xmm4, xmm13
00007FF7F1D81431  addss        xmm4, xmm13
00007FF7F1D81436  mulss        xmm2, xmm13
00007FF7F1D8143B  addss        xmm2, xmm13
00007FF7F1D81440  movss        dword ptr [rbx+4Ch], xmm2
00007FF7F1D81445  movss        dword ptr [rbx+50h], xmm4
00007FF7F1D8144A  mov          qword ptr [rbx+54h], r9
00007FF7F1D8144E  mov          dword ptr [rbx+5Ch], r8d
00007FF7F1D81452  mov          qword ptr [rbx+60h], rdx
00007FF7F1D81456  mov          qword ptr [rbx+68h], rcx
00007FF7F1D8145A  mov          qword ptr [rbx+70h], rdi
00007FF7F1D8145E  mov          qword ptr [rbx+78h], rsi
00007FF7F1D81462  mov          qword ptr [rbx+80h], rbp
00007FF7F1D81469  movd         eax, xmm6
00007FF7F1D8146D  mov          dword ptr [rbx+88h], eax
00007FF7F1D81473  movaps       xmm2, xmm0
00007FF7F1D81476  andnps       xmm2, xmm9
00007FF7F1D8147A  andps        xmm0, xmm12
00007FF7F1D8147E  orps         xmm0, xmm2
00007FF7F1D81481  andnps       xmm3, xmm0
00007FF7F1D81484  orps         xmm1, xmm3
00007FF7F1D81487  xorps        xmm0, xmm0
00007FF7F1D8148A  cmpps        xmm0, xmm15, 2h
00007FF7F1D8148F  movaps       xmm2, xmm15
00007FF7F1D81493  cmpps        xmm2, xmm15, 3h
00007FF7F1D81498  movaps       xmm3, xmm2
00007FF7F1D8149B  andps        xmm2, xmm15
00007FF7F1D8149F  punpckhqdq   xmm15, xmm15
00007FF7F1D814A4  pshufd       xmm4, xmm1, 55h
00007FF7F1D814A9  mulss        xmm4, xmm13
00007FF7F1D814AE  addss        xmm4, xmm13
00007FF7F1D814B3  mulss        xmm1, xmm13
00007FF7F1D814B8  addss        xmm1, xmm13
00007FF7F1D814BD  movss        dword ptr [rbx+8Ch], xmm1
00007FF7F1D814C5  movss        dword ptr [rbx+90h], xmm4
00007FF7F1D814CD  mov          qword ptr [rbx+94h], r9
00007FF7F1D814D4  mov          dword ptr [rbx+9Ch], r8d
00007FF7F1D814DB  mov          qword ptr [rbx+A0h], rdx
00007FF7F1D814E2  mov          qword ptr [rbx+A8h], rcx
00007FF7F1D814E9  mov          qword ptr [rbx+B0h], rdi
00007FF7F1D814F0  mov          qword ptr [rbx+B8h], rsi
00007FF7F1D814F7  mov          qword ptr [rbx+C0h], r10
00007FF7F1D814FE  movd         eax, xmm15
00007FF7F1D81503  mov          dword ptr [rbx+C8h], eax
00007FF7F1D81509  movaps       xmm1, xmm0
00007FF7F1D8150C  andnps       xmm1, xmm9
00007FF7F1D81510  andps        xmm0, xmm12
00007FF7F1D81514  orps         xmm0, xmm1
00007FF7F1D81517  andnps       xmm3, xmm0
00007FF7F1D8151A  orps         xmm2, xmm3
00007FF7F1D8151D  pshufd       xmm0, xmm2, 55h
00007FF7F1D81522  mulss        xmm0, xmm13
00007FF7F1D81527  addss        xmm0, xmm13
00007FF7F1D8152C  mulss        xmm2, xmm13
00007FF7F1D81531  addss        xmm2, xmm13
00007FF7F1D81536  movss        dword ptr [rbx+CCh], xmm2
00007FF7F1D8153E  movss        dword ptr [rbx+D0h], xmm0
00007FF7F1D81546  mov          qword ptr [rbx+D4h], r9
00007FF7F1D8154D  mov          dword ptr [rbx+DCh], r8d
00007FF7F1D81554  mov          qword ptr [rbx+E0h], rdx
00007FF7F1D8155B  mov          qword ptr [rbx+E8h], rcx
00007FF7F1D81562  mov          qword ptr [rbx+F0h], rdi
00007FF7F1D81569  mov          qword ptr [rbx+F8h], rsi
00007FF7F1D81570  mov          rdi, qword ptr [rsp+38h]
00007FF7F1D81575  cmp          rdi, qword ptr [rsp+30h]
00007FF7F1D8157A  jne          static void Fabled_Engine::main+130h (00007FF7F1D81130h)
00007FF7F1D81580  lea          rcx, [rsp+28h]
00007FF7F1D81585  mov          rdx, rdi
00007FF7F1D81588  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::mesh_data::Mesh,alloc::alloc::Global> (00007FF7F1D9A040h)
00007FF7F1D8158D  mov          rdi, qword ptr [rsp+38h]
00007FF7F1D81592  jmp          static void Fabled_Engine::main+130h (00007FF7F1D81130h)
00007FF7F1D81597  mov          rbx, qword ptr [rsp+30h]
00007FF7F1D8159C  lea          rcx, [rsp+40h]
