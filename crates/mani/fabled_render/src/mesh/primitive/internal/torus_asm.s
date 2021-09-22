# --------------- Torus Dissassembly -------------------
00007FF670B1135A  mov          rcx, qword ptr [00007FF670B331C8h]
00007FF670B11361  test         rcx, rcx
00007FF670B11364  jne          static void Fabled_Engine::main+BEh (00007FF670B1137Eh)
00007FF670B11366  call         GetProcessHeap (00007FF670B28E72h)
00007FF670B1136B  test         rax, rax
00007FF670B1136E  je           static void Fabled_Engine::main+6B2h (00007FF670B11972h)
00007FF670B11374  mov          rcx, rax
00007FF670B11377  mov          qword ptr [00007FF670B331C8h], rax
00007FF670B1137E  mov          r8d, CE40h
00007FF670B11384  xor          edx, edx
00007FF670B11386  call         HeapAlloc (00007FF670B28E78h)
00007FF670B1138B  test         rax, rax
00007FF670B1138E  je           static void Fabled_Engine::main+6B2h (00007FF670B11972h)
00007FF670B11394  mov          qword ptr [rsp+68h], rax
00007FF670B11399  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000339 (00007FF670B2C530h)]
00007FF670B113A0  movups       xmmword ptr [rsp+70h], xmm0
00007FF670B113A5  mov          rcx, qword ptr [00007FF670B331C8h]
00007FF670B113AC  test         rcx, rcx
00007FF670B113AF  jne          static void Fabled_Engine::main+109h (00007FF670B113C9h)
00007FF670B113B1  call         GetProcessHeap (00007FF670B28E72h)
00007FF670B113B6  test         rax, rax
00007FF670B113B9  je           static void Fabled_Engine::main+6B9h (00007FF670B11979h)
00007FF670B113BF  mov          rcx, rax
00007FF670B113C2  mov          qword ptr [00007FF670B331C8h], rax
00007FF670B113C9  mov          r8d, 9000h
00007FF670B113CF  mov          edx, 8h
00007FF670B113D4  call         HeapAlloc (00007FF670B28E78h)
00007FF670B113D9  mov          qword ptr [rsp+98h], rax
00007FF670B113E1  test         rax, rax
00007FF670B113E4  je           static void Fabled_Engine::main+6B9h (00007FF670B11979h)
00007FF670B113EA  xor          r14d, r14d
00007FF670B113ED  movss        xmm11, dword ptr [__real@40c90fdb (00007FF670B2C544h)]
00007FF670B113F6  movss        xmm13, dword ptr [__real@3f000000 (00007FF670B2C54Ch)]
00007FF670B113FF  xorps        xmm15, xmm15
00007FF670B11403  lea          r15, [rsp+68h]
00007FF670B11408  xor          r12d, r12d
00007FF670B1140B  nop          dword ptr [rax+rax*1], eax
00007FF670B11410  test         r12, r12
00007FF670B11413  js           static void Fabled_Engine::main+160h (00007FF670B11420h)
00007FF670B11415  cvtsi2ss     xmm10, r12
00007FF670B1141A  jmp          static void Fabled_Engine::main+179h (00007FF670B11439h)
00007FF670B1141C  nop          dword ptr [rax], eax
00007FF670B11420  mov          rax, r12
00007FF670B11423  shr          rax, 1h
00007FF670B11426  mov          ecx, r12d
00007FF670B11429  and          ecx, 1h
00007FF670B1142C  or           rcx, rax
00007FF670B1142F  cvtsi2ss     xmm10, rcx
00007FF670B11434  addss        xmm10, xmm10
00007FF670B11439  mulss        xmm10, dword ptr [__real@3d000000 (00007FF670B2C540h)]
00007FF670B11442  movaps       xmm6, xmm10
00007FF670B11446  mulss        xmm6, xmm11
00007FF670B1144B  movaps       xmm0, xmm6
00007FF670B1144E  call         sinf (00007FF670B29F99h)
00007FF670B11453  movaps       xmm8, xmm0
00007FF670B11457  movaps       xmm0, xmm6
00007FF670B1145A  call         cosf (00007FF670B29F9Fh)
00007FF670B1145F  movaps       xmm14, xmm0
00007FF670B11463  xor          r13d, r13d
00007FF670B11466  nop          word ptr cs:[rax+rax*1], ax
00007FF670B11470  test         r13, r13
00007FF670B11473  js           static void Fabled_Engine::main+1C0h (00007FF670B11480h)
00007FF670B11475  cvtsi2ss     xmm12, r13
00007FF670B1147A  jmp          static void Fabled_Engine::main+1D9h (00007FF670B11499h)
00007FF670B1147C  nop          dword ptr [rax], eax
00007FF670B11480  mov          rax, r13
00007FF670B11483  shr          rax, 1h
00007FF670B11486  mov          ecx, r13d
00007FF670B11489  and          ecx, 1h
00007FF670B1148C  or           rcx, rax
00007FF670B1148F  cvtsi2ss     xmm12, rcx
00007FF670B11494  addss        xmm12, xmm12
00007FF670B11499  mulss        xmm12, dword ptr [__real@3d2aaaab (00007FF670B2C548h)]
00007FF670B114A2  movaps       xmm7, xmm12
00007FF670B114A6  mulss        xmm7, xmm11
00007FF670B114AB  movaps       xmm0, xmm7
00007FF670B114AE  call         sinf (00007FF670B29F99h)
00007FF670B114B3  movaps       xmm6, xmm0
00007FF670B114B6  movaps       xmm0, xmm7
00007FF670B114B9  call         cosf (00007FF670B29F9Fh)
00007FF670B114BE  movaps       xmm7, xmm0
00007FF670B114C1  mulss        xmm7, xmm13
00007FF670B114C6  addss        xmm7, dword ptr [__real@3f800000 (00007FF670B2C550h)]
00007FF670B114CE  movaps       xmm1, xmm14
00007FF670B114D2  mulss        xmm1, xmm7
00007FF670B114D6  movaps       xmm9, xmm14
00007FF670B114DA  mulss        xmm9, xmm0
00007FF670B114DF  mulss        xmm0, xmm8
00007FF670B114E4  unpcklps     xmm9, xmm6
00007FF670B114E8  mulss        xmm6, xmm13
00007FF670B114ED  movlhps      xmm9, xmm0
00007FF670B114F1  movaps       xmm0, xmm9
00007FF670B114F5  mulps        xmm0, xmm9
00007FF670B114F9  movaps       xmm2, xmm0
00007FF670B114FC  shufps       xmm2, xmm0, 55h
00007FF670B11500  addss        xmm2, xmm0
00007FF670B11504  movhlps      xmm0, xmm0
00007FF670B11507  addss        xmm0, xmm2
00007FF670B1150B  shufps       xmm0, xmm0, 0h
00007FF670B1150F  sqrtps       xmm0, xmm0
00007FF670B11512  divps        xmm9, xmm0
00007FF670B11516  movd         ebp, xmm1
00007FF670B1151A  movd         eax, xmm6
00007FF670B1151E  shl          rax, 20h
00007FF670B11522  or           rbp, rax
00007FF670B11525  movd         esi, xmm9
00007FF670B1152A  movaps       xmm0, xmm9
00007FF670B1152E  shufps       xmm0, xmm9, 55h
00007FF670B11533  movd         eax, xmm0
00007FF670B11537  shl          rax, 20h
00007FF670B1153B  or           rsi, rax
00007FF670B1153E  cmp          r13, 18h
00007FF670B11542  mulss        xmm7, xmm8
00007FF670B11547  movaps       xmmword ptr [rsp+40h], xmm15
00007FF670B1154D  movaps       xmmword ptr [rsp+30h], xmm15
00007FF670B11553  mov          rdi, r13
00007FF670B11556  adc          rdi, 0h
00007FF670B1155A  cmp          r14, qword ptr [rsp+70h]
00007FF670B1155F  je           static void Fabled_Engine::main+311h (00007FF670B115D1h)
00007FF670B11561  movd         eax, xmm7
00007FF670B11565  punpckhqdq   xmm9, xmm9
00007FF670B1156A  movd         ecx, xmm9
00007FF670B1156F  mov          rdx, qword ptr [rsp+68h]
00007FF670B11574  mov          r14, qword ptr [rsp+78h]
00007FF670B11579  mov          rbx, r14
00007FF670B1157C  shl          rbx, 6h
00007FF670B11580  add          r14, 1h
00007FF670B11584  mov          qword ptr [rdx+rbx*1], rbp
00007FF670B11588  mov          dword ptr [rdx+rbx*1+8h], eax
00007FF670B1158C  movss        dword ptr [rdx+rbx*1+Ch], xmm10
00007FF670B11593  movss        dword ptr [rdx+rbx*1+10h], xmm12
00007FF670B1159A  mov          qword ptr [rdx+rbx*1+14h], rsi
00007FF670B1159F  mov          dword ptr [rdx+rbx*1+1Ch], ecx
00007FF670B115A3  movaps       xmm0, xmmword ptr [rsp+40h]
00007FF670B115A8  movaps       xmmword ptr [rdx+rbx*1+20h], xmm0
00007FF670B115AD  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF670B115B2  movaps       xmmword ptr [rdx+rbx*1+30h], xmm0
00007FF670B115B7  mov          qword ptr [rsp+78h], r14
00007FF670B115BC  cmp          r13, 17h
00007FF670B115C0  ja           static void Fabled_Engine::main+320h (00007FF670B115E0h)
00007FF670B115C2  mov          r13, rdi
00007FF670B115C5  cmp          rdi, 19h
00007FF670B115C9  jb           static void Fabled_Engine::main+1B0h (00007FF670B11470h)
00007FF670B115CF  jmp          static void Fabled_Engine::main+320h (00007FF670B115E0h)
00007FF670B115D1  mov          rcx, r15
00007FF670B115D4  mov          rdx, r14
00007FF670B115D7  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF670B2A9B0h)
00007FF670B115DC  jmp          static void Fabled_Engine::main+2A1h (00007FF670B11561h)
00007FF670B115DE  nop
00007FF670B115E0  cmp          r12, 20h
00007FF670B115E4  mov          rax, r12
00007FF670B115E7  adc          rax, 0h
00007FF670B115EB  cmp          r12, 1Fh
00007FF670B115EF  ja           static void Fabled_Engine::main+33Eh (00007FF670B115FEh)
00007FF670B115F1  mov          r12, rax
00007FF670B115F4  cmp          rax, 21h
00007FF670B115F8  jb           static void Fabled_Engine::main+150h (00007FF670B11410h)
00007FF670B115FE  mov          r14, qword ptr [rsp+98h]
00007FF670B11606  mov          r9, r14
00007FF670B11609  add          r9, 28h
00007FF670B1160D  xor          r10d, r10d
00007FF670B11610  xor          r8d, r8d
00007FF670B11613  nop          word ptr cs:[rax+rax*1], ax
00007FF670B1161D  nop          dword ptr [rax], eax
00007FF670B11620  add          r8, 1h
00007FF670B11624  mov          rax, r9
00007FF670B11627  xor          ebx, ebx
00007FF670B11629  nop          dword ptr [rax], eax
00007FF670B11630  lea          rdi, [r10+rbx*1]
00007FF670B11634  lea          rsi, [r10+rbx*1]
00007FF670B11638  add          rsi, 1h
00007FF670B1163C  lea          rdx, [r10+rbx*1+19h]
00007FF670B11641  lea          rbp, [r10+rbx*1+1Ah]
00007FF670B11646  lea          rcx, [rbx+1h]
00007FF670B1164A  mov          qword ptr [rax-28h], rdi
00007FF670B1164E  mov          qword ptr [rax-20h], rsi
00007FF670B11652  mov          qword ptr [rax-18h], rdx
00007FF670B11656  mov          qword ptr [rax-10h], rsi
00007FF670B1165A  mov          qword ptr [rax-8h], rbp
00007FF670B1165E  mov          qword ptr [rax], rdx
00007FF670B11661  add          rax, 30h
00007FF670B11665  mov          rbx, rcx
00007FF670B11668  cmp          rcx, 18h
00007FF670B1166C  jne          static void Fabled_Engine::main+370h (00007FF670B11630h)
00007FF670B1166E  add          r10, 19h
00007FF670B11672  add          r9, 480h
00007FF670B11679  cmp          r8, 20h
00007FF670B1167D  jne          static void Fabled_Engine::main+360h (00007FF670B11620h)
00007FF670B1167F  mov          rcx, qword ptr [00007FF670B331C8h]
00007FF670B11686  test         rcx, rcx
00007FF670B11689  jne          static void Fabled_Engine::main+3E3h (00007FF670B116A3h)
00007FF670B1168B  call         GetProcessHeap (00007FF670B28E72h)
00007FF670B11690  test         rax, rax
00007FF670B11693  je           static void Fabled_Engine::main+6CAh (00007FF670B1198Ah)
00007FF670B11699  mov          rcx, rax
00007FF670B1169C  mov          qword ptr [00007FF670B331C8h], rax
00007FF670B116A3  mov          r8d, 40h
00007FF670B116A9  xor          edx, edx
00007FF670B116AB  call         HeapAlloc (00007FF670B28E78h)
00007FF670B116B0  test         rax, rax
00007FF670B116B3  je           static void Fabled_Engine::main+6CAh (00007FF670B1198Ah)
00007FF670B116B9  mov          rdi, rax
00007FF670B116BC  mov          rax, qword ptr [rsp+78h]
00007FF670B116C1  mov          qword ptr [rdi+10h], rax
00007FF670B116C5  movups       xmm0, xmmword ptr [rsp+68h]
00007FF670B116CA  movups       xmmword ptr [rdi], xmm0
00007FF670B116CD  mov          dword ptr [rdi+18h], 0h
00007FF670B116D4  mov          qword ptr [rdi+20h], r14
00007FF670B116D8  movaps       xmm0, xmmword ptr [__xmm@00000000000012000000000000001200 (00007FF670B2C560h)]
00007FF670B116DF  movups       xmmword ptr [rdi+28h], xmm0
00007FF670B116E3  lea          rax, [00007FF670B2FAB0h]
00007FF670B116EA  mov          qword ptr [rsp+A0h], rax
00007FF670B116F2  mov          qword ptr [rsp+A8h], 6h