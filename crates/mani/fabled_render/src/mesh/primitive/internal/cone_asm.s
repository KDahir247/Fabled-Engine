# --------------- Cone Dissassembly -------------------
00007FF6A4361357  mov          rcx, qword ptr [00007FF6A43831C8h]
00007FF6A436135E  test         rcx, rcx
00007FF6A4361361  jne          static void Fabled_Engine::main+BBh (00007FF6A436137Bh)
00007FF6A4361363  call         GetProcessHeap (00007FF6A4378FD2h)
00007FF6A4361368  test         rax, rax
00007FF6A436136B  je           static void Fabled_Engine::main+806h (00007FF6A4361AC6h)
00007FF6A4361371  mov          rcx, rax
00007FF6A4361374  mov          qword ptr [00007FF6A43831C8h], rax
00007FF6A436137B  mov          r8d, 300h
00007FF6A4361381  xor          edx, edx
00007FF6A4361383  call         HeapAlloc (00007FF6A4378FD8h)
00007FF6A4361388  test         rax, rax
00007FF6A436138B  je           static void Fabled_Engine::main+806h (00007FF6A4361AC6h)
00007FF6A4361391  mov          qword ptr [rsp+28h], rax
00007FF6A4361396  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000060 (00007FF6A437C530h)]
00007FF6A436139D  movups       xmmword ptr [rsp+30h], xmm0
00007FF6A43613A2  mov          rcx, qword ptr [00007FF6A43831C8h]
00007FF6A43613A9  test         rcx, rcx
00007FF6A43613AC  jne          static void Fabled_Engine::main+106h (00007FF6A43613C6h)
00007FF6A43613AE  call         GetProcessHeap (00007FF6A4378FD2h)
00007FF6A43613B3  test         rax, rax
00007FF6A43613B6  je           static void Fabled_Engine::main+817h (00007FF6A4361AD7h)
00007FF6A43613BC  mov          rcx, rax
00007FF6A43613BF  mov          qword ptr [00007FF6A43831C8h], rax
00007FF6A43613C6  mov          r8d, 480h
00007FF6A43613CC  xor          edx, edx
00007FF6A43613CE  call         HeapAlloc (00007FF6A4378FD8h)
00007FF6A43613D3  test         rax, rax
00007FF6A43613D6  je           static void Fabled_Engine::main+817h (00007FF6A4361AD7h)
00007FF6A43613DC  mov          qword ptr [rsp+40h], rax
00007FF6A43613E1  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000012 (00007FF6A437C540h)]
00007FF6A43613E8  movups       xmmword ptr [rsp+48h], xmm0
00007FF6A43613ED  mov          rcx, 4000000000000000h
00007FF6A43613F7  mov          qword ptr [rax], rcx
00007FF6A43613FA  mov          dword ptr [rax+8h], 0h
00007FF6A4361401  movaps       xmm0, xmmword ptr [__xmm@3f800000000000003f80000000000000 (00007FF6A437C550h)]
00007FF6A4361408  movups       xmmword ptr [rax+Ch], xmm0
00007FF6A436140C  movaps       xmm0, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF6A437C560h)]
00007FF6A4361413  movups       xmmword ptr [rax+1Ch], xmm0
00007FF6A4361417  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000000000003f800000 (00007FF6A437C570h)]
00007FF6A436141E  movups       xmmword ptr [rax+2Ch], xmm0
00007FF6A4361422  mov          dword ptr [rax+3Ch], 3F800000h
00007FF6A4361429  mov          qword ptr [rsp+50h], 1h
00007FF6A4361432  mov          rbx, qword ptr [rsp+40h]
00007FF6A4361437  xorps        xmm8, xmm8
00007FF6A436143B  movaps       xmmword ptr [rbx+40h], xmm8
00007FF6A4361440  movaps       xmm0, xmmword ptr [__xmm@00000000bf80000000000000bf800000 (00007FF6A437C580h)]
00007FF6A4361447  movaps       xmmword ptr [rbx+50h], xmm0
00007FF6A436144B  movaps       xmm0, xmmword ptr [__xmm@3f8000000000000000000000bf800000 (00007FF6A437C590h)]
00007FF6A4361452  movaps       xmmword ptr [rbx+60h], xmm0
00007FF6A4361456  movaps       xmm0, xmmword ptr [__xmm@3f800000bf8000000000000000000000 (00007FF6A437C5A0h)]
00007FF6A436145D  movaps       xmmword ptr [rbx+70h], xmm0
00007FF6A4361461  mov          qword ptr [rsp+50h], 2h
00007FF6A436146A  mov          edi, 2h
00007FF6A436146F  xor          ebp, ebp
00007FF6A4361471  movss        xmm10, dword ptr [__real@3f800000 (00007FF6A437C5B4h)]
00007FF6A436147A  movaps       xmm11, xmmword ptr [__xmm@3f8000003f8000000000000000000000 (00007FF6A437C5C0h)]
00007FF6A4361482  movaps       xmm12, xmmword ptr [__xmm@40a0000040a0000040a0000040a00000 (00007FF6A437C5D0h)]
00007FF6A436148A  movaps       xmm13, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF6A437C5E0h)]
00007FF6A4361492  movaps       xmm14, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF6A437C5F0h)]
00007FF6A436149A  movss        xmm15, dword ptr [__real@3d800000 (00007FF6A437C600h)]
00007FF6A43614A3  lea          r14, [rsp+40h]
00007FF6A43614A8  nop          dword ptr [rax+rax*1], eax
00007FF6A43614B0  test         rbp, rbp
00007FF6A43614B3  js           static void Fabled_Engine::main+200h (00007FF6A43614C0h)
00007FF6A43614B5  xorps        xmm9, xmm9
00007FF6A43614B9  cvtsi2ss     xmm9, rbp
00007FF6A43614BE  jmp          static void Fabled_Engine::main+21Ch (00007FF6A43614DCh)
00007FF6A43614C0  mov          rax, rbp
00007FF6A43614C3  shr          rax, 1h
00007FF6A43614C6  mov          ecx, ebp
00007FF6A43614C8  and          ecx, 1h
00007FF6A43614CB  or           rcx, rax
00007FF6A43614CE  xorps        xmm9, xmm9
00007FF6A43614D2  cvtsi2ss     xmm9, rcx
00007FF6A43614D7  addss        xmm9, xmm9
00007FF6A43614DC  movaps       xmm7, xmm9
00007FF6A43614E0  mulss        xmm7, dword ptr [__real@3ec90fdb (00007FF6A437C5B0h)]
00007FF6A43614E8  movaps       xmm0, xmm7
00007FF6A43614EB  call         sinf (00007FF6A437A0F9h)
00007FF6A43614F0  movaps       xmm6, xmm0
00007FF6A43614F3  movaps       xmm0, xmm7
00007FF6A43614F6  call         cosf (00007FF6A437A0FFh)
00007FF6A43614FB  cmp          rbp, 10h
00007FF6A43614FF  shufps       xmm0, xmm0, 0h
00007FF6A4361503  mulps        xmm0, xmm10
00007FF6A4361507  shufps       xmm6, xmm6, 0h
00007FF6A436150B  mulps        xmm6, xmm11
00007FF6A436150F  addps        xmm6, xmm0
00007FF6A4361512  mulps        xmm6, xmm12
00007FF6A4361516  addps        xmm6, xmm8
00007FF6A436151A  movaps       xmm0, xmm13
00007FF6A436151E  subps        xmm0, xmm6
00007FF6A4361521  movaps       xmm1, xmm6
00007FF6A4361524  mulps        xmm1, xmm6
00007FF6A4361527  movaps       xmm2, xmm1
00007FF6A436152A  shufps       xmm2, xmm1, 55h
00007FF6A436152E  addss        xmm2, xmm1
00007FF6A4361532  movhlps      xmm1, xmm1
00007FF6A4361535  addss        xmm1, xmm2
00007FF6A4361539  shufps       xmm1, xmm1, 0h
00007FF6A436153D  sqrtps       xmm1, xmm1
00007FF6A4361540  movaps       xmm2, xmm6
00007FF6A4361543  divps        xmm2, xmm1
00007FF6A4361546  movaps       xmm1, xmm2
00007FF6A4361549  unpckhpd     xmm1, xmm2
00007FF6A436154D  xorps        xmm1, xmm14
00007FF6A4361551  xorps        xmm3, xmm3
00007FF6A4361554  movss        xmm3, xmm1
00007FF6A4361558  shufps       xmm3, xmm2, 4h
00007FF6A436155C  movaps       xmm1, xmm0
00007FF6A436155F  shufps       xmm1, xmm0, D2h
00007FF6A4361563  mulps        xmm1, xmm3
00007FF6A4361566  shufps       xmm3, xmm3, D2h
00007FF6A436156A  mulps        xmm3, xmm0
00007FF6A436156D  subps        xmm1, xmm3
00007FF6A4361570  movaps       xmm7, xmm1
00007FF6A4361573  shufps       xmm7, xmm1, D2h
00007FF6A4361577  mulps        xmm1, xmm1
00007FF6A436157A  movaps       xmm0, xmm1
00007FF6A436157D  unpckhpd     xmm0, xmm1
00007FF6A4361581  addss        xmm0, xmm1
00007FF6A4361585  shufps       xmm1, xmm1, 55h
00007FF6A4361589  addss        xmm1, xmm0
00007FF6A436158D  shufps       xmm1, xmm1, 0h
00007FF6A4361591  sqrtps       xmm0, xmm1
00007FF6A4361594  divps        xmm7, xmm0
00007FF6A4361597  mulss        xmm9, xmm15
00007FF6A436159C  movaps       xmmword ptr [rsp+60h], xmm8
00007FF6A43615A2  movaps       xmmword ptr [rsp+90h], xmm8
00007FF6A43615AB  mov          rsi, rbp
00007FF6A43615AE  adc          rsi, 0h
00007FF6A43615B2  cmp          rdi, qword ptr [rsp+48h]
00007FF6A43615B7  je           static void Fabled_Engine::main+389h (00007FF6A4361649h)
00007FF6A43615BD  movaps       xmm0, xmm6
00007FF6A43615C0  shufps       xmm0, xmm6, 55h
00007FF6A43615C4  movaps       xmm1, xmm7
00007FF6A43615C7  shufps       xmm1, xmm7, 55h
00007FF6A43615CB  movaps       xmm2, xmm6
00007FF6A43615CE  unpckhpd     xmm2, xmm6
00007FF6A43615D2  movaps       xmm3, xmm7
00007FF6A43615D5  unpckhpd     xmm3, xmm7
00007FF6A43615D9  shl          rdi, 6h
00007FF6A43615DD  movss        dword ptr [rbx+rdi*1], xmm6
00007FF6A43615E2  movss        dword ptr [rbx+rdi*1+4h], xmm0
00007FF6A43615E8  movss        dword ptr [rbx+rdi*1+8h], xmm2
00007FF6A43615EE  movss        dword ptr [rbx+rdi*1+Ch], xmm9
00007FF6A43615F5  mov          dword ptr [rbx+rdi*1+10h], 0h
00007FF6A43615FD  movss        dword ptr [rbx+rdi*1+14h], xmm7
00007FF6A4361603  movss        dword ptr [rbx+rdi*1+18h], xmm1
00007FF6A4361609  movss        dword ptr [rbx+rdi*1+1Ch], xmm3
00007FF6A436160F  movaps       xmm0, xmmword ptr [rsp+60h]
00007FF6A4361614  movaps       xmmword ptr [rbx+rdi*1+20h], xmm0
00007FF6A4361619  movaps       xmm0, xmmword ptr [rsp+90h]
00007FF6A4361621  movaps       xmmword ptr [rbx+rdi*1+30h], xmm0
00007FF6A4361626  mov          rdi, qword ptr [rsp+50h]
00007FF6A436162B  add          rdi, 1h
00007FF6A436162F  mov          qword ptr [rsp+50h], rdi
00007FF6A4361634  cmp          rbp, Fh
00007FF6A4361638  ja           static void Fabled_Engine::main+3A3h (00007FF6A4361663h)
00007FF6A436163A  mov          rbp, rsi
00007FF6A436163D  cmp          rsi, 11h
00007FF6A4361641  jb           static void Fabled_Engine::main+1F0h (00007FF6A43614B0h)
00007FF6A4361647  jmp          static void Fabled_Engine::main+3A3h (00007FF6A4361663h)
00007FF6A4361649  mov          rcx, r14
00007FF6A436164C  mov          rdx, rdi
00007FF6A436164F  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF6A437AB10h)
00007FF6A4361654  mov          rbx, qword ptr [rsp+40h]
00007FF6A4361659  mov          rdi, qword ptr [rsp+50h]
00007FF6A436165E  jmp          static void Fabled_Engine::main+2FDh (00007FF6A43615BDh)
00007FF6A4361663  mov          esi, 3h
00007FF6A4361668  mov          rdx, qword ptr [rsp+38h]
00007FF6A436166D  lea          r12, [rsp+28h]
00007FF6A4361672  jmp          static void Fabled_Engine::main+3DBh (00007FF6A436169Bh)
00007FF6A4361674  nop          word ptr cs:[rax+rax*1], ax
00007FF6A436167E  nop
00007FF6A4361680  mov          qword ptr [rax+rdx*8], rsi
00007FF6A4361684  add          rdx, 1h
00007FF6A4361688  mov          qword ptr [rsp+38h], rdx
00007FF6A436168D  add          rsi, 1h
00007FF6A4361691  cmp          rsi, 13h
00007FF6A4361695  je           static void Fabled_Engine::main+50Dh (00007FF6A43617CDh)
00007FF6A436169B  cmp          rdx, qword ptr [rsp+30h]
00007FF6A43616A0  je           static void Fabled_Engine::main+4A4h (00007FF6A4361764h)
00007FF6A43616A6  mov          rax, qword ptr [rsp+28h]
00007FF6A43616AB  mov          qword ptr [rax+rdx*8], 0h
00007FF6A43616B3  mov          rdx, qword ptr [rsp+38h]
00007FF6A43616B8  add          rdx, 1h
00007FF6A43616BC  mov          qword ptr [rsp+38h], rdx
00007FF6A43616C1  cmp          rdx, qword ptr [rsp+30h]
00007FF6A43616C6  je           static void Fabled_Engine::main+4B6h (00007FF6A4361776h)
00007FF6A43616CC  lea          rbx, [rsi-1h]
00007FF6A43616D0  mov          qword ptr [rax+rdx*8], rsi
00007FF6A43616D4  mov          rdx, qword ptr [rsp+38h]
00007FF6A43616D9  add          rdx, 1h
00007FF6A43616DD  mov          qword ptr [rsp+38h], rdx
00007FF6A43616E2  cmp          rdx, qword ptr [rsp+30h]
00007FF6A43616E7  je           static void Fabled_Engine::main+4CDh (00007FF6A436178Dh)
00007FF6A43616ED  mov          qword ptr [rax+rdx*8], rbx
00007FF6A43616F1  mov          rdx, qword ptr [rsp+38h]
00007FF6A43616F6  add          rdx, 1h
00007FF6A43616FA  mov          qword ptr [rsp+38h], rdx
00007FF6A43616FF  cmp          rdx, qword ptr [rsp+30h]
00007FF6A4361704  je           static void Fabled_Engine::main+4E4h (00007FF6A43617A4h)
00007FF6A436170A  mov          rax, qword ptr [rsp+28h]
00007FF6A436170F  mov          qword ptr [rax+rdx*8], 1h
00007FF6A4361717  mov          rdx, qword ptr [rsp+38h]
00007FF6A436171C  add          rdx, 1h
00007FF6A4361720  mov          qword ptr [rsp+38h], rdx
00007FF6A4361725  cmp          rdx, qword ptr [rsp+30h]
00007FF6A436172A  je           static void Fabled_Engine::main+4F6h (00007FF6A43617B6h)
00007FF6A4361730  mov          qword ptr [rax+rdx*8], rbx
00007FF6A4361734  mov          rdx, qword ptr [rsp+38h]
00007FF6A4361739  add          rdx, 1h
00007FF6A436173D  mov          qword ptr [rsp+38h], rdx
00007FF6A4361742  cmp          rdx, qword ptr [rsp+30h]
00007FF6A4361747  jne          static void Fabled_Engine::main+3C0h (00007FF6A4361680h)
00007FF6A436174D  mov          rcx, r12
00007FF6A4361750  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A437ABD0h)
00007FF6A4361755  mov          rax, qword ptr [rsp+28h]
00007FF6A436175A  mov          rdx, qword ptr [rsp+38h]
00007FF6A436175F  jmp          static void Fabled_Engine::main+3C0h (00007FF6A4361680h)
00007FF6A4361764  mov          rcx, r12
00007FF6A4361767  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A437ABD0h)
00007FF6A436176C  mov          rdx, qword ptr [rsp+38h]
00007FF6A4361771  jmp          static void Fabled_Engine::main+3E6h (00007FF6A43616A6h)
00007FF6A4361776  mov          rcx, r12
00007FF6A4361779  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A437ABD0h)
00007FF6A436177E  mov          rax, qword ptr [rsp+28h]
00007FF6A4361783  mov          rdx, qword ptr [rsp+38h]
00007FF6A4361788  jmp          static void Fabled_Engine::main+40Ch (00007FF6A43616CCh)
00007FF6A436178D  mov          rcx, r12
00007FF6A4361790  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A437ABD0h)
00007FF6A4361795  mov          rax, qword ptr [rsp+28h]
00007FF6A436179A  mov          rdx, qword ptr [rsp+38h]
00007FF6A436179F  jmp          static void Fabled_Engine::main+42Dh (00007FF6A43616EDh)
00007FF6A43617A4  mov          rcx, r12
00007FF6A43617A7  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A437ABD0h)
00007FF6A43617AC  mov          rdx, qword ptr [rsp+38h]
00007FF6A43617B1  jmp          static void Fabled_Engine::main+44Ah (00007FF6A436170Ah)
00007FF6A43617B6  mov          rcx, r12
00007FF6A43617B9  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF6A437ABD0h)
00007FF6A43617BE  mov          rax, qword ptr [rsp+28h]
00007FF6A43617C3  mov          rdx, qword ptr [rsp+38h]
00007FF6A43617C8  jmp          static void Fabled_Engine::main+470h (00007FF6A4361730h)
00007FF6A43617CD  mov          rax, qword ptr [rsp+38h]
00007FF6A43617D2  mov          qword ptr [rsp+74h], rax
00007FF6A43617D7  movups       xmm0, xmmword ptr [rsp+28h]
00007FF6A43617DC  movups       xmmword ptr [rsp+64h], xmm0
00007FF6A43617E1  mov          rcx, qword ptr [00007FF6A43831C8h]
00007FF6A43617E8  test         rcx, rcx
00007FF6A43617EB  jne          static void Fabled_Engine::main+545h (00007FF6A4361805h)
00007FF6A43617ED  call         GetProcessHeap (00007FF6A4378FD2h)
00007FF6A43617F2  test         rax, rax
00007FF6A43617F5  je           static void Fabled_Engine::main+81Eh (00007FF6A4361ADEh)
00007FF6A43617FB  mov          rcx, rax
00007FF6A43617FE  mov          qword ptr [00007FF6A43831C8h], rax
00007FF6A4361805  mov          r8d, 40h
00007FF6A436180B  xor          edx, edx
00007FF6A436180D  call         HeapAlloc (00007FF6A4378FD8h)
00007FF6A4361812  test         rax, rax
00007FF6A4361815  je           static void Fabled_Engine::main+81Eh (00007FF6A4361ADEh)
00007FF6A436181B  mov          rbx, rax
00007FF6A436181E  mov          rax, qword ptr [rsp+50h]
00007FF6A4361823  mov          qword ptr [rbx+10h], rax
00007FF6A4361827  movups       xmm0, xmmword ptr [rsp+40h]
00007FF6A436182C  movups       xmmword ptr [rbx], xmm0
00007FF6A436182F  movups       xmm0, xmmword ptr [rsp+60h]
00007FF6A4361834  movups       xmm1, xmmword ptr [rsp+6Ch]
00007FF6A4361839  movups       xmmword ptr [rbx+1Ch], xmm0
00007FF6A436183D  movups       xmmword ptr [rbx+28h], xmm1
00007FF6A4361841  mov          dword ptr [rbx+18h], 0h
00007FF6A4361848  lea          rax, [00007FF6A437FB40h]
00007FF6A436184F  mov          qword ptr [rsp+90h], rax
00007FF6A4361857  mov          qword ptr [rsp+98h], 6h
