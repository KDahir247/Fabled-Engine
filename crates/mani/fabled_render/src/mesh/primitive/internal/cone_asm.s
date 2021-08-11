# --------------- Cone Dissassembly -------------------
00007FF7FA09109B  mov          rcx, qword ptr [00007FF7FA0B31C8h]
00007FF7FA0910A2  test         rcx, rcx
00007FF7FA0910A5  jne          static void Fabled_Engine::main+BFh (00007FF7FA0910BFh)
00007FF7FA0910A7  call         GetProcessHeap (00007FF7FA0A905Ch)
00007FF7FA0910AC  test         rax, rax
00007FF7FA0910AF  je           static void Fabled_Engine::main+89Ch (00007FF7FA09189Ch)
00007FF7FA0910B5  mov          rcx, rax
00007FF7FA0910B8  mov          qword ptr [00007FF7FA0B31C8h], rax
00007FF7FA0910BF  mov          r8d, 90h
00007FF7FA0910C5  xor          edx, edx
00007FF7FA0910C7  call         HeapAlloc (00007FF7FA0A9062h)
00007FF7FA0910CC  test         rax, rax
00007FF7FA0910CF  je           static void Fabled_Engine::main+89Ch (00007FF7FA09189Ch)
00007FF7FA0910D5  mov          qword ptr [rsp+28h], rax
00007FF7FA0910DA  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000012 (00007FF7FA0AC520h)]
00007FF7FA0910E1  movups       xmmword ptr [rsp+30h], xmm0
00007FF7FA0910E6  mov          rcx, qword ptr [00007FF7FA0B31C8h]
00007FF7FA0910ED  test         rcx, rcx
00007FF7FA0910F0  jne          static void Fabled_Engine::main+10Ah (00007FF7FA09110Ah)
00007FF7FA0910F2  call         GetProcessHeap (00007FF7FA0A905Ch)
00007FF7FA0910F7  test         rax, rax
00007FF7FA0910FA  je           static void Fabled_Engine::main+8ADh (00007FF7FA0918ADh)
00007FF7FA091100  mov          rcx, rax
00007FF7FA091103  mov          qword ptr [00007FF7FA0B31C8h], rax
00007FF7FA09110A  mov          r8d, 140h
00007FF7FA091110  xor          edx, edx
00007FF7FA091112  call         HeapAlloc (00007FF7FA0A9062h)
00007FF7FA091117  test         rax, rax
00007FF7FA09111A  je           static void Fabled_Engine::main+8ADh (00007FF7FA0918ADh)
00007FF7FA091120  mov          qword ptr [rsp+40h], rax
00007FF7FA091125  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000005 (00007FF7FA0AC530h)]
00007FF7FA09112C  movups       xmmword ptr [rsp+48h], xmm0
00007FF7FA091131  mov          rcx, 4000000000000000h
00007FF7FA09113B  mov          qword ptr [rax], rcx
00007FF7FA09113E  mov          dword ptr [rax+8h], 0h
00007FF7FA091145  movaps       xmm0, xmmword ptr [__xmm@3f800000000000003f80000000000000 (00007FF7FA0AC540h)]
00007FF7FA09114C  movups       xmmword ptr [rax+Ch], xmm0
00007FF7FA091150  movaps       xmm0, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF7FA0AC550h)]
00007FF7FA091157  movups       xmmword ptr [rax+1Ch], xmm0
00007FF7FA09115B  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000000000003f800000 (00007FF7FA0AC560h)]
00007FF7FA091162  movups       xmmword ptr [rax+2Ch], xmm0
00007FF7FA091166  mov          dword ptr [rax+3Ch], 3F800000h
00007FF7FA09116D  mov          qword ptr [rsp+50h], 1h
00007FF7FA091176  mov          rax, qword ptr [rsp+40h]
00007FF7FA09117B  xorps        xmm0, xmm0
00007FF7FA09117E  movaps       xmmword ptr [rax+40h], xmm0
00007FF7FA091182  movaps       xmm0, xmmword ptr [__xmm@00000000bf80000000000000bf800000 (00007FF7FA0AC570h)]
00007FF7FA091189  movaps       xmmword ptr [rax+50h], xmm0
00007FF7FA09118D  movaps       xmm0, xmmword ptr [__xmm@3f8000000000000000000000bf800000 (00007FF7FA0AC580h)]
00007FF7FA091194  movaps       xmmword ptr [rax+60h], xmm0
00007FF7FA091198  movaps       xmm0, xmmword ptr [__xmm@3f800000bf8000000000000000000000 (00007FF7FA0AC590h)]
00007FF7FA09119F  movaps       xmmword ptr [rax+70h], xmm0
00007FF7FA0911A3  mov          qword ptr [rsp+50h], 2h
00007FF7FA0911AC  xor          ecx, ecx
00007FF7FA0911AE  movss        xmm9, dword ptr [__real@40060a92 (00007FF7FA0AC5A0h)]
00007FF7FA0911B7  movss        xmm10, dword ptr [__real@3f800000 (00007FF7FA0AC5A4h)]
00007FF7FA0911C0  movaps       xmm11, xmmword ptr [__xmm@3f8000003f8000000000000000000000 (00007FF7FA0AC5B0h)]
00007FF7FA0911C8  movaps       xmm12, xmmword ptr [__xmm@40a0000040a0000040a0000040a00000 (00007FF7FA0AC5C0h)]
00007FF7FA0911D0  movaps       xmm13, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF7FA0AC5D0h)]
00007FF7FA0911D8  movaps       xmm14, xmmword ptr [__xmm@00000000bf8000008000000080000000 (00007FF7FA0AC5E0h)]
00007FF7FA0911E0  movss        xmm15, dword ptr [__real@40400000 (00007FF7FA0AC5F0h)]
00007FF7FA0911E9  xor          eax, eax
00007FF7FA0911EB  nop          dword ptr [rax+rax*1], eax
00007FF7FA0911F0  cmp          rcx, 3h
00007FF7FA0911F4  adc          rax, 0h
00007FF7FA0911F8  test         rcx, rcx
00007FF7FA0911FB  mov          qword ptr [rsp+98h], rcx
00007FF7FA091203  mov          qword ptr [rsp+90h], rax
00007FF7FA09120B  js           static void Fabled_Engine::main+220h (00007FF7FA091220h)
00007FF7FA09120D  xorps        xmm8, xmm8
00007FF7FA091211  cvtsi2ss     xmm8, rcx
00007FF7FA091216  jmp          static void Fabled_Engine::main+236h (00007FF7FA091236h)
00007FF7FA091218  nop          dword ptr [rax+rax*1], eax
00007FF7FA091220  mov          rax, rcx
00007FF7FA091223  shr          rax, 1h
00007FF7FA091226  and          ecx, 1h
00007FF7FA091229  or           rcx, rax
00007FF7FA09122C  cvtsi2ss     xmm8, rcx
00007FF7FA091231  addss        xmm8, xmm8
00007FF7FA091236  movaps       xmm7, xmm8
00007FF7FA09123A  mulss        xmm7, xmm9
00007FF7FA09123F  movaps       xmm0, xmm7
00007FF7FA091242  call         sinf (00007FF7FA0AA189h)
00007FF7FA091247  movaps       xmm6, xmm0
00007FF7FA09124A  movaps       xmm0, xmm7
00007FF7FA09124D  call         cosf (00007FF7FA0AA18Fh)
00007FF7FA091252  shufps       xmm0, xmm0, 0h
00007FF7FA091256  mulps        xmm0, xmm10
00007FF7FA09125A  shufps       xmm6, xmm6, 0h
00007FF7FA09125E  mulps        xmm6, xmm11
00007FF7FA091262  addps        xmm6, xmm0
00007FF7FA091265  mulps        xmm6, xmm12
00007FF7FA091269  addps        xmm6, xmmword ptr [__xmm@00000000000000000000000000000000 (00007FF7FA0AC600h)]
00007FF7FA091270  movaps       xmm1, xmm13
00007FF7FA091274  subps        xmm1, xmm6
00007FF7FA091277  movaps       xmm0, xmm6
00007FF7FA09127A  mulps        xmm0, xmm6
00007FF7FA09127D  movaps       xmm2, xmm0
00007FF7FA091280  shufps       xmm2, xmm0, 55h
00007FF7FA091284  addss        xmm2, xmm0
00007FF7FA091288  movhlps      xmm0, xmm0
00007FF7FA09128B  addss        xmm0, xmm2
00007FF7FA09128F  shufps       xmm0, xmm0, 0h
00007FF7FA091293  sqrtps       xmm2, xmm0
00007FF7FA091296  movaps       xmm0, xmm6
00007FF7FA091299  divps        xmm0, xmm2
00007FF7FA09129C  movaps       xmm2, xmm0
00007FF7FA09129F  mulps        xmm2, xmm10
00007FF7FA0912A3  shufps       xmm2, xmm2, D2h
00007FF7FA0912A7  mulps        xmm0, xmm14
00007FF7FA0912AB  addps        xmm0, xmm2
00007FF7FA0912AE  movaps       xmm2, xmm0
00007FF7FA0912B1  shufps       xmm2, xmm0, 52h
00007FF7FA0912B5  movaps       xmm3, xmm1
00007FF7FA0912B8  shufps       xmm3, xmm1, D2h
00007FF7FA0912BC  movaps       xmm4, xmm0
00007FF7FA0912BF  shufps       xmm4, xmm0, 49h
00007FF7FA0912C3  mulps        xmm3, xmm2
00007FF7FA0912C6  mulps        xmm1, xmm4
00007FF7FA0912C9  subps        xmm3, xmm1
00007FF7FA0912CC  movaps       xmm7, xmm3
00007FF7FA0912CF  shufps       xmm7, xmm3, 52h
00007FF7FA0912D3  mulps        xmm3, xmm3
00007FF7FA0912D6  movaps       xmm1, xmm3
00007FF7FA0912D9  unpckhpd     xmm1, xmm3
00007FF7FA0912DD  addss        xmm1, xmm3
00007FF7FA0912E1  shufps       xmm3, xmm3, 55h
00007FF7FA0912E5  addss        xmm3, xmm1
00007FF7FA0912E9  shufps       xmm3, xmm3, 0h
00007FF7FA0912ED  sqrtps       xmm1, xmm3
00007FF7FA0912F0  divps        xmm7, xmm1
00007FF7FA0912F3  movaps       xmm1, xmm7
00007FF7FA0912F6  shufps       xmm1, xmm7, D2h
00007FF7FA0912FA  mulps        xmm1, xmm2
00007FF7FA0912FD  mulps        xmm4, xmm7
00007FF7FA091300  subps        xmm1, xmm4
00007FF7FA091303  movd         r12d, xmm6
00007FF7FA091308  movaps       xmm2, xmm6
00007FF7FA09130B  shufps       xmm2, xmm6, 55h
00007FF7FA09130F  movd         eax, xmm2
00007FF7FA091313  shl          rax, 20h
00007FF7FA091317  or           r12, rax
00007FF7FA09131A  divss        xmm8, xmm15
00007FF7FA09131F  movd         r13d, xmm7
00007FF7FA091324  movaps       xmm2, xmm7
00007FF7FA091327  shufps       xmm2, xmm7, 55h
00007FF7FA09132B  movd         eax, xmm2
00007FF7FA09132F  shl          rax, 20h
00007FF7FA091333  or           r13, rax
00007FF7FA091336  movaps       xmm2, xmm0
00007FF7FA091339  unpckhpd     xmm2, xmm0
00007FF7FA09133D  movd         ebp, xmm2
00007FF7FA091341  movd         eax, xmm0
00007FF7FA091345  shufps       xmm0, xmm0, 55h
00007FF7FA091349  movd         edi, xmm0
00007FF7FA09134D  shl          rax, 20h
00007FF7FA091351  or           rbp, rax
00007FF7FA091354  mov          rcx, 3F80000000000000h
00007FF7FA09135E  or           rdi, rcx
00007FF7FA091361  movaps       xmm0, xmm1
00007FF7FA091364  unpckhpd     xmm0, xmm1
00007FF7FA091368  movd         r14d, xmm0
00007FF7FA09136D  movd         eax, xmm1
00007FF7FA091371  shufps       xmm1, xmm1, 55h
00007FF7FA091375  movd         r15d, xmm1
00007FF7FA09137A  shl          rax, 20h
00007FF7FA09137E  or           r14, rax
00007FF7FA091381  or           r15, rcx
00007FF7FA091384  mov          rdx, qword ptr [rsp+50h]
00007FF7FA091389  cmp          rdx, qword ptr [rsp+48h]
00007FF7FA09138E  je           static void Fabled_Engine::main+40Bh (00007FF7FA09140Bh)
00007FF7FA091390  punpckhqdq   xmm6, xmm6
00007FF7FA091394  movd         eax, xmm6
00007FF7FA091398  punpckhqdq   xmm7, xmm7
00007FF7FA09139C  mov          rcx, rdx
00007FF7FA09139F  shl          rcx, 6h
00007FF7FA0913A3  mov          rbx, qword ptr [rsp+40h]
00007FF7FA0913A8  mov          qword ptr [rbx+rcx*1], r12
00007FF7FA0913AC  movd         esi, xmm7
00007FF7FA0913B0  add          rdx, 1h
00007FF7FA0913B4  mov          dword ptr [rbx+rcx*1+8h], eax
00007FF7FA0913B8  movss        dword ptr [rbx+rcx*1+Ch], xmm8
00007FF7FA0913BF  mov          dword ptr [rbx+rcx*1+10h], 0h
00007FF7FA0913C7  mov          qword ptr [rbx+rcx*1+14h], r13
00007FF7FA0913CC  mov          dword ptr [rbx+rcx*1+1Ch], esi
00007FF7FA0913D0  mov          qword ptr [rbx+rcx*1+28h], rdi
00007FF7FA0913D5  mov          qword ptr [rbx+rcx*1+20h], rbp
00007FF7FA0913DA  mov          qword ptr [rbx+rcx*1+38h], r15
00007FF7FA0913DF  mov          qword ptr [rbx+rcx*1+30h], r14
00007FF7FA0913E4  mov          qword ptr [rsp+50h], rdx
00007FF7FA0913E9  cmp          qword ptr [rsp+98h], 2h
00007FF7FA0913F2  mov          rax, qword ptr [rsp+90h]
00007FF7FA0913FA  ja           static void Fabled_Engine::main+41Fh (00007FF7FA09141Fh)
00007FF7FA0913FC  mov          rcx, rax
00007FF7FA0913FF  cmp          rax, 4h
00007FF7FA091403  jb           static void Fabled_Engine::main+1F0h (00007FF7FA0911F0h)
00007FF7FA091409  jmp          static void Fabled_Engine::main+41Fh (00007FF7FA09141Fh)
00007FF7FA09140B  lea          rcx, [rsp+40h]
00007FF7FA091410  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF7FA0AABA0h)
00007FF7FA091415  mov          rdx, qword ptr [rsp+50h]
00007FF7FA09141A  jmp          static void Fabled_Engine::main+390h (00007FF7FA091390h)
00007FF7FA09141F  mov          esi, 3h
00007FF7FA091424  mov          rdx, qword ptr [rsp+38h]
00007FF7FA091429  lea          r12, [rsp+28h]
00007FF7FA09142E  jmp          static void Fabled_Engine::main+44Bh (00007FF7FA09144Bh)
00007FF7FA091430  mov          qword ptr [rax+rdx*8], rsi
00007FF7FA091434  add          rdx, 1h
00007FF7FA091438  mov          qword ptr [rsp+38h], rdx
00007FF7FA09143D  add          rsi, 1h
00007FF7FA091441  cmp          rsi, 6h
00007FF7FA091445  je           static void Fabled_Engine::main+57Dh (00007FF7FA09157Dh)
00007FF7FA09144B  cmp          rdx, qword ptr [rsp+30h]
00007FF7FA091450  je           static void Fabled_Engine::main+514h (00007FF7FA091514h)
00007FF7FA091456  mov          rax, qword ptr [rsp+28h]
00007FF7FA09145B  mov          qword ptr [rax+rdx*8], 0h
00007FF7FA091463  mov          rdx, qword ptr [rsp+38h]
00007FF7FA091468  add          rdx, 1h
00007FF7FA09146C  mov          qword ptr [rsp+38h], rdx
00007FF7FA091471  cmp          rdx, qword ptr [rsp+30h]
00007FF7FA091476  je           static void Fabled_Engine::main+526h (00007FF7FA091526h)
00007FF7FA09147C  lea          rbx, [rsi-1h]
00007FF7FA091480  mov          qword ptr [rax+rdx*8], rsi
00007FF7FA091484  mov          rdx, qword ptr [rsp+38h]
00007FF7FA091489  add          rdx, 1h
00007FF7FA09148D  mov          qword ptr [rsp+38h], rdx
00007FF7FA091492  cmp          rdx, qword ptr [rsp+30h]
00007FF7FA091497  je           static void Fabled_Engine::main+53Dh (00007FF7FA09153Dh)
00007FF7FA09149D  mov          qword ptr [rax+rdx*8], rbx
00007FF7FA0914A1  mov          rdx, qword ptr [rsp+38h]
00007FF7FA0914A6  add          rdx, 1h
00007FF7FA0914AA  mov          qword ptr [rsp+38h], rdx
00007FF7FA0914AF  cmp          rdx, qword ptr [rsp+30h]
00007FF7FA0914B4  je           static void Fabled_Engine::main+554h (00007FF7FA091554h)
00007FF7FA0914BA  mov          rax, qword ptr [rsp+28h]
00007FF7FA0914BF  mov          qword ptr [rax+rdx*8], 1h
00007FF7FA0914C7  mov          rdx, qword ptr [rsp+38h]
00007FF7FA0914CC  add          rdx, 1h
00007FF7FA0914D0  mov          qword ptr [rsp+38h], rdx
00007FF7FA0914D5  cmp          rdx, qword ptr [rsp+30h]
00007FF7FA0914DA  je           static void Fabled_Engine::main+566h (00007FF7FA091566h)
00007FF7FA0914E0  mov          qword ptr [rax+rdx*8], rbx
00007FF7FA0914E4  mov          rdx, qword ptr [rsp+38h]
00007FF7FA0914E9  add          rdx, 1h
00007FF7FA0914ED  mov          qword ptr [rsp+38h], rdx
00007FF7FA0914F2  cmp          rdx, qword ptr [rsp+30h]
00007FF7FA0914F7  jne          static void Fabled_Engine::main+430h (00007FF7FA091430h)
00007FF7FA0914FD  mov          rcx, r12
00007FF7FA091500  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7FA0AAC60h)
00007FF7FA091505  mov          rax, qword ptr [rsp+28h]
00007FF7FA09150A  mov          rdx, qword ptr [rsp+38h]
00007FF7FA09150F  jmp          static void Fabled_Engine::main+430h (00007FF7FA091430h)
00007FF7FA091514  mov          rcx, r12
00007FF7FA091517  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7FA0AAC60h)
00007FF7FA09151C  mov          rdx, qword ptr [rsp+38h]
00007FF7FA091521  jmp          static void Fabled_Engine::main+456h (00007FF7FA091456h)
00007FF7FA091526  mov          rcx, r12
00007FF7FA091529  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7FA0AAC60h)
00007FF7FA09152E  mov          rax, qword ptr [rsp+28h]
00007FF7FA091533  mov          rdx, qword ptr [rsp+38h]
00007FF7FA091538  jmp          static void Fabled_Engine::main+47Ch (00007FF7FA09147Ch)
00007FF7FA09153D  mov          rcx, r12
00007FF7FA091540  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7FA0AAC60h)
00007FF7FA091545  mov          rax, qword ptr [rsp+28h]
00007FF7FA09154A  mov          rdx, qword ptr [rsp+38h]
00007FF7FA09154F  jmp          static void Fabled_Engine::main+49Dh (00007FF7FA09149Dh)
00007FF7FA091554  mov          rcx, r12
00007FF7FA091557  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7FA0AAC60h)
00007FF7FA09155C  mov          rdx, qword ptr [rsp+38h]
00007FF7FA091561  jmp          static void Fabled_Engine::main+4BAh (00007FF7FA0914BAh)
00007FF7FA091566  mov          rcx, r12
00007FF7FA091569  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF7FA0AAC60h)
00007FF7FA09156E  mov          rax, qword ptr [rsp+28h]
00007FF7FA091573  mov          rdx, qword ptr [rsp+38h]
00007FF7FA091578  jmp          static void Fabled_Engine::main+4E0h (00007FF7FA0914E0h)
00007FF7FA09157D  mov          rax, qword ptr [rsp+38h]
00007FF7FA091582  mov          qword ptr [rsp+74h], rax
00007FF7FA091587  movups       xmm0, xmmword ptr [rsp+28h]
00007FF7FA09158C  movups       xmmword ptr [rsp+64h], xmm0
00007FF7FA091591  mov          rcx, qword ptr [00007FF7FA0B31C8h]
00007FF7FA091598  test         rcx, rcx
00007FF7FA09159B  jne          static void Fabled_Engine::main+5B5h (00007FF7FA0915B5h)
00007FF7FA09159D  call         GetProcessHeap (00007FF7FA0A905Ch)
00007FF7FA0915A2  test         rax, rax
00007FF7FA0915A5  je           static void Fabled_Engine::main+8B4h (00007FF7FA0918B4h)
00007FF7FA0915AB  mov          rcx, rax
00007FF7FA0915AE  mov          qword ptr [00007FF7FA0B31C8h], rax
00007FF7FA0915B5  mov          r8d, 40h
00007FF7FA0915BB  xor          edx, edx
00007FF7FA0915BD  call         HeapAlloc (00007FF7FA0A9062h)
00007FF7FA0915C2  test         rax, rax
00007FF7FA0915C5  je           static void Fabled_Engine::main+8B4h (00007FF7FA0918B4h)
00007FF7FA0915CB  mov          rbx, rax
00007FF7FA0915CE  mov          rax, qword ptr [rsp+50h]
00007FF7FA0915D3  mov          qword ptr [rbx+10h], rax
00007FF7FA0915D7  movups       xmm0, xmmword ptr [rsp+40h]
00007FF7FA0915DC  movups       xmmword ptr [rbx], xmm0
00007FF7FA0915DF  movups       xmm0, xmmword ptr [rsp+60h]
00007FF7FA0915E4  movups       xmm1, xmmword ptr [rsp+6Ch]
00007FF7FA0915E9  movups       xmmword ptr [rbx+1Ch], xmm0
00007FF7FA0915ED  movups       xmmword ptr [rbx+28h], xmm1
00007FF7FA0915F1  mov          rax, rbx
00007FF7FA0915F4  add          rax, 18h
00007FF7FA0915F8  mov          dword ptr [rbx+18h], 0h