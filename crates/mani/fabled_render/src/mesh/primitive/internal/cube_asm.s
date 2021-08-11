# --------------- Cube Dissassembly -------------------
00007FF6D5791092  mov          rcx, qword ptr [00007FF6D57B31C8h]
00007FF6D5791099  test         rcx, rcx
00007FF6D579109C  jne          static void Fabled_Engine::main+B6h (00007FF6D57910B6h)
00007FF6D579109E  call         GetProcessHeap (00007FF6D57A915Ch)
00007FF6D57910A3  test         rax, rax
00007FF6D57910A6  je           static void Fabled_Engine::main+9A0h (00007FF6D57919A0h)
00007FF6D57910AC  mov          rcx, rax
00007FF6D57910AF  mov          qword ptr [00007FF6D57B31C8h], rax
00007FF6D57910B6  mov          r8d, 600h
00007FF6D57910BC  xor          edx, edx
00007FF6D57910BE  call         HeapAlloc (00007FF6D57A9162h)
00007FF6D57910C3  test         rax, rax
00007FF6D57910C6  je           static void Fabled_Engine::main+9A0h (00007FF6D57919A0h)
00007FF6D57910CC  mov          qword ptr [rsp+28h], rax
00007FF6D57910D1  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000018 (00007FF6D57AC510h)]
00007FF6D57910D8  movups       xmmword ptr [rsp+30h], xmm0
00007FF6D57910DD  xorps        xmm0, xmm0
00007FF6D57910E0  movaps       xmmword ptr [rsp+170h], xmm0
00007FF6D57910E8  movaps       xmmword ptr [rsp+160h], xmm0
00007FF6D57910F0  movaps       xmmword ptr [rsp+150h], xmm0
00007FF6D57910F8  movaps       xmmword ptr [rsp+140h], xmm0
00007FF6D5791100  movaps       xmmword ptr [rsp+130h], xmm0
00007FF6D5791108  movaps       xmmword ptr [rsp+120h], xmm0
00007FF6D5791110  movaps       xmmword ptr [rsp+110h], xmm0
00007FF6D5791118  movaps       xmmword ptr [rsp+100h], xmm0
00007FF6D5791120  movaps       xmmword ptr [rsp+F0h], xmm0
00007FF6D5791128  movaps       xmmword ptr [rsp+E0h], xmm0
00007FF6D5791130  movaps       xmmword ptr [rsp+D0h], xmm0
00007FF6D5791138  movaps       xmmword ptr [rsp+C0h], xmm0
00007FF6D5791140  movaps       xmmword ptr [rsp+B0h], xmm0
00007FF6D5791148  movaps       xmmword ptr [rsp+A0h], xmm0
00007FF6D5791150  movaps       xmmword ptr [rsp+90h], xmm0
00007FF6D5791158  movaps       xmmword ptr [rsp+80h], xmm0
00007FF6D5791160  lea          r12, [00007FF6D57ADA80h]
00007FF6D5791167  xor          edi, edi
00007FF6D5791169  mov          r13, 3F80000000000000h
00007FF6D5791173  movdqa       xmm8, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF6D57AC520h)]
00007FF6D579117C  movd         xmm9, dword ptr [__real@3f800000 (00007FF6D57AC530h)]
00007FF6D5791185  movss        xmm13, dword ptr [__real@3f000000 (00007FF6D57AC534h)]
00007FF6D579118E  movss        xmm12, dword ptr [__real@7fc00000 (00007FF6D57AC538h)]
00007FF6D5791197  movaps       xmm10, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF6D57AC540h)]
00007FF6D579119F  movaps       xmm11, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF6D57AC550h)]
00007FF6D57911A7  jmp          static void Fabled_Engine::main+1F4h (00007FF6D57911F4h)
00007FF6D57911A9  nop          dword ptr [rax], eax
00007FF6D57911B0  mov          rsi, qword ptr [rsp+28h]
00007FF6D57911B5  mov          r14, rdx
00007FF6D57911B8  shl          r14, 6h
00007FF6D57911BC  mov          qword ptr [rsi+r14*1], rdi
00007FF6D57911C0  movups       xmmword ptr [rsi+r14*1+8h], xmm10
00007FF6D57911C6  mov          dword ptr [rsi+r14*1+18h], 0h
00007FF6D57911CF  mov          qword ptr [rsi+r14*1+20h], rbx
00007FF6D57911D4  movups       xmmword ptr [rsi+r14*1+28h], xmm11
00007FF6D57911DA  add          rdx, 1h
00007FF6D57911DE  mov          qword ptr [rsp+38h], rdx
00007FF6D57911E3  add          r12, 10h
00007FF6D57911E7  mov          rdi, r15
00007FF6D57911EA  cmp          r15, 12h
00007FF6D57911EE  je           static void Fabled_Engine::main+68Ah (00007FF6D579168Ah)
00007FF6D57911F4  movaps       xmm14, xmmword ptr [r12]
00007FF6D57911F9  movaps       xmm7, xmmword ptr [r12+60h]
00007FF6D57911FF  movaps       xmm6, xmmword ptr [r12+C0h]
00007FF6D5791208  movaps       xmm0, xmm14
00007FF6D579120C  subps        xmm0, xmm6
00007FF6D579120F  movaps       xmm1, xmm7
00007FF6D5791212  addps        xmm1, xmm0
00007FF6D5791215  subps        xmm0, xmm7
00007FF6D5791218  movaps       xmm2, xmm14
00007FF6D579121C  addps        xmm2, xmm6
00007FF6D579121F  movaps       xmm3, xmm7
00007FF6D5791222  addps        xmm3, xmm2
00007FF6D5791225  subps        xmm2, xmm7
00007FF6D5791228  movaps       xmmword ptr [rsp+40h], xmm0
00007FF6D579122D  movaps       xmmword ptr [rsp+50h], xmm1
00007FF6D5791232  movaps       xmmword ptr [rsp+60h], xmm3
00007FF6D5791237  movaps       xmmword ptr [rsp+70h], xmm2
00007FF6D579123C  mov          rax, qword ptr [00007FF6D57B31C8h]
00007FF6D5791243  test         rax, rax
00007FF6D5791246  jne          static void Fabled_Engine::main+25Dh (00007FF6D579125Dh)
00007FF6D5791248  call         GetProcessHeap (00007FF6D57A915Ch)
00007FF6D579124D  test         rax, rax
00007FF6D5791250  je           static void Fabled_Engine::main+97Eh (00007FF6D579197Eh)
00007FF6D5791256  mov          qword ptr [00007FF6D57B31C8h], rax
00007FF6D579125D  mov          r8d, 30h
00007FF6D5791263  mov          rcx, rax
00007FF6D5791266  xor          edx, edx
00007FF6D5791268  call         HeapAlloc (00007FF6D57A9162h)
00007FF6D579126D  test         rax, rax
00007FF6D5791270  je           static void Fabled_Engine::main+97Eh (00007FF6D579197Eh)
00007FF6D5791276  mov          rbx, rax
00007FF6D5791279  lea          rax, [rdi+1h]
00007FF6D579127D  lea          rcx, [rdi+2h]
00007FF6D5791281  lea          r15, [rdi+3h]
00007FF6D5791285  mov          qword ptr [rbx], rdi
00007FF6D5791288  mov          qword ptr [rbx+8h], rax
00007FF6D579128C  mov          qword ptr [rbx+10h], rcx
00007FF6D5791290  mov          qword ptr [rbx+18h], rcx
00007FF6D5791294  mov          qword ptr [rbx+20h], r15
00007FF6D5791298  mov          qword ptr [rbx+28h], rdi
00007FF6D579129C  movd         r8d, xmm14
00007FF6D57912A1  movaps       xmm0, xmm14
00007FF6D57912A5  shufps       xmm0, xmm14, 55h
00007FF6D57912AA  movd         eax, xmm0
00007FF6D57912AE  movhlps      xmm14, xmm14
00007FF6D57912B2  shl          rax, 20h
00007FF6D57912B6  or           r8, rax
00007FF6D57912B9  movd         ecx, xmm7
00007FF6D57912BD  movaps       xmm0, xmm7
00007FF6D57912C0  shufps       xmm0, xmm7, 55h
00007FF6D57912C4  movd         eax, xmm0
00007FF6D57912C8  punpckhqdq   xmm7, xmm7
00007FF6D57912CC  movd         edx, xmm7
00007FF6D57912D0  shl          rax, 20h
00007FF6D57912D4  or           rcx, rax
00007FF6D57912D7  or           rdx, r13
00007FF6D57912DA  movd         edi, xmm6
00007FF6D57912DE  movaps       xmm0, xmm6
00007FF6D57912E1  shufps       xmm0, xmm6, 55h
00007FF6D57912E5  movd         eax, xmm0
00007FF6D57912E9  punpckhqdq   xmm6, xmm6
00007FF6D57912ED  movd         esi, xmm6
00007FF6D57912F1  shl          rax, 20h
00007FF6D57912F5  or           rdi, rax
00007FF6D57912F8  or           rsi, r13
00007FF6D57912FB  movd         xmm1, dword ptr [rsp+40h]
00007FF6D5791301  movd         xmm0, dword ptr [rsp+44h]
00007FF6D5791307  movd         eax, xmm0
00007FF6D579130B  shl          rax, 20h
00007FF6D579130F  movd         ebp, xmm1
00007FF6D5791313  or           rbp, rax
00007FF6D5791316  mov          eax, dword ptr [rsp+48h]
00007FF6D579131A  movdqa       xmm2, xmm0
00007FF6D579131E  pand         xmm2, xmm8
00007FF6D5791323  por          xmm2, xmm9
00007FF6D5791328  movdqa       xmm3, xmm1
00007FF6D579132C  pand         xmm3, xmm8
00007FF6D5791331  por          xmm3, xmm9
00007FF6D5791336  mulss        xmm2, xmm13
00007FF6D579133B  addss        xmm2, xmm13
00007FF6D5791340  cmpss        xmm0, xmm0, 3h
00007FF6D5791345  movaps       xmm4, xmm0
00007FF6D5791348  andps        xmm4, xmm12
00007FF6D579134C  andnps       xmm0, xmm2
00007FF6D579134F  orps         xmm0, xmm4
00007FF6D5791352  mulss        xmm3, xmm13
00007FF6D5791357  addss        xmm3, xmm13
00007FF6D579135C  cmpss        xmm1, xmm1, 3h
00007FF6D5791361  movaps       xmm2, xmm1
00007FF6D5791364  andps        xmm2, xmm12
00007FF6D5791368  andnps       xmm1, xmm3
00007FF6D579136B  orps         xmm1, xmm2
00007FF6D579136E  mov          dword ptr [rsp+88h], eax
00007FF6D5791375  mov          qword ptr [rsp+80h], rbp
00007FF6D579137D  movss        dword ptr [rsp+8Ch], xmm1
00007FF6D5791386  movss        dword ptr [rsp+90h], xmm0
00007FF6D579138F  movss        dword ptr [rsp+9Ch], xmm14
00007FF6D5791399  mov          qword ptr [rsp+94h], r8
00007FF6D57913A1  mov          qword ptr [rsp+A8h], rdx
00007FF6D57913A9  mov          qword ptr [rsp+A0h], rcx
00007FF6D57913B1  mov          qword ptr [rsp+B8h], rsi
00007FF6D57913B9  mov          qword ptr [rsp+B0h], rdi
00007FF6D57913C1  movd         xmm1, dword ptr [rsp+50h]
00007FF6D57913C7  movd         xmm0, dword ptr [rsp+54h]
00007FF6D57913CD  movd         eax, xmm0
00007FF6D57913D1  shl          rax, 20h
00007FF6D57913D5  movd         ebp, xmm1
00007FF6D57913D9  or           rbp, rax
00007FF6D57913DC  mov          eax, dword ptr [rsp+58h]
00007FF6D57913E0  movdqa       xmm2, xmm0
00007FF6D57913E4  pand         xmm2, xmm8
00007FF6D57913E9  por          xmm2, xmm9
00007FF6D57913EE  movdqa       xmm3, xmm1
00007FF6D57913F2  pand         xmm3, xmm8
00007FF6D57913F7  por          xmm3, xmm9
00007FF6D57913FC  mulss        xmm2, xmm13
00007FF6D5791401  addss        xmm2, xmm13
00007FF6D5791406  cmpss        xmm0, xmm0, 3h
00007FF6D579140B  movaps       xmm4, xmm0
00007FF6D579140E  andps        xmm4, xmm12
00007FF6D5791412  andnps       xmm0, xmm2
00007FF6D5791415  orps         xmm0, xmm4
00007FF6D5791418  mulss        xmm3, xmm13
00007FF6D579141D  addss        xmm3, xmm13
00007FF6D5791422  cmpss        xmm1, xmm1, 3h
00007FF6D5791427  movaps       xmm2, xmm1
00007FF6D579142A  andps        xmm2, xmm12
00007FF6D579142E  andnps       xmm1, xmm3
00007FF6D5791431  orps         xmm1, xmm2
00007FF6D5791434  mov          dword ptr [rsp+C8h], eax
00007FF6D579143B  mov          qword ptr [rsp+C0h], rbp
00007FF6D5791443  movss        dword ptr [rsp+CCh], xmm1
00007FF6D579144C  movss        dword ptr [rsp+D0h], xmm0
00007FF6D5791455  movss        dword ptr [rsp+DCh], xmm14
00007FF6D579145F  mov          qword ptr [rsp+D4h], r8
00007FF6D5791467  mov          qword ptr [rsp+E8h], rdx
00007FF6D579146F  mov          qword ptr [rsp+E0h], rcx
00007FF6D5791477  mov          qword ptr [rsp+F8h], rsi
00007FF6D579147F  mov          qword ptr [rsp+F0h], rdi
00007FF6D5791487  movd         xmm1, dword ptr [rsp+60h]
00007FF6D579148D  movd         xmm0, dword ptr [rsp+64h]
00007FF6D5791493  movd         eax, xmm0
00007FF6D5791497  shl          rax, 20h
00007FF6D579149B  movd         ebp, xmm1
00007FF6D579149F  or           rbp, rax
00007FF6D57914A2  mov          eax, dword ptr [rsp+68h]
00007FF6D57914A6  movdqa       xmm2, xmm0
00007FF6D57914AA  pand         xmm2, xmm8
00007FF6D57914AF  por          xmm2, xmm9
00007FF6D57914B4  movdqa       xmm3, xmm1
00007FF6D57914B8  pand         xmm3, xmm8
00007FF6D57914BD  por          xmm3, xmm9
00007FF6D57914C2  mulss        xmm2, xmm13
00007FF6D57914C7  addss        xmm2, xmm13
00007FF6D57914CC  cmpss        xmm0, xmm0, 3h
00007FF6D57914D1  movaps       xmm4, xmm0
00007FF6D57914D4  andps        xmm4, xmm12
00007FF6D57914D8  andnps       xmm0, xmm2
00007FF6D57914DB  orps         xmm0, xmm4
00007FF6D57914DE  mulss        xmm3, xmm13
00007FF6D57914E3  addss        xmm3, xmm13
00007FF6D57914E8  cmpss        xmm1, xmm1, 3h
00007FF6D57914ED  movaps       xmm2, xmm1
00007FF6D57914F0  andps        xmm2, xmm12
00007FF6D57914F4  andnps       xmm1, xmm3
00007FF6D57914F7  orps         xmm1, xmm2
00007FF6D57914FA  mov          dword ptr [rsp+108h], eax
00007FF6D5791501  mov          qword ptr [rsp+100h], rbp
00007FF6D5791509  movss        dword ptr [rsp+10Ch], xmm1
00007FF6D5791512  movss        dword ptr [rsp+110h], xmm0
00007FF6D579151B  mov          qword ptr [rsp+114h], r8
00007FF6D5791523  movss        dword ptr [rsp+11Ch], xmm14
00007FF6D579152D  mov          qword ptr [rsp+120h], rcx
00007FF6D5791535  mov          qword ptr [rsp+128h], rdx
00007FF6D579153D  mov          qword ptr [rsp+130h], rdi
00007FF6D5791545  mov          qword ptr [rsp+138h], rsi
00007FF6D579154D  movd         xmm1, dword ptr [rsp+70h]
00007FF6D5791553  movd         xmm0, dword ptr [rsp+74h]
00007FF6D5791559  movd         eax, xmm0
00007FF6D579155D  shl          rax, 20h
00007FF6D5791561  movd         ebp, xmm1
00007FF6D5791565  or           rbp, rax
00007FF6D5791568  mov          eax, dword ptr [rsp+78h]
00007FF6D579156C  movdqa       xmm2, xmm0
00007FF6D5791570  pand         xmm2, xmm8
00007FF6D5791575  por          xmm2, xmm9
00007FF6D579157A  movdqa       xmm3, xmm1
00007FF6D579157E  pand         xmm3, xmm8
00007FF6D5791583  por          xmm3, xmm9
00007FF6D5791588  mulss        xmm2, xmm13
00007FF6D579158D  addss        xmm2, xmm13
00007FF6D5791592  cmpss        xmm0, xmm0, 3h
00007FF6D5791597  movaps       xmm4, xmm0
00007FF6D579159A  andps        xmm4, xmm12
00007FF6D579159E  andnps       xmm0, xmm2
00007FF6D57915A1  orps         xmm0, xmm4
00007FF6D57915A4  mulss        xmm3, xmm13
00007FF6D57915A9  addss        xmm3, xmm13
00007FF6D57915AE  cmpss        xmm1, xmm1, 3h
00007FF6D57915B3  movaps       xmm2, xmm1
00007FF6D57915B6  andps        xmm2, xmm12
00007FF6D57915BA  andnps       xmm1, xmm3
00007FF6D57915BD  orps         xmm1, xmm2
00007FF6D57915C0  mov          dword ptr [rsp+148h], eax
00007FF6D57915C7  mov          qword ptr [rsp+140h], rbp
00007FF6D57915CF  movss        dword ptr [rsp+14Ch], xmm1
00007FF6D57915D8  movss        dword ptr [rsp+150h], xmm0
00007FF6D57915E1  mov          qword ptr [rsp+154h], r8
00007FF6D57915E9  movss        dword ptr [rsp+15Ch], xmm14
00007FF6D57915F3  mov          qword ptr [rsp+160h], rcx
00007FF6D57915FB  mov          qword ptr [rsp+168h], rdx
00007FF6D5791603  mov          qword ptr [rsp+170h], rdi
00007FF6D579160B  mov          qword ptr [rsp+178h], rsi
00007FF6D5791613  mov          rax, qword ptr [00007FF6D57B31C8h]
00007FF6D579161A  test         rax, rax
00007FF6D579161D  jne          static void Fabled_Engine::main+634h (00007FF6D5791634h)
00007FF6D579161F  call         GetProcessHeap (00007FF6D57A915Ch)
00007FF6D5791624  test         rax, rax
00007FF6D5791627  je           static void Fabled_Engine::main+98Fh (00007FF6D579198Fh)
00007FF6D579162D  mov          qword ptr [00007FF6D57B31C8h], rax
00007FF6D5791634  mov          r8d, 100h
00007FF6D579163A  mov          rcx, rax
00007FF6D579163D  xor          edx, edx
00007FF6D579163F  call         HeapAlloc (00007FF6D57A9162h)
00007FF6D5791644  test         rax, rax
00007FF6D5791647  je           static void Fabled_Engine::main+98Fh (00007FF6D579198Fh)
00007FF6D579164D  mov          rdi, rax
00007FF6D5791650  mov          r8d, 100h
00007FF6D5791656  mov          rcx, rax
00007FF6D5791659  lea          rdx, [rsp+80h]
00007FF6D5791661  call         memcpy (00007FF6D57AA247h)
00007FF6D5791666  mov          rdx, qword ptr [rsp+38h]
00007FF6D579166B  cmp          rdx, qword ptr [rsp+30h]
00007FF6D5791670  jne          static void Fabled_Engine::main+1B0h (00007FF6D57911B0h)
00007FF6D5791676  lea          rcx, [rsp+28h]
00007FF6D579167B  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::mesh_data::Mesh,alloc::alloc::Global> (00007FF6D57AAC90h)
00007FF6D5791680  mov          rdx, qword ptr [rsp+38h]
00007FF6D5791685  jmp          static void Fabled_Engine::main+1B0h (00007FF6D57911B0h)
