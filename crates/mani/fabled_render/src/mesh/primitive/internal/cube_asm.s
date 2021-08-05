# --------------- Cube Dissassembly -------------------
00007FF686571352  mov          rcx, qword ptr [00007FF68659A1C8h]
00007FF686571359  test         rcx, rcx
00007FF68657135C  jne          static void Fabled_Engine::main+B6h (00007FF686571376h)
00007FF68657135E  call         GetProcessHeap (00007FF68658F352h)
00007FF686571363  test         rax, rax
00007FF686571366  je           static void Fabled_Engine::main+97Bh (00007FF686571C3Bh)
00007FF68657136C  mov          rcx, rax
00007FF68657136F  mov          qword ptr [00007FF68659A1C8h], rax
00007FF686571376  mov          r8d, 600h
00007FF68657137C  xor          edx, edx
00007FF68657137E  call         HeapAlloc (00007FF68658F358h)
00007FF686571383  test         rax, rax
00007FF686571386  je           static void Fabled_Engine::main+97Bh (00007FF686571C3Bh)
00007FF68657138C  mov          qword ptr [rsp+28h], rax
00007FF686571391  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000018 (00007FF686592520h)]
00007FF686571398  movups       xmmword ptr [rsp+30h], xmm0
00007FF68657139D  xorps        xmm0, xmm0
00007FF6865713A0  movaps       xmmword ptr [rsp+170h], xmm0
00007FF6865713A8  movaps       xmmword ptr [rsp+160h], xmm0
00007FF6865713B0  movaps       xmmword ptr [rsp+150h], xmm0
00007FF6865713B8  movaps       xmmword ptr [rsp+140h], xmm0
00007FF6865713C0  movaps       xmmword ptr [rsp+130h], xmm0
00007FF6865713C8  movaps       xmmword ptr [rsp+120h], xmm0
00007FF6865713D0  movaps       xmmword ptr [rsp+110h], xmm0
00007FF6865713D8  movaps       xmmword ptr [rsp+100h], xmm0
00007FF6865713E0  movaps       xmmword ptr [rsp+F0h], xmm0
00007FF6865713E8  movaps       xmmword ptr [rsp+E0h], xmm0
00007FF6865713F0  movaps       xmmword ptr [rsp+D0h], xmm0
00007FF6865713F8  movaps       xmmword ptr [rsp+C0h], xmm0
00007FF686571400  movaps       xmmword ptr [rsp+B0h], xmm0
00007FF686571408  movaps       xmmword ptr [rsp+A0h], xmm0
00007FF686571410  movaps       xmmword ptr [rsp+90h], xmm0
00007FF686571418  movaps       xmmword ptr [rsp+80h], xmm0
00007FF686571420  lea          r14, [00007FF686594810h]
00007FF686571427  xor          esi, esi
00007FF686571429  movaps       xmm8, xmmword ptr [__xmm@3f0000003f0000003f0000003f000000 (00007FF686592530h)]
00007FF686571431  mov          r12, 3F80000000000000h
00007FF68657143B  movdqa       xmm9, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF686592540h)]
00007FF686571444  movd         xmm12, dword ptr [__real@3f800000 (00007FF686592550h)]
00007FF68657144D  movss        xmm13, dword ptr [__real@7fc00000 (00007FF686592554h)]
00007FF686571456  lea          r15, [rsp+80h]
00007FF68657145E  movaps       xmm10, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF686592510h)]
00007FF686571466  movaps       xmm11, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF686592560h)]
00007FF68657146E  jmp          static void Fabled_Engine::main+1F3h (00007FF6865714B3h)
00007FF686571470  mov          rax, qword ptr [rsp+28h]
00007FF686571475  mov          rcx, rdx
00007FF686571478  shl          rcx, 6h
00007FF68657147C  mov          qword ptr [rax+rcx*1], rsi
00007FF686571480  movups       xmmword ptr [rax+rcx*1+8h], xmm10
00007FF686571486  mov          dword ptr [rax+rcx*1+18h], 0h
00007FF68657148E  mov          qword ptr [rax+rcx*1+20h], rbx
00007FF686571493  movups       xmmword ptr [rax+rcx*1+28h], xmm11
00007FF686571499  add          rdx, 1h
00007FF68657149D  mov          qword ptr [rsp+38h], rdx
00007FF6865714A2  add          r14, 10h
00007FF6865714A6  mov          rsi, r13
00007FF6865714A9  cmp          r13, 12h
00007FF6865714AD  je           static void Fabled_Engine::main+641h (00007FF686571901h)
00007FF6865714B3  movaps       xmm14, xmmword ptr [r14]
00007FF6865714B7  movaps       xmm6, xmmword ptr [r14+60h]
00007FF6865714BC  movaps       xmm7, xmmword ptr [r14+C0h]
00007FF6865714C4  movaps       xmm0, xmm14
00007FF6865714C8  subps        xmm0, xmm7
00007FF6865714CB  movaps       xmm1, xmm6
00007FF6865714CE  addps        xmm1, xmm0
00007FF6865714D1  subps        xmm0, xmm6
00007FF6865714D4  mulps        xmm0, xmm8
00007FF6865714D8  mulps        xmm1, xmm8
00007FF6865714DC  movaps       xmm2, xmm14
00007FF6865714E0  addps        xmm2, xmm7
00007FF6865714E3  movaps       xmm3, xmm6
00007FF6865714E6  addps        xmm3, xmm2
00007FF6865714E9  mulps        xmm3, xmm8
00007FF6865714ED  subps        xmm2, xmm6
00007FF6865714F0  mulps        xmm2, xmm8
00007FF6865714F4  movaps       xmmword ptr [rsp+40h], xmm0
00007FF6865714F9  movaps       xmmword ptr [rsp+50h], xmm1
00007FF6865714FE  movaps       xmmword ptr [rsp+60h], xmm3
00007FF686571503  movaps       xmmword ptr [rsp+70h], xmm2
00007FF686571508  mov          rax, qword ptr [00007FF68659A1C8h]
00007FF68657150F  test         rax, rax
00007FF686571512  jne          static void Fabled_Engine::main+269h (00007FF686571529h)
00007FF686571514  call         GetProcessHeap (00007FF68658F352h)
00007FF686571519  test         rax, rax
00007FF68657151C  je           static void Fabled_Engine::main+959h (00007FF686571C19h)
00007FF686571522  mov          qword ptr [00007FF68659A1C8h], rax
00007FF686571529  mov          r8d, 30h
00007FF68657152F  mov          rcx, rax
00007FF686571532  xor          edx, edx
00007FF686571534  call         HeapAlloc (00007FF68658F358h)
00007FF686571539  test         rax, rax
00007FF68657153C  je           static void Fabled_Engine::main+959h (00007FF686571C19h)
00007FF686571542  mov          rbx, rax
00007FF686571545  lea          rax, [rsi+1h]
00007FF686571549  lea          rcx, [rsi+2h]
00007FF68657154D  lea          r13, [rsi+3h]
00007FF686571551  mov          qword ptr [rbx], rsi
00007FF686571554  mov          qword ptr [rbx+8h], rax
00007FF686571558  mov          qword ptr [rbx+10h], rcx
00007FF68657155C  mov          qword ptr [rbx+18h], rcx
00007FF686571560  mov          qword ptr [rbx+20h], r13
00007FF686571564  mov          qword ptr [rbx+28h], rsi
00007FF686571568  movd         r8d, xmm14
00007FF68657156D  movaps       xmm0, xmm14
00007FF686571571  shufps       xmm0, xmm14, 55h
00007FF686571576  movd         ecx, xmm0
00007FF68657157A  movhlps      xmm14, xmm14
00007FF68657157E  shl          rcx, 20h
00007FF686571582  or           r8, rcx
00007FF686571585  movd         ecx, xmm6
00007FF686571589  movaps       xmm0, xmm6
00007FF68657158C  shufps       xmm0, xmm6, 55h
00007FF686571590  movd         edi, xmm0
00007FF686571594  punpckhqdq   xmm6, xmm6
00007FF686571598  movd         edx, xmm6
00007FF68657159C  shl          rdi, 20h
00007FF6865715A0  or           rcx, rdi
00007FF6865715A3  or           rdx, r12
00007FF6865715A6  movd         esi, xmm7
00007FF6865715AA  movaps       xmm0, xmm7
00007FF6865715AD  shufps       xmm0, xmm7, 55h
00007FF6865715B1  movd         ebp, xmm0
00007FF6865715B5  punpckhqdq   xmm7, xmm7
00007FF6865715B9  movd         edi, xmm7
00007FF6865715BD  shl          rbp, 20h
00007FF6865715C1  or           rsi, rbp
00007FF6865715C4  or           rdi, r12
00007FF6865715C7  movd         xmm1, dword ptr [rsp+40h]
00007FF6865715CD  movd         xmm0, dword ptr [rsp+44h]
00007FF6865715D3  movd         ebp, xmm0
00007FF6865715D7  shl          rbp, 20h
00007FF6865715DB  movd         eax, xmm1
00007FF6865715DF  or           rax, rbp
00007FF6865715E2  mov          ebp, dword ptr [rsp+48h]
00007FF6865715E6  movdqa       xmm2, xmm0
00007FF6865715EA  pand         xmm2, xmm9
00007FF6865715EF  por          xmm2, xmm12
00007FF6865715F4  cmpss        xmm0, xmm0, 3h
00007FF6865715F9  movaps       xmm3, xmm0
00007FF6865715FC  andps        xmm3, xmm13
00007FF686571600  andnps       xmm0, xmm2
00007FF686571603  orps         xmm0, xmm3
00007FF686571606  movdqa       xmm2, xmm1
00007FF68657160A  pand         xmm2, xmm9
00007FF68657160F  por          xmm2, xmm12
00007FF686571614  cmpss        xmm1, xmm1, 3h
00007FF686571619  movaps       xmm3, xmm1
00007FF68657161C  andps        xmm3, xmm13
00007FF686571620  andnps       xmm1, xmm2
00007FF686571623  orps         xmm1, xmm3
00007FF686571626  mov          dword ptr [rsp+88h], ebp
00007FF68657162D  mov          qword ptr [rsp+80h], rax
00007FF686571635  movss        dword ptr [rsp+8Ch], xmm1
00007FF68657163E  movss        dword ptr [rsp+90h], xmm0
00007FF686571647  movss        dword ptr [rsp+9Ch], xmm14
00007FF686571651  mov          qword ptr [rsp+94h], r8
00007FF686571659  mov          qword ptr [rsp+A8h], rdx
00007FF686571661  mov          qword ptr [rsp+A0h], rcx
00007FF686571669  mov          qword ptr [rsp+B8h], rdi
00007FF686571671  mov          qword ptr [rsp+B0h], rsi
00007FF686571679  movd         xmm1, dword ptr [rsp+50h]
00007FF68657167F  movd         xmm0, dword ptr [rsp+54h]
00007FF686571685  movd         eax, xmm0
00007FF686571689  shl          rax, 20h
00007FF68657168D  movd         ebp, xmm1
00007FF686571691  or           rbp, rax
00007FF686571694  mov          eax, dword ptr [rsp+58h]
00007FF686571698  movdqa       xmm2, xmm0
00007FF68657169C  pand         xmm2, xmm9
00007FF6865716A1  por          xmm2, xmm12
00007FF6865716A6  cmpss        xmm0, xmm0, 3h
00007FF6865716AB  movaps       xmm3, xmm0
00007FF6865716AE  andps        xmm3, xmm13
00007FF6865716B2  andnps       xmm0, xmm2
00007FF6865716B5  orps         xmm0, xmm3
00007FF6865716B8  movdqa       xmm2, xmm1
00007FF6865716BC  pand         xmm2, xmm9
00007FF6865716C1  por          xmm2, xmm12
00007FF6865716C6  cmpss        xmm1, xmm1, 3h
00007FF6865716CB  movaps       xmm3, xmm1
00007FF6865716CE  andps        xmm3, xmm13
00007FF6865716D2  andnps       xmm1, xmm2
00007FF6865716D5  orps         xmm1, xmm3
00007FF6865716D8  mov          dword ptr [rsp+C8h], eax
00007FF6865716DF  mov          qword ptr [rsp+C0h], rbp
00007FF6865716E7  movss        dword ptr [rsp+CCh], xmm1
00007FF6865716F0  movss        dword ptr [rsp+D0h], xmm0
00007FF6865716F9  movss        dword ptr [rsp+DCh], xmm14
00007FF686571703  mov          qword ptr [rsp+D4h], r8
00007FF68657170B  mov          qword ptr [rsp+E8h], rdx
00007FF686571713  mov          qword ptr [rsp+E0h], rcx
00007FF68657171B  mov          qword ptr [rsp+F8h], rdi
00007FF686571723  mov          qword ptr [rsp+F0h], rsi
00007FF68657172B  movd         xmm1, dword ptr [rsp+60h]
00007FF686571731  movd         xmm0, dword ptr [rsp+64h]
00007FF686571737  movd         eax, xmm0
00007FF68657173B  shl          rax, 20h
00007FF68657173F  movd         ebp, xmm1
00007FF686571743  or           rbp, rax
00007FF686571746  mov          eax, dword ptr [rsp+68h]
00007FF68657174A  movdqa       xmm2, xmm0
00007FF68657174E  pand         xmm2, xmm9
00007FF686571753  por          xmm2, xmm12
00007FF686571758  cmpss        xmm0, xmm0, 3h
00007FF68657175D  movaps       xmm3, xmm0
00007FF686571760  andps        xmm3, xmm13
00007FF686571764  andnps       xmm0, xmm2
00007FF686571767  orps         xmm0, xmm3
00007FF68657176A  movdqa       xmm2, xmm1
00007FF68657176E  pand         xmm2, xmm9
00007FF686571773  por          xmm2, xmm12
00007FF686571778  cmpss        xmm1, xmm1, 3h
00007FF68657177D  movaps       xmm3, xmm1
00007FF686571780  andps        xmm3, xmm13
00007FF686571784  andnps       xmm1, xmm2
00007FF686571787  orps         xmm1, xmm3
00007FF68657178A  mov          dword ptr [rsp+108h], eax
00007FF686571791  mov          qword ptr [rsp+100h], rbp
00007FF686571799  movss        dword ptr [rsp+10Ch], xmm1
00007FF6865717A2  movss        dword ptr [rsp+110h], xmm0
00007FF6865717AB  mov          qword ptr [rsp+114h], r8
00007FF6865717B3  movss        dword ptr [rsp+11Ch], xmm14
00007FF6865717BD  mov          qword ptr [rsp+120h], rcx
00007FF6865717C5  mov          qword ptr [rsp+128h], rdx
00007FF6865717CD  mov          qword ptr [rsp+130h], rsi
00007FF6865717D5  mov          qword ptr [rsp+138h], rdi
00007FF6865717DD  movd         xmm1, dword ptr [rsp+70h]
00007FF6865717E3  movd         xmm0, dword ptr [rsp+74h]
00007FF6865717E9  movd         eax, xmm0
00007FF6865717ED  shl          rax, 20h
00007FF6865717F1  movd         ebp, xmm1
00007FF6865717F5  or           rbp, rax
00007FF6865717F8  mov          eax, dword ptr [rsp+78h]
00007FF6865717FC  movdqa       xmm2, xmm0
00007FF686571800  pand         xmm2, xmm9
00007FF686571805  por          xmm2, xmm12
00007FF68657180A  cmpss        xmm0, xmm0, 3h
00007FF68657180F  movaps       xmm3, xmm0
00007FF686571812  andps        xmm3, xmm13
00007FF686571816  andnps       xmm0, xmm2
00007FF686571819  orps         xmm0, xmm3
00007FF68657181C  movdqa       xmm2, xmm1
00007FF686571820  pand         xmm2, xmm9
00007FF686571825  por          xmm2, xmm12
00007FF68657182A  cmpss        xmm1, xmm1, 3h
00007FF68657182F  movaps       xmm3, xmm1
00007FF686571832  andps        xmm3, xmm13
00007FF686571836  andnps       xmm1, xmm2
00007FF686571839  orps         xmm1, xmm3
00007FF68657183C  mov          dword ptr [rsp+148h], eax
00007FF686571843  mov          qword ptr [rsp+140h], rbp
00007FF68657184B  movss        dword ptr [rsp+14Ch], xmm1
00007FF686571854  movss        dword ptr [rsp+150h], xmm0
00007FF68657185D  mov          qword ptr [rsp+154h], r8
00007FF686571865  movss        dword ptr [rsp+15Ch], xmm14
00007FF68657186F  mov          qword ptr [rsp+160h], rcx
00007FF686571877  mov          qword ptr [rsp+168h], rdx
00007FF68657187F  mov          qword ptr [rsp+170h], rsi
00007FF686571887  mov          qword ptr [rsp+178h], rdi
00007FF68657188F  mov          rax, qword ptr [00007FF68659A1C8h]
00007FF686571896  test         rax, rax
00007FF686571899  jne          static void Fabled_Engine::main+5F0h (00007FF6865718B0h)
00007FF68657189B  call         GetProcessHeap (00007FF68658F352h)
00007FF6865718A0  test         rax, rax
00007FF6865718A3  je           static void Fabled_Engine::main+96Ah (00007FF686571C2Ah)
00007FF6865718A9  mov          qword ptr [00007FF68659A1C8h], rax
00007FF6865718B0  mov          r8d, 100h
00007FF6865718B6  mov          rcx, rax
00007FF6865718B9  xor          edx, edx
00007FF6865718BB  call         HeapAlloc (00007FF68658F358h)
00007FF6865718C0  test         rax, rax
00007FF6865718C3  je           static void Fabled_Engine::main+96Ah (00007FF686571C2Ah)
00007FF6865718C9  mov          rsi, rax
00007FF6865718CC  mov          r8d, 100h
00007FF6865718D2  mov          rcx, rax
00007FF6865718D5  mov          rdx, r15
00007FF6865718D8  call         memcpy (00007FF686590437h)
00007FF6865718DD  mov          rdx, qword ptr [rsp+38h]
00007FF6865718E2  cmp          rdx, qword ptr [rsp+30h]
00007FF6865718E7  jne          static void Fabled_Engine::main+1B0h (00007FF686571470h)
00007FF6865718ED  lea          rcx, [rsp+28h]
00007FF6865718F2  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::mesh_data::Mesh,alloc::alloc::Global> (00007FF686590EE0h)
00007FF6865718F7  mov          rdx, qword ptr [rsp+38h]
00007FF6865718FC  jmp          static void Fabled_Engine::main+1B0h (00007FF686571470h)
00007FF686571901  mov          rax, qword ptr [rsp+38h]
00007FF686571906  mov          qword ptr [rsp+190h], rax
00007FF68657190E  movups       xmm0, xmmword ptr [rsp+28h]
00007FF686571913  movaps       xmmword ptr [rsp+180h], xmm0
00007FF68657191B  lea          rax, [rsp+180h]